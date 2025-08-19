#[inline(never)]
fn t_yield(&mut self) -> bool {
    let mut pos = self.current;
    while self.threads[pos].state != State::Ready {
        pos += 1;
        if pos == self.threads.len() {
            pos = 0;
        }
        if pos == self.current {
            return false; // No threads are ready to run
        }
    }

    if self.threads[self.current].state != State::Available {
        self.threads[self.current].state = State::Ready;
    }

    self.threads[pos].state = State::Running;
    let old_pos = self.current;
    self.current = pos;

    unsafe {
        let old: *mut ThreadContext = &mut self.threads[old_pos].ctx;
        let new: *const ThreadContext = &self.threads[pos].ctx;
        asm!("call switch", in("rdi") old, in("rsi") new, clobber_abi("C"));
    }
    true
}
