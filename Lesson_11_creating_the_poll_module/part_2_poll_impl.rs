impl Poll {
    pub fn new() -> Result<Self> {
        let fd = unsafe { ffi::epoll_create(1) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(Poll { raw_fd: fd })
    }
}
