mod executor;
mod timer;

pub use executor::Executor;
pub use timer::{Pend3TimesFuture, TimerFuture};

#[cfg(feature = "tokio")]
use tokio as _;

#[cfg(test)]
mod tests {
    use super::{Executor, Pend3TimesFuture, TimerFuture};
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};
    use std::time::Duration;

    #[test]
    fn future_is_pending_three_times() {
        let mut future = pin!(Pend3TimesFuture::new());
        let mut context = Context::from_waker(Waker::noop());

        for _ in 0..3 {
            assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
        }
        assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(()));
    }

    #[test]
    fn timer_finishes_after_deadline() {
        let mut future = pin!(TimerFuture::new(Duration::ZERO));
        let mut context = Context::from_waker(Waker::noop());
        assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(()));
    }

    #[test]
    fn executor_runs_async_tasks_to_completion() {
        let mut executor = Executor::new();
        executor.spawn(async {
            Pend3TimesFuture::new().await;
        });
        executor.spawn(async {
            TimerFuture::new(Duration::ZERO).await;
        });
        executor.run();
        assert_eq!(executor.task_count(), 0);
    }
}
