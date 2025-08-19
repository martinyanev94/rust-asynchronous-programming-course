let listener = TcpListener::bind("127.0.0.1:8080").expect("Could not bind to address");
listener.set_nonblocking(true).expect("Failed to set non-blocking");
