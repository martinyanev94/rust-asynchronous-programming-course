use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); // Using Arc to share ownership

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // Locking the mutex 
            *num += 1; // Safely incrementing the counter
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // Ensure all threads complete
    }

    println!("Result: {}", *counter.lock().unwrap()); // Accessing the final value
}
