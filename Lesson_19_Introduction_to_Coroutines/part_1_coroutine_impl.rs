struct Coroutine {
    state: State,
}

enum State {
    Start,
    Wait1(Box<dyn Future<Output = String>>),
    Wait2(Box<dyn Future<Output = String>>),
    Resolved,
}

impl Coroutine {
    fn new() -> Self {
        Self {
            state: State::Start,
        }
    }
}
