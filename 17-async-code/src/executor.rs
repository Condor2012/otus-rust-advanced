use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

/// Учебный однопоточный executor с активным повторным опросом задач.
#[derive(Default)]
pub struct Executor {
    tasks: Vec<Pin<Box<dyn Future<Output = ()>>>>,
}

impl Executor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn(&mut self, future: impl Future<Output = ()> + 'static) {
        self.tasks.push(Box::pin(future));
    }

    pub fn task_count(&self) -> usize {
        self.tasks.len()
    }

    pub fn run(&mut self) {
        while !self.tasks.is_empty() {
            let mut ready = Vec::new();
            let mut context = Context::from_waker(Waker::noop());

            for (index, task) in self.tasks.iter_mut().enumerate() {
                if task.as_mut().poll(&mut context) == Poll::Ready(()) {
                    ready.push(index);
                }
            }
            for index in ready.into_iter().rev() {
                drop(self.tasks.swap_remove(index));
            }
        }
    }
}
