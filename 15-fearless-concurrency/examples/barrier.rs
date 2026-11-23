use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let workers = 4;
    let barrier = Arc::new(Barrier::new(workers));
    let mut handles = Vec::new();

    for worker in 0..workers {
        let barrier = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("Поток {worker}: подготовка завершена");
            barrier.wait();
            println!("Поток {worker}: общий этап обработки");
        }));
    }

    for handle in handles {
        handle.join().expect("worker thread panicked");
    }
}
