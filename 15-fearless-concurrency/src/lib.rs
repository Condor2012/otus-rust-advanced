use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, mpsc};
use std::thread;

pub fn parallel_increments(thread_count: usize, increments_per_thread: usize) -> usize {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    for _ in 0..thread_count {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..increments_per_thread {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for handle in handles {
        handle.join().expect("worker thread panicked");
    }
    counter.load(Ordering::Relaxed)
}

pub fn pipeline_sum(values: Vec<i32>) -> i32 {
    let (input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::channel();

    let worker = thread::spawn(move || {
        for value in input_rx {
            output_tx.send(value * 2).expect("output receiver dropped");
        }
    });

    for value in values {
        input_tx.send(value).expect("input receiver dropped");
    }
    drop(input_tx);

    let sum = output_rx.iter().sum();
    worker.join().expect("pipeline worker panicked");
    sum
}

#[cfg(test)]
mod tests {
    use super::{parallel_increments, pipeline_sum};
    use std::thread;

    #[test]
    fn atomic_counter_keeps_all_increments() {
        assert_eq!(parallel_increments(4, 1_000), 4_000);
    }

    #[test]
    fn channel_pipeline_processes_every_value() {
        assert_eq!(pipeline_sum(vec![1, 2, 3, 4]), 20);
    }

    #[test]
    fn scoped_threads_can_borrow_local_data() {
        let text = String::from("shared");
        let length = thread::scope(|scope| {
            let handle = scope.spawn(|| text.len());
            handle.join().expect("scoped thread panicked")
        });
        assert_eq!(length, 6);
    }
}
