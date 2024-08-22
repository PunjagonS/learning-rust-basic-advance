// --------------------------------------------
//                  Barriers
// --------------------------------------------

/*
    Barriers enable multiple threads synchronize
    the beginning of some computation. 
*/

use core::task;
use std::sync::{Arc, Barrier, Mutex};
use std::thread;

fn main() {
    let mut threads_vec = Vec::new();
    let tasks = Arc::new(Mutex::new(vec![]));

    /*
        5 is the number of threads that need to reach the barrier 
        before they can all process.
    */
    let barrier = Arc::new(Barrier::new(5));
    // Define range 5 align to input of barrier above.
    for i in 0..5 {
        let tasks = tasks.clone();

        let barrier = barrier.clone();

        let handle = thread::spawn(move || {
            // Task 1
            tasks.lock()
            .unwrap()
            .push(format!("Thread {i}, Completed its part on Task 1"));

            /*
                Block the calling the thread until all 
                the threads reach this point.
            */
            barrier.wait();

            // Task 2
            tasks.lock()
            .unwrap()
            .push(format!("Thread {i}, Completed its part on Task 2"));
        });
        threads_vec.push(handle);
    }

    for handle in threads_vec {
        handle.join().unwrap();
    }

    let task_lock = tasks.lock().unwrap();
    // Dereference(*) from MutexGard and add reference for borrow here.
    for contents in &*task_lock {
        println!("{contents}");
    }
}