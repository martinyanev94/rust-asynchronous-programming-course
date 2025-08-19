fn main() {
    let executor = SimpleExecutor;
    let delayed_future = DelayedFuture::new(Duration::new(3, 0)); // 3 seconds delay

    println!("Executing delayed future...");
    executor.run(delayed_future);
}
