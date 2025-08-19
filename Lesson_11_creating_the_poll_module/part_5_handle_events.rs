pub fn handle_events(&self, events: &[Event], streams: &mut Vec<TcpStream>) -> Result<()> {
    for event in events {
        let index = event.epoll_data as usize; 
        let mut buffer = [0; 4096];
        let bytes_read = streams[index].read(&mut buffer)?;
        println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
    }
    Ok(())
}
