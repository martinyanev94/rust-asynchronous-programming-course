#[derive(Debug)]
enum MyError {
    TaskFailed(String),
    Timeout,
    Other(String),
}

async fn perform_async_task() -> Result<(), MyError> {
    // Hypothetical async operation that might fail
    Err(MyError::TaskFailed("Operation failed".to_string()))
}

#[tokio::main]
async fn main() {
    match perform_async_task().await {
        Ok(_) => println!("Task completed successfully."),
        Err(e) => println!("Error occurred: {:?}", e),
    }
}
