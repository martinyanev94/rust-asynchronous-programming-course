fn handle_events(event: &Event, streams: &[TcpStream]) -> Result<()> {
    if event.events == EPOLLIN { // Check for readability
        let index = event.epoll_data as usize;
        let mut buffer = [0; 4096];
        let bytes_read = streams[index].read(&mut buffer)?;
        
        if bytes_read > 0 {
            println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
            // Here we would typically respond back to the client
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 9\r\n\r\nHandled";
            streams[index].write_all(response.as_bytes())?;
        }
    }
    Ok(())
}
