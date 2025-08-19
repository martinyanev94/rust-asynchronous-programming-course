struct SimpleExecutor;

impl SimpleExecutor {
    fn run<F>(&self, future: F) 
    where 
        F: Future<Output = String>,
    {
        let mut future = Box::pin(future);
        loop {
            let waker = std::task::noop_waker_ref();
            let mut context = Context::from_waker(&waker);

            match future.as_mut().poll(&mut context) {
                Poll::Ready(value) => {
                    println!("Future resolved with: {}", value);
                    break;
                }
                Poll::Pending => {
                    println!("Still pending...");
                }
            }
        }
    }
}
