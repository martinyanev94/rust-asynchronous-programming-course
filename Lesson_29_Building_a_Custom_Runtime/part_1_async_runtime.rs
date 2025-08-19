use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::future::Future;

struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
    waker: Option<Waker>,
}

struct Runtime {
    tasks: VecDeque<Task>,
}
impl Runtime {
    fn new() -> Self {
        Runtime {
            tasks: VecDeque::new(),
        }
    }

    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        let task = Task {
            future: Box::pin(future),
            waker: None,
        };
        self.tasks.push_back(task);
    }
}
