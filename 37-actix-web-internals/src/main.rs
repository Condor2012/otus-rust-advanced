#![forbid(unsafe_code)] // в коде приложения unsafe запрещён целиком

mod errors;
mod extract;
mod middleware;
mod routes;

use std::{cell::Cell, rc::Rc, sync::atomic::AtomicU64, time::Duration};

use actix_web::{
    App, HttpResponse, HttpServer, error,
    middleware::{Logger, from_fn},
    web,
};
use serde_json::json;

use routes::{Global, PerWorker};

/// JsonConfig: маленький лимит + свой формат ошибки.
fn json_cfg() -> web::JsonConfig {
    web::JsonConfig::default().limit(1024).error_handler(|err, _req| {
        let status = match &err {
            error::JsonPayloadError::OverflowKnownLength { .. } | error::JsonPayloadError::Overflow { .. } => {
                actix_web::http::StatusCode::PAYLOAD_TOO_LARGE
            }
            _ => actix_web::http::StatusCode::BAD_REQUEST,
        };
        let body = json!({ "error": err.to_string(), "code": status.as_u16() });
        error::InternalError::from_response(err, HttpResponse::build(status).json(body)).into()
    })
}

/// Сборка приложения вынесена, чтобы тесты строили то же самое.
pub fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(json_cfg()).configure(routes::config);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let workers: usize = std::env::var("WORKERS").ok().and_then(|v| v.parse().ok()).unwrap_or(2);
    let addr = ("127.0.0.1", 8080);

    // Создаётся ОДИН раз: общий Arc на все воркеры.
    let global = web::Data::new(Global { hits: AtomicU64::new(0) });

    log::info!("main thread: {:?}", std::thread::current().name());

    HttpServer::new(move || {
        // Это замыкание вызывается в КАЖДОМ воркере.
        log::info!("App factory on {:?}", std::thread::current().name());
        let per_worker = PerWorker { hits: Rc::new(Cell::new(0)) };
        App::new()
            .app_data(global.clone())
            .app_data(per_worker)
            .wrap(middleware::RequestId)
            .wrap(from_fn(middleware::timing))
            .wrap(Logger::new("%a \"%r\" %s %b %Dms"))
            .configure(app_config)
    })
    .workers(workers)
    .keep_alive(Duration::from_secs(5))
    .client_request_timeout(Duration::from_secs(2))
    .client_disconnect_timeout(Duration::from_secs(1))
    .shutdown_timeout(5)
    .bind_auto_h2c(addr)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test};

    fn per_worker() -> PerWorker {
        PerWorker { hits: Rc::new(Cell::new(0)) }
    }

    #[actix_web::test]
    async fn hits_counts_both_levels() {
        let global = web::Data::new(Global { hits: AtomicU64::new(0) });
        let app = test::init_service(
            App::new().app_data(global.clone()).app_data(per_worker()).configure(app_config),
        )
        .await;
        for _ in 0..3 {
            let req = test::TestRequest::get().uri("/hits").to_request();
            let _: serde_json::Value = test::call_and_read_body_json(&app, req).await;
        }
        assert_eq!(global.hits.load(std::sync::atomic::Ordering::Relaxed), 3);
    }

    #[actix_web::test]
    async fn notes_require_client_id() {
        let app = test::init_service(App::new().configure(app_config)).await;
        let req = test::TestRequest::post().uri("/notes").set_json(json!({"text":"hi"})).to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn json_limit_gives_413() {
        let app = test::init_service(App::new().configure(app_config)).await;
        let big = "x".repeat(2000);
        let req = test::TestRequest::post()
            .uri("/notes")
            .insert_header(("x-client-id", "alice"))
            .set_json(json!({ "text": big }))
            .to_request();
        let res = test::call_service(&app, req).await;
        assert_eq!(res.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[actix_web::test]
    async fn middleware_adds_headers() {
        let app = test::init_service(
            App::new()
                .wrap(middleware::RequestId)
                .wrap(from_fn(middleware::timing))
                .configure(app_config),
        )
        .await;
        let res = test::call_service(&app, test::TestRequest::get().uri("/fast").to_request()).await;
        assert!(res.headers().contains_key("x-request-id"));
        assert!(res.headers().contains_key("x-response-time-us"));
    }
}
