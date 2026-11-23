use async_code::Pend3TimesFuture;
use std::future::Future;
use std::pin::pin;
use std::task::{Context, Waker};

fn main() {
    let mut future = pin!(async {
        println!("Hello");
        Pend3TimesFuture::new().await;
        println!("Hello again");
    });
    let mut context = Context::from_waker(Waker::noop());

    for _ in 0..4 {
        let _ = future.as_mut().poll(&mut context);
    }
}
