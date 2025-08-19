mod ffi;
mod poll;

use std::net::TcpStream;

fn main() -> Result<()> {
    let mut poll = poll::Poll::new()?;
    // Set up TCP streams, register them with poll, etc.
    Ok(())
}
