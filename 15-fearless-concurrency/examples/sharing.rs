use std::sync::Arc;
use std::thread;

fn main() {
    let text = String::from("scoped threads");
    thread::scope(|scope| {
        scope.spawn(|| println!("Длина локальной строки: {}", text.len()));
    });

    let command = Arc::new(String::from("show database"));
    let mut handles = Vec::new();
    for worker in 0..3 {
        let command = Arc::clone(&command);
        handles.push(thread::spawn(move || {
            println!("Поток {worker}: {}", command.as_str());
        }));
    }
    for handle in handles {
        handle.join().expect("worker thread panicked");
    }
    println!("Сильных ссылок Arc: {}", Arc::strong_count(&command));
}
