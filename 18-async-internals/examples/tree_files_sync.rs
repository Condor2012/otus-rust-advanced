#![allow(unused_variables)]
#![allow(dead_code)]

use std::{fs, time::Instant};

fn main() {
    let start = Instant::now();

    println!("Читаем первый файл...");
    let file1 = fs::read_to_string("file1.txt").unwrap();

    println!("Читаем второй файл...");
    let file2 = fs::read_to_string("file2.txt").unwrap();

    println!("Читаем третий файл...");
    let file3 = fs::read_to_string("file3.txt").unwrap();

    println!("Всё прочитано за {:?}", start.elapsed());
}
