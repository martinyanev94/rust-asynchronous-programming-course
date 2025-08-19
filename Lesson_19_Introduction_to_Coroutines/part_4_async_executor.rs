fn main() {
    let future = async_main();
    let mut executor = Executor::new(); // Assume we have an executor setup
    executor.run(future); // Run our future until it's done
}
