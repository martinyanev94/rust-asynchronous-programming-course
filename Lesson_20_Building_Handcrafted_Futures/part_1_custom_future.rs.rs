use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct MyCustomFuture {
    completed: bool,
}

impl MyCustomFuture {
    fn new() -> Self {
        MyCustomFuture { completed: false }
    }
}

impl Future for MyCustomFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.completed {
            Poll::Ready("Future is complete!".to_string())
        } else {
            Poll::Pending
        }
    }
}
