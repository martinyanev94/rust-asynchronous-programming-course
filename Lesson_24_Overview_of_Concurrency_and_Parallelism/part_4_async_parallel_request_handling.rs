use tokio::task;
use rayon::prelude::*;

async fn handle_request(request_id: u32) {
    // Simulate processing the request
    println!("Handling request {}", request_id);
    let response = task::spawn_blocking(move || {
        // Intensive computation performed in parallel (blocking the current thread)
        let data: Vec<i32> = (1..=100_000).collect();
        let sum: i32 = data.par_iter().map(|&x| x * 2).sum();
        println!("Processed request {} with response {}", request_id, sum);
    }).await.unwrap();
}

#[tokio::main]
async fn main() {
    let requests = vec![1, 2, 3, 4, 5]; // Simulated request IDs

    for id in requests {
        task::spawn(handle_request(id)); // Handling each request concurrently
    }

    // Wait to ensure all requests are handled
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}
