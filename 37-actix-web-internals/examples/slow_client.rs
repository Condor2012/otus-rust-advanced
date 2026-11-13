//! «Медленный клиент»: шлём заголовки по байту и смотрим, что ответит сервер.
use std::{
    io::{Read, Write},
    net::TcpStream,
    time::{Duration, Instant},
};

fn main() -> std::io::Result<()> {
    let mut s = TcpStream::connect("127.0.0.1:8080")?;
    let started = Instant::now();
    s.write_all(b"GET /fast HTTP/1.1\r\nHost: localhost\r\n")?; // без финального \r\n
    println!("sent partial headers, waiting...");
    let mut buf = Vec::new();
    s.set_read_timeout(Some(Duration::from_secs(10)))?;
    let _ = s.read_to_end(&mut buf);
    let text = String::from_utf8_lossy(&buf);
    println!("after {:.1?}: {}", started.elapsed(), text.lines().next().unwrap_or("<connection closed>"));
    Ok(())
}
