use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

#[derive(Debug, Default)]
pub struct Pend3TimesFuture {
    times: u32,
}

impl Pend3TimesFuture {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Future for Pend3TimesFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        println!("Poll {} times", self.times);
        if self.times < 3 {
            self.times += 1;
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

#[derive(Debug)]
pub struct TimerFuture {
    deadline: Instant,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        Self {
            deadline: Instant::now() + duration,
        }
    }
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.deadline {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}
