pub fn poll(&mut self, events: &mut Vec<Event>, timeout: Option<i32>) -> Result<()> {
    let timeout = timeout.unwrap_or(-1);
    let res = unsafe { ffi::epoll_wait(self.raw_fd, events.as_mut_ptr(), events.capacity() as i32, timeout) };
    if res < 0 {
        return Err(io::Error::last_os_error());
    }
    unsafe { events.set_len(res as usize); }
    Ok(())
}
