/*
Developers are encouraged to use Rust's standard
library features, such as threads, `Arc`, and `Mutex`, to write concurrent
programs that are free from data races and other concurrency issues.

The Arc stands for Atomic Reference Counter. It is a thread-safe reference counter that allows data to be shared between threads.
When the last reference is dropped, the data is automatically cleaned up.

Mutex stands for Mutual Exclusion. It is a concurrency primitive that ensures only one thread can access some data at any point in time. If a thread needs to read or write the data, it must first acquire the lock on the Mutex.

In the above code, we have a counter that is incremented by 10 threads concurrently. The Arc ensures that all threads have a reference to the counter, and the Mutex ensures that only one thread can increment the counter at a time.



https://reintech.io/blog/understanding-implementing-rust-arc-mutex
*/
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Counter: {}", *counter.lock().unwrap());
}
