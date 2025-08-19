use std::pin::Pin;
use std::future::Future;

struct MyFuture {
    value: String,
}

impl Future for MyFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _: &std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let this = self.get_ref(); // Get a reference to the underlying value
        std::task::Poll::Ready(this.value.clone())
    }
}

fn main() {
    let my_future = MyFuture {
        value: String::from("Hello, Future!"),
    };

    let pinned_future = Box::pin(my_future); // Pinning the future
}
