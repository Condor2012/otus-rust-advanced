use utility_crates::parallel::{
    atomic_updates, parallel_sum, park_until_ready, rayon_join, wait_for_workers,
};

fn main() {
    let (integer, float) = rayon_join(42, 42.0);
    println!("rayon::join: int = {integer}, float = {float}");
    println!("Параллельная сумма: {}", parallel_sum(&[1, 2, 3, 100]));
    println!("AtomicCell после fetch_add: {}", atomic_updates());
    park_until_ready();
    println!("Parker/Unparker: основной поток продолжен");
    println!("WaitGroup: сумма = {}", wait_for_workers(vec![1, 2, 3, 4]));
}
