fn main() {
    let executor = SimpleExecutor;
    let my_future = MyCustomFuture::new();

    executor.run(my_future);
    
    // Now simulate a completion
    my_future.completed = true;
    
    executor.run(my_future);
}
