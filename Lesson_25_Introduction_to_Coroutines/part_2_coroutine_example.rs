struct CoroutineState {
    counter: Option<usize>,
}

// This will be utilized in the coroutine definition.
impl Default for CoroutineState {
    fn default() -> Self {
        CoroutineState { counter: Some(0) }
    }
}

struct Coroutine0 {
    state: CoroutineState,
}
coroutine fn async_main() {
    let mut coroutine = Coroutine0 {
        state: CoroutineState::default(),
    };

    println!("Program starting");
    
    // First HTTP request
    let txt = Http::get("/600/HelloAsyncAwait").wait;
    println!("{txt}");
    
    // Store state
    coroutine.state.counter = coroutine.state.counter.map(|c| c + 1);
    
    // Second HTTP request
    let txt = Http::get("/400/HelloAsyncAwait").wait;
    println!("{txt}");
    
    // Store state
    coroutine.state.counter = coroutine.state.counter.map(|c| c + 1);
    
    // Output final count
    let count = coroutine.state.counter.unwrap_or(0);
    println!("Received {} responses.", count);
}
