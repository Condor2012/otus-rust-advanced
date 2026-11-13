#![allow(unused_variables)]
#![allow(dead_code)]

use std::time::Instant;

use tokio::fs;

#[tokio::main]
async fn main() {
    let start = Instant::now();

    let (file1, file2, file3) = tokio::join!(
        fs::read_to_string("file1.txt"),
        fs::read_to_string("file2.txt"),
        fs::read_to_string("file3.txt"),
    );

    println!("Всё прочитано за {:?}", start.elapsed());
}
