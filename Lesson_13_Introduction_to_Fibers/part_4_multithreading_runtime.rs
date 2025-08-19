fn main() {
    let mut runtime = Runtime::new();
    runtime.init();
    runtime.spawn(|| {
        println!("THREAD 1 STARTING");
        for i in 0..10 {
            println!("thread: 1 counter: {}", i);
            runtime.yield_thread(); // Allow context switching here
        }
        println!("THREAD 1 FINISHED");
    });

    runtime.spawn(|| {
        println!("THREAD 2 STARTING");
        for i in 0..15 {
            println!("thread: 2 counter: {}", i);
            runtime.yield_thread();
        }
        println!("THREAD 2 FINISHED");
    });

    runtime.run(); // Initiate execution of the fibers
}
