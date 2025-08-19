use futures::future::join_all;

#[tokio::main]
async fn main() {
    let delays = vec![
        Delay::new(std::time::Duration::from_secs(2)),
        Delay::new(std::time::Duration::from_secs(3)),
        Delay::new(std::time::Duration::from_secs(1)),
    ];

    join_all(delays).await; // Awaits all delays to complete
    println!("All delays completed!");
}
