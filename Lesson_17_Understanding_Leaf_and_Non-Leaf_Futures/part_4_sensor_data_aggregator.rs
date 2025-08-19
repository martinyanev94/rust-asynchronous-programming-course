use tokio::time::{sleep, Duration};
use futures::stream::{self, StreamExt};

async fn fetch_sensor_data(sensor_id: u32) -> f64 {
    sleep(Duration::from_secs(1)).await; // Simulated data fetch delay
    sensor_id as f64 * 1.5 // Simulating data returned
}

async fn aggregate_data(sensor_ids: Vec<u32>) -> f64 {
    let futures = sensor_ids.into_iter().map(fetch_sensor_data);
    let results: Vec<f64> = futures::future::join_all(futures).await; // Collecting results
    results.iter().sum() // Aggregating results
}

#[tokio::main]
async fn main() {
    let sensor_ids = vec![1, 2, 3, 4, 5];
    let aggregated_value = aggregate_data(sensor_ids).await;
    println!("Aggregated value from sensors: {}", aggregated_value);
}
