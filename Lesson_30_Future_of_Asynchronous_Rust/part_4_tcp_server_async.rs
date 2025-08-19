use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

async fn handle_connection(mut socket: TcpStream) {
    println!("New connection: {:?}", socket.peer_addr());
    let mut buf = vec![0; 1024];

    while match socket.read(&mut buf).await {
        Ok(0) => return,
        Ok(n) => {
            // Assume we process input here
            println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
            true
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            false
        }
    } {}
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(handle_connection(socket));
    }
}
