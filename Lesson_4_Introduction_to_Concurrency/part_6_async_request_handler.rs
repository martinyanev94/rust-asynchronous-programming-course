use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let request1 = request_handler("http://example.com/1");
    let request2 = request_handler("http://example.com/2");

    let response1 = request1.await;
    let response2 = request2.await;

    println!("{}", response1);
    println!("{}", response2);
}

async fn request_handler(url: &str) -> String {
    sleep(Duration::from_secs(1)).await;
    format!("Handled request for {}", url)
}
