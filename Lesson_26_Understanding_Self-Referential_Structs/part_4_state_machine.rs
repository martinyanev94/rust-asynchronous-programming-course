struct State {
    name: String,
    next: Option<Rc<State>>,
}

impl State {
    fn new(name: &str) -> State {
        State {
            name: name.to_string(),
            next: None,
        }
    }

    fn set_next(&mut self, next_state: Rc<State>) {
        self.next = Some(next_state);
    }

    fn transition(&self) {
        println!("Transitioning to state: {}", self.name);
        if let Some(ref next_state) = self.next {
            next_state.transition();
        }
    }
}

fn main() {
    let mut state1 = State::new("State 1");
    let state2 = Rc::new(State::new("State 2"));
    let state3 = Rc::new(State::new("State 3"));

    state1.set_next(state2.clone());
    state2.borrow_mut().set_next(state3.clone());

    state1.transition();
}
