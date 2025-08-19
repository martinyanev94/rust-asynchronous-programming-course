use futures::future::join;
use tokio::time::{sleep, Duration};

async fn fetch_data() -> i32 {
    sleep(Duration::from_secs(1)).await; // Simulating a delay
    42
}

async fn compute_result(data: i32) -> i32 {
    data * 2
}

async fn process_data() -> i32 {
    let data_future = fetch_data(); // This is a leaf future
    let result_future = compute_result(data_future.await); // Awaiting the leaf future
    result_future.await // This is a non-leaf future
}

#[tokio::main]
async fn main() {
    let result = process_data().await;
    println!("Processed result: {}", result);
}
