use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut producers = Vec::new();
    for producer in 0..3 {
        let tx = tx.clone();
        producers.push(thread::spawn(move || {
            tx.send(producer * 10).expect("receiver dropped");
        }));
    }
    drop(tx);
    for producer in producers {
        producer.join().expect("producer panicked");
    }
    let values: Vec<_> = rx.iter().collect();
    println!(
        "MPSC: получено {} значения, сумма = {}",
        values.len(),
        values.iter().sum::<i32>()
    );

    let (sync_tx, sync_rx) = mpsc::sync_channel(1);
    let producer = thread::spawn(move || {
        for value in 1..=3 {
            sync_tx.send(value).expect("receiver dropped");
            println!("SyncSender отправил {value}");
        }
    });
    let received: Vec<_> = sync_rx.iter().collect();
    producer.join().expect("producer panicked");
    println!("sync_channel с буфером 1: {received:?}");
}
