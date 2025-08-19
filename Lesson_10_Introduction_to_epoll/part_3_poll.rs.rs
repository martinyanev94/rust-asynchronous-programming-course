use std::io::{self, Result};
use std::net::TcpStream;
use crate::ffi;

type Events = Vec<ffi::Event>;

pub struct Poll {
    raw_fd: i32,
}

impl Poll {
    pub fn new() -> Result<Self> {
        let fd = unsafe { ffi::epoll_create(1) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Poll { raw_fd: fd })
    }

    pub fn poll(&mut self, events: &mut Events, timeout: Option<i32>) -> Result<()> {
        let timeout = timeout.unwrap_or(-1);
        let res = unsafe { ffi::epoll_wait(self.raw_fd, events.as_mut_ptr(), events.capacity() as i32, timeout) };
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
        unsafe { events.set_len(res as usize); }
        Ok(())
    }
}
