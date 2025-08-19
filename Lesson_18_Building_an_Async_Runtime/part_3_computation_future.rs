use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::time::{sleep, Duration};

struct ComputationFuture {
    delay: Duration,
}

impl Future for ComputationFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.delay.is_zero() {
            Poll::Ready(42)  // The computation is immediately ready
        } else {
            let waker = cx.waker().clone();
            tokio::spawn(async move {
                sleep(self.delay).await;
                waker.wake();
            });
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let future = ComputationFuture { delay: Duration::from_secs(2) };
    let result = future.await;
    println!("Computation result: {}", result);
}
