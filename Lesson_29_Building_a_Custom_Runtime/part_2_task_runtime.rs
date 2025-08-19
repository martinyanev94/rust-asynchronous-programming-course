impl Runtime {
    fn run(&mut self) {
        while let Some(mut task) = self.tasks.pop_front() {
            let waker = Wake { task: &mut task };
            let mut context = Context::from_waker(&waker);

            match task.future.as_mut().poll(&mut context) {
                Poll::Ready(()) => println!("Task completed!"),
                Poll::Pending => {
                    // If the task is still pending, we must save the waker.
                    task.waker = Some(context.waker().clone());
                    self.tasks.push_back(task);
                },
            }
        }
    }
}

struct Wake<'a> {
    task: &'a mut Task,
}

impl<'a> std::task::Wake for Wake<'a> {
    fn wake(self: Arc<Self>) {
        // Logic to wake the task and add it back to the runtime
    }
}
