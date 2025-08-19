impl Drop for Poll {
    fn drop(&mut self) {
        unsafe {
            ffi::close(self.raw_fd);
        }
    }
}
