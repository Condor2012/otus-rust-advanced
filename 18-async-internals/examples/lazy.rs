#![allow(unused_variables)]
#![allow(dead_code)]

use tokio::fs;

async fn expansive_operation() -> i32 {
    tokio::spawn(async {
        let content1 = fs::read_to_string("file1.txt").await.unwrap();
    });
    let content1 = fs::read_to_string("file1.txt").await.unwrap();
    println!("This will never print!");
    42
}

// #[tokio::main]
// async fn main() {
fn main() {
    let future = expansive_operation();

    // future.await;

    // Нет запуска ленивой функции
}
