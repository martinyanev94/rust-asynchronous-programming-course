fn handle_events(events: &[ffi::Event], streams: &mut Vec<TcpStream>) -> Result<()> {
    for event in events {
        let index = event.token() as usize;  // Make sure to have proper type handling!
        let mut buffer = [0; 4096];
        let bytes_read = streams[index].read(&mut buffer)?;
        println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
    }
    Ok(())
}
