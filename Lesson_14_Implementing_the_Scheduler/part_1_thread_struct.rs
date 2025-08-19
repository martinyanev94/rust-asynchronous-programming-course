struct Thread {
    stack: Vec<u8>,
    ctx: ThreadContext,
    state: State,
}
