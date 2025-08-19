use std::future::Future;
use std::pin::Pin;

async fn fetch_data() -> String {
    // Simulate a network request with an example delay
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    String::from("Data received")
}

fn get_future_example() -> impl Future<Output = String> {
    fetch_data()
}
