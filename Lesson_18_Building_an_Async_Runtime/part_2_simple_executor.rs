use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct SimpleExecutor;

impl SimpleExecutor {
    pub fn run<F>(&self, future: F) where F: Future<Output = ()> {
        let mut future = Box::pin(future);

        loop {
            let waker = futures::task::waker_fn(|| {});
            let mut context = Context::from_waker(&waker);

            match future.as_mut().poll(&mut context) {
                Poll::Ready(()) => break,
                Poll::Pending => {}
            }
        }
    }
}

async fn say_hello() {
    println!("Hello, world!");
}

fn main() {
    let executor = SimpleExecutor;
    executor.run(say_hello());
}
