match self.state {
    State::Wait1(ref mut fut) => {
        match fut.poll(waker) {
            PollState::Ready(response) => {
                println!("{response}");
                // Transition to Wait2 or next state
                self.state = State::Wait2(another_future);
            }
            PollState::NotReady => break PollState::NotReady,
        }
    }
}
