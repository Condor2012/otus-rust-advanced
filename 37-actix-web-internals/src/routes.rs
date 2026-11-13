//! Обработчики: архитектура, протоколы, идиомы.
use std::{
    cell::Cell,
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use actix_web::{
    Error, HttpRequest, HttpResponse, Responder, get, post, rt, web,
    web::{Bytes, Json, Payload},
};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{errors::ApiError, extract::ClientId};

/// Общее для всех воркеров состояние: Data<T> = Arc<T>.
pub struct Global {
    pub hits: AtomicU64,
}

/// Состояние одного воркера: Rc<Cell<_>> — !Send, и это нормально.
#[derive(Clone)]
pub struct PerWorker {
    pub hits: Rc<Cell<u64>>,
}

fn thread_name() -> String {
    std::thread::current().name().unwrap_or("?").to_owned()
}

// ---------- 1. Архитектура: воркеры и состояние ----------
#[get("/hits")]
async fn hits(req: HttpRequest, global: web::Data<Global>) -> impl Responder {
    let g = global.hits.fetch_add(1, Ordering::Relaxed) + 1;
    // app_data без Data<>: достаём !Send-тип напрямую.
    let local = req.app_data::<PerWorker>().expect("PerWorker зарегистрирован");
    local.hits.set(local.hits.get() + 1);
    HttpResponse::Ok().json(json!({
        "thread": thread_name(),
        "worker_hits": local.hits.get(),
        "global_hits": g,
    }))
}

// ---------- 2. Блокирующий код ----------
#[get("/fast")]
async fn fast() -> impl Responder {
    format!("fast from {}\n", thread_name())
}

/// ПЛОХО: std::thread::sleep блокирует поток воркера целиком.
#[get("/block-bad")]
async fn block_bad() -> impl Responder {
    std::thread::sleep(Duration::from_secs(3));
    "slept 3s on the worker thread\n"
}

/// ХОРОШО: блокирующая работа уходит в пул spawn_blocking.
#[get("/block-good")]
async fn block_good() -> Result<impl Responder, Error> {
    let who = web::block(|| {
        std::thread::sleep(Duration::from_secs(3));
        thread_name()
    })
    .await?;
    Ok(format!("slept 3s on {who}\n"))
}

// ---------- 3. Тело запроса и лимиты ----------
#[derive(Deserialize, Serialize)]
pub struct Note {
    text: String,
}

#[post("/notes")]
async fn create_note(client: ClientId, note: Json<Note>) -> impl Responder {
    HttpResponse::Created().json(json!({ "client": client.0, "len": note.text.len() }))
}

const UPLOAD_LIMIT: usize = 1024 * 1024;

/// Потоковое чтение тела: не держим всё в памяти, сами проверяем лимит.
#[post("/upload")]
async fn upload(mut body: Payload) -> Result<impl Responder, ApiError> {
    let (mut total, mut chunks) = (0usize, 0usize);
    while let Some(chunk) = body.next().await {
        let chunk = chunk.map_err(|_| ApiError::Internal)?;
        total += chunk.len();
        chunks += 1;
        if total > UPLOAD_LIMIT {
            return Err(ApiError::TooLarge(total, UPLOAD_LIMIT));
        }
    }
    Ok(HttpResponse::Ok().json(json!({ "bytes": total, "chunks": chunks })))
}

// ---------- 4. Потоковый ответ (chunked) ----------
#[get("/stream")]
async fn stream() -> impl Responder {
    let s = futures_util::stream::unfold(1u32, |n| async move {
        if n > 5 {
            return None;
        }
        rt::time::sleep(Duration::from_millis(500)).await;
        Some((Ok::<_, Error>(Bytes::from(format!("tick {n}\n"))), n + 1))
    });
    HttpResponse::Ok().content_type("text/plain").streaming(s)
}

/// Версия протокола глазами сервера.
#[get("/proto")]
async fn proto(req: HttpRequest) -> impl Responder {
    format!("{:?}\n", req.version())
}

// ---------- 5. WebSocket (actix-ws) ----------
#[get("/ws")]
async fn ws(req: HttpRequest, body: Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, msgs) = actix_ws::handle(&req, body)?;
    let mut msgs = msgs.max_frame_size(64 * 1024).aggregate_continuations();

    rt::spawn(async move {
        // interval_at: первый тик через 5 с, а не сразу.
        let start = rt::time::Instant::now() + Duration::from_secs(5);
        let mut heartbeat = rt::time::interval_at(start, Duration::from_secs(5));
        loop {
            tokio::select! {
                _ = heartbeat.tick() => {
                    if session.ping(b"hb").await.is_err() { break; }
                }
                msg = msgs.next() => {
                    use actix_ws::AggregatedMessage as M;
                    match msg {
                        Some(Ok(M::Text(t))) => {
                            if session.text(format!("echo: {t}")).await.is_err() { break; }
                        }
                        Some(Ok(M::Ping(p))) => { let _ = session.pong(&p).await; }
                        Some(Ok(M::Close(reason))) => { let _ = session.close(reason).await; break; }
                        Some(Ok(_)) => {}
                        Some(Err(e)) => { log::warn!("ws error: {e}"); break; }
                        None => break,
                    }
                }
            }
        }
    });
    Ok(res)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(hits)
        .service(fast)
        .service(block_bad)
        .service(block_good)
        .service(create_note)
        .service(upload)
        .service(stream)
        .service(proto)
        .service(ws);
}
