pub struct Registry {
    poll: Poll,
}

impl Registry {
    pub fn register(&self, source: &TcpStream, token: usize, interests: i32) -> Result<()> {
        let event = Event {
            events: interests as u32,
            epoll_data: token,
        };
        let res = unsafe { ffi::epoll_ctl(self.poll.raw_fd, ffi::EPOLL_CTL_ADD, source.as_raw_fd(), &event as *const _ as *mut _) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}
