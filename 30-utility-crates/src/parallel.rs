use crossbeam::atomic::AtomicCell;
use crossbeam::sync::{Parker, WaitGroup};
use crossbeam::utils::Backoff;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn parallel_sum(values: &[i32]) -> i32 {
    values.par_iter().sum()
}

pub fn rayon_join(a: u32, b: f32) -> (u32, f32) {
    rayon::join(|| a + 1, || b + 0.5)
}

pub fn atomic_updates() -> usize {
    let number = AtomicCell::new(42usize);
    let text = AtomicCell::new("hello".to_string());
    number.fetch_add(text.take().len());
    number.take()
}

pub fn park_until_ready() {
    let parker = Parker::new();
    let unparker = parker.unparker().clone();
    std::thread::spawn(move || unparker.unpark());
    parker.park();
}

pub fn spin_wait(ready: &AtomicBool) {
    let backoff = Backoff::new();
    while !ready.load(Ordering::Acquire) {
        backoff.snooze();
    }
}

pub fn wait_for_workers(values: Vec<i32>) -> i32 {
    let result = AtomicCell::new(0i32);
    std::thread::scope(|scope| {
        let group = WaitGroup::new();
        for value in values {
            let group = group.clone();
            let result = &result;
            scope.spawn(move || {
                result.fetch_add(value);
                drop(group);
            });
        }
        group.wait();
    });
    result.into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rayon_examples_return_deterministic_results() {
        assert_eq!(parallel_sum(&[1, 2, 3, 4]), 10);
        assert_eq!(rayon_join(42, 42.0), (43, 42.5));
    }

    #[test]
    fn crossbeam_examples_complete() {
        assert_eq!(atomic_updates(), 47);
        park_until_ready();
        assert_eq!(wait_for_workers(vec![1, 2, 3, 4]), 10);
    }
}
