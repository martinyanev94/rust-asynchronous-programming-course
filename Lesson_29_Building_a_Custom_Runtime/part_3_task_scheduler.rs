impl Wake<'_> {
    fn wake(self: Arc<Self>) {
        // Re-add the task to the runtime so it gets polled again
        RUNTIME.lock().unwrap().tasks.push_back(self.task.take().unwrap());
    }
}
fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    while let Some(mut task) = self.tasks.pop_front() {
        let waker = Wake { task: &mut task };
        let mut context = Context::from_waker(&waker);

        match task.future.as_mut().poll(&mut context) {
            Poll::Ready(()) => {
                println!("Task completed!");
            }
            Poll::Pending => {
                // Store the waker for future polling
                task.waker = Some(context.waker().clone());
                self.tasks.push_back(task);
            }
        }
    }
    Ok(())
}
