let mut registry = Registry { poll: Poll::new().unwrap() };
let mut incoming_streams = Vec::new();
loop {
    match listener.accept() {
        Ok((stream, _)) => {
            stream.set_nonblocking(true).unwrap();
            registry.register(&stream, incoming_streams.len(), EPOLLIN).unwrap();
            incoming_streams.push(stream);
        },
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // No incoming connections at the moment
        },
        Err(e) => {
            eprintln!("Failed to accept connection: {}", e);
        }
    }
    // Handle polling
    ...
}
