//! Грубый замер: cargo run --release --example mask_bench
use std::{hint::black_box, time::Instant};

use actix_internals_demo::unsafe_demo::{mask_align_to, mask_chunks, mask_naive};

fn bench(name: &str, f: fn(&mut [u8], [u8; 4])) {
    let mut buf = vec![0xAB_u8; 64 * 1024 * 1024 + 3];
    let mask = [0x6d, 0xb6, 0xb2, 0x80];
    f(black_box(&mut buf[1..]), mask); // прогрев
    let t = Instant::now();
    for _ in 0..10 {
        f(black_box(&mut buf[1..]), mask); // смещение 1 → невыровненное начало
    }
    let secs = t.elapsed().as_secs_f64();
    let gbps = (10.0 * 64.0) / 1024.0 / secs;
    println!("{name:<10} {:>7.1} ms   {gbps:>5.1} GiB/s", secs * 1000.0);
}

fn main() {
    bench("naive", mask_naive);
    bench("align_to", mask_align_to);
    bench("chunks", mask_chunks);
}
