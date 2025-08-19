use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct ImmediateFuture;

impl Future for ImmediateFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(42) // The result is ready immediately
    }
}

fn main() {
    let future = ImmediateFuture;
    let result = futures::executor::block_on(future);
    println!("The immediate value is: {}", result);
}
