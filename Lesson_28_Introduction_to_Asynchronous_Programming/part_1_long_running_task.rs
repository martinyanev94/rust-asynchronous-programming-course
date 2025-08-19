use std::time::Duration;
use tokio::time::sleep;

async fn long_running_task() {
    println!("Task started...");
    sleep(Duration::from_secs(5)).await; // Simulates a long computation
    println!("Task completed!");
}
