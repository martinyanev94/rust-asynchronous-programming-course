fn main() {
    let mut runtime = Runtime::new();
    
    runtime.spawn(|| {
        println!("Fiber 1 starting");
        for i in 0..10 {
            println!("Fiber 1 iteration: {}", i);
            runtime.yield(); // Switch context to allow other fibers to run
        }
        println!("Fiber 1 finished");
    });

    runtime.spawn(|| {
        println!("Fiber 2 starting");
        for i in 0..15 {
            println!("Fiber 2 iteration: {}", i);
            runtime.yield(); // Yield control
        }
        println!("Fiber 2 finished");
    });

    runtime.run(); // Execute the fibers.
}
