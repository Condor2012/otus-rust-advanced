use async_code::{Executor, TimerFuture};
use std::time::{Duration, Instant};

async fn read_request(client: u32) -> String {
    TimerFuture::new(Duration::from_millis(30)).await;
    format!("request from client {client}")
}

async fn send_reply(client: u32, reply: String) {
    TimerFuture::new(Duration::from_millis(10)).await;
    println!("Клиент {client}: {reply}");
}

fn main() {
    let mut executor = Executor::new();
    let start = Instant::now();

    for client in 0..3 {
        executor.spawn(async move {
            let request = read_request(client).await;
            let reply = format!("reply to {request}");
            send_reply(client, reply).await;
            let second_request = read_request(client).await;
            println!("Клиент {client}: повторный {second_request}");
        });
    }

    executor.run();
    println!(
        "Все задачи завершены за {:.3} с",
        start.elapsed().as_secs_f64()
    );
}
