use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::thread;

struct DelayedFuture {
    start: Instant,
    duration: Duration,
    completed: bool,
}

impl DelayedFuture {
    fn new(duration: Duration) -> Self {
        DelayedFuture {
            start: Instant::now(),
            duration,
            completed: false,
        }
    }
}

impl Future for DelayedFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.completed && self.start.elapsed() >= self.duration {
            self.completed = true;
            Poll::Ready("Operation completed after delay!".to_string())
        } else {
            Poll::Pending
        }
    }
}
