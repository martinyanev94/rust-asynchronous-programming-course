struct Runtime {
    threads: Vec<Thread>,
    current: usize,
}
impl Runtime {
    pub fn new() -> Self {
        let mut threads = Vec::new();
        for _ in 0..MAX_THREADS {
            threads.push(Thread {
                stack: vec![0; DEFAULT_STACK_SIZE],
                ctx: ThreadContext::default(),
                state: State::Available,
            });
        }
        Runtime { threads, current: 0 }
    }

    pub fn spawn(&mut self, f: fn()) {
        let available = self
            .threads
            .iter_mut()
            .find(|t| t.state == State::Available)
            .expect("No available threads.");

        let size = available.stack.len();
        unsafe {
            let s_ptr = available.stack.as_mut_ptr().offset(size as isize);
            let s_ptr = (s_ptr as usize & !15) as *mut u8; // Align to 16 bytes
            std::ptr::write(s_ptr.offset(-16) as *mut u64, guard as u64);
            std::ptr::write(s_ptr.offset(-24) as *mut u64, skip as u64);
            std::ptr::write(s_ptr.offset(-32) as *mut u64, f as u64);
            available.ctx.rsp = s_ptr.offset(-32) as u64;
        }

        available.state = State::Ready;
    }
}
