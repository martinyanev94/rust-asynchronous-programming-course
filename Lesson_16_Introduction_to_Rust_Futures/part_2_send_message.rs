use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

async fn send_message() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await.unwrap();
    stream.write_all(b"Hello, world!").await.unwrap();
    println!("Message sent!");
}

fn main() {
    let future = send_message();
    tokio::runtime::Runtime::new().unwrap().block_on(future);
}
