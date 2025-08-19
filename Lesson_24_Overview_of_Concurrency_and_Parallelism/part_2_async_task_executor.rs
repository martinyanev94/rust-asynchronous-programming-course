use tokio::time::{sleep, Duration};

async fn perform_task(task_id: u32) {
    println!("Task {} is starting.", task_id);
    sleep(Duration::from_secs(2)).await; // Simulates a task taking 2 seconds
    println!("Task {} is completed.", task_id);
}

#[tokio::main]
async fn main() {
    let tasks = vec![perform_task(1), perform_task(2), perform_task(3)];
    
    for task in tasks {
        tokio::spawn(task);
    }

    // Wait a moment for all tasks to finish
    sleep(Duration::from_secs(3)).await;
    println!("All tasks are done!");
}
