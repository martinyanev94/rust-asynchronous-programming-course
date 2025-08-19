use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let future = fetch_data();
    println!("Fetching data...");
    let result = future.await;
    println!("Data received: {}", result);
}

async fn fetch_data() -> String {
    sleep(Duration::from_secs(2)).await;
    "Hello from Rust!".to_string()
}
