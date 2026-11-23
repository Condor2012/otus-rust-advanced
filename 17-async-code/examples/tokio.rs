use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let start = Instant::now();
    tokio::join!(
        sleep(Duration::from_millis(100)),
        sleep(Duration::from_millis(200)),
        sleep(Duration::from_millis(300)),
    );
    println!("join!: {:.3} с", start.elapsed().as_secs_f64());

    let start = Instant::now();
    tokio::select! {
        () = sleep(Duration::from_millis(100)) => println!("select!: победила первая задача"),
        () = sleep(Duration::from_millis(200)) => println!("select!: победила вторая задача"),
        () = sleep(Duration::from_millis(300)) => println!("select!: победила третья задача"),
    }
    println!("select!: {:.3} с", start.elapsed().as_secs_f64());
}
