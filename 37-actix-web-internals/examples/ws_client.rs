//! WebSocket-клиент на awc (тот же стек actix).
use actix_web::web::Bytes;
use awc::ws;
use futures_util::{SinkExt, StreamExt};

#[actix_web::main]
async fn main() {
    let (resp, mut conn) = awc::Client::new()
        .ws("ws://127.0.0.1:8080/ws")
        .connect()
        .await
        .expect("connect");
    println!("handshake: {}", resp.status());

    let msgs = [
        ws::Message::Text("привет".into()),
        ws::Message::Text("actix".into()),
        ws::Message::Ping(Bytes::from_static(b"p")),
        ws::Message::Close(Some(ws::CloseCode::Normal.into())),
    ];
    for msg in msgs {
        println!("-> {msg:?}");
        conn.send(msg).await.unwrap();
        match conn.next().await {
            Some(Ok(ws::Frame::Text(b))) => println!("<- Text: {}", String::from_utf8_lossy(&b)),
            Some(Ok(frame)) => println!("<- {frame:?}"),
            Some(Err(e)) => println!("<- error: {e}"),
            None => println!("<- connection closed"),
        }
    }
}
