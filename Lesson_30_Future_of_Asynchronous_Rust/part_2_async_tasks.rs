use tokio::time::{sleep, Duration};
use tokio::task;

#[tokio::main]
async fn main() {
    let task1 = task::spawn(async {
        println!("Task 1 started");
        sleep(Duration::from_secs(2)).await;
        println!("Task 1 completed");
    });

    let task2 = task::spawn(async {
        println!("Task 2 started");
        sleep(Duration::from_secs(1)).await;
        println!("Task 2 completed");
    });

    task1.await.unwrap();
    task2.await.unwrap();
}
