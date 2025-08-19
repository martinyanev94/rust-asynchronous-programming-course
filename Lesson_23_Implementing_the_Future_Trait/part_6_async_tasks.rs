async fn task_one() -> i32 {
    Delay::new(std::time::Duration::from_secs(1)).await;
    42 // Return some value
}

async fn task_two(result: i32) {
    println!("Received: {}", result);
}

#[tokio::main]
async fn main() {
    let result = task_one().await; // Wait for task_one to complete
    task_two(result).await; // Use the result in task_two
}
