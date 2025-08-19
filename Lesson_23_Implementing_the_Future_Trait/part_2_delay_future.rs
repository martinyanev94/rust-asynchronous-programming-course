pub struct Delay {
    start: std::time::Instant,
    duration: std::time::Duration,
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.start.elapsed() >= self.duration {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

impl Delay {
    pub fn new(duration: std::time::Duration) -> Self {
        Self {
            start: std::time::Instant::now(),
            duration,
        }
    }
}
