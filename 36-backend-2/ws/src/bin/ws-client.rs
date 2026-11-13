use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name = std::env::args().nth(1).unwrap_or_else(|| "anon".into());
    let (ws, _resp) = connect_async("ws://127.0.0.1:3001/ws").await?;
    let (mut write, mut read) = ws.split();

    // stdin -> сервер
    tokio::spawn(async move {
        let mut lines = tokio::io::AsyncBufReadExt::lines(tokio::io::BufReader::new(tokio::io::stdin()));
        while let Ok(Some(line)) = lines.next_line().await {
            let msg = format!("{name}: {line}");
            if write.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
        let _ = write.send(Message::Close(None)).await;
    });

    // сервер -> stdout
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => println!("{text}"),
            Ok(Message::Close(_)) | Err(_) => break, // сервер закрыл соединение
            Ok(_) => {}
        }
    }
    Ok(())
}
