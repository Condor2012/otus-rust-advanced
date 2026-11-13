use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
    routing::get,
};
use futures::{SinkExt, StreamExt};
use tokio::sync::broadcast;

#[derive(Clone)]
struct AppState {
    tx: broadcast::Sender<String>,
}

async fn ws_handler(ws: WebSocketUpgrade, State(st): State<AppState>) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, st))
}

async fn handle_socket(socket: WebSocket, st: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = st.tx.subscribe();

    // broadcast -> этот клиент
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break; // клиент отключился
            }
        }
    });

    // этот клиент -> broadcast
    let tx = st.tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    let _ = tx.send(text.to_string());
                }
                Message::Close(_) => break,
                _ => {} // Ping/Pong axum обрабатывает сам
            }
        }
    });

    // кто первый завершился — гасим второго
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(100);
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .with_state(AppState { tx });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("ws://localhost:3001/ws");
    axum::serve(listener, app).await.unwrap();
}
