use fearless_concurrency::parallel_increments;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;

#[derive(Debug)]
struct Settings {
    prefix: String,
}

fn main() {
    println!(
        "AtomicUsize после четырёх потоков: {}",
        parallel_increments(4, 100_000)
    );

    let command = Arc::new(Mutex::new(String::from("show database")));
    let worker_command = Arc::clone(&command);
    let handle = thread::spawn(move || {
        let mut command = worker_command.lock().expect("mutex poisoned");
        command.push_str(" --verbose");
    });
    handle.join().expect("worker thread panicked");
    println!("Mutex: {}", command.lock().expect("mutex poisoned"));

    let settings = Arc::new(RwLock::new(Settings {
        prefix: String::from("info"),
    }));
    let reader = Arc::clone(&settings);
    let handle = thread::spawn(move || {
        println!(
            "RwLock читает: {}",
            reader.read().expect("lock poisoned").prefix
        );
    });
    handle.join().expect("reader thread panicked");
    settings.write().expect("lock poisoned").prefix = String::from("debug");
    println!(
        "RwLock после записи: {}",
        settings.read().expect("lock poisoned").prefix
    );
}
