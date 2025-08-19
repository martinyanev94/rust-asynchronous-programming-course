use tokio::time::{sleep, Duration};

async fn example_task() {
    println!("Example task is starting...");
    sleep(Duration::from_secs(2)).await;
    println!("Example task has finished!");
}
fn main() {
    let mut runtime = Runtime::new();
    runtime.spawn(example_task());  // Spawn the example task
    runtime.run();                   // Run our custom runtime
}
