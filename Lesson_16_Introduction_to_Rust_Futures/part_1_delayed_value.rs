use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct DelayedValue;

impl Future for DelayedValue {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(42) // Simulate a computation that is ready immediately
    }
}

fn main() {
    let future = DelayedValue;
    let result = futures::executor::block_on(future);
    println!("The value is: {}", result);
}
