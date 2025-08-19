impl Future for Coroutine {
    type Output = ();

    fn poll(&mut self) -> PollState<Self::Output> {
        loop {
            match self.state {
                State::Start => {
                    println!("Coroutine starting.");
                    let future = Box::new(Http::get("/some_endpoint"));
                    self.state = State::Wait1(future);
                }
                State::Wait1(ref mut future) => match future.poll() {
                    PollState::Ready(response) => {
                        println!("Response received: {}", response);
                        let next_future = Box::new(Http::get("/another_endpoint"));
                        self.state = State::Wait2(next_future);
                    }
                    PollState::NotReady => break PollState::NotReady,
                },
                State::Wait2(ref mut future) => match future.poll() {
                    PollState::Ready(response) => {
                        println!("Second response received: {}", response);
                        self.state = State::Resolved;
                        break PollState::Ready(());
                    }
                    PollState::NotReady => break PollState::NotReady,
                },
                State::Resolved => panic!("Coroutine was polled after being resolved."),
            }
        }
    }
}
