// --------------------------------------------
//               Thread Park
// --------------------------------------------

/*
    To halt specific thread at specific time relate to
    line of code to let other thread do some work before.
*/

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let data = Arc::new(Mutex::new(5));
    let data_clone = data.clone();

    let thread_1 = thread::spawn(move || {
        println!("Thread 1 : I am doing some work");
        println!("Thread 1 : I am doing some more work");

        /*
            Solution 1: use thread::sleep

            Use thread sleep to ensure that thread_1
            will print data after thread_2 updated. but
            this is not effective way because we do not know
            in advance for how long we need to sleep.
        */
        // thread::sleep(Duration::from_secs(2));

        println!("Thread 1: Parked");

        /*
            Solution 2: use thread::park

            Difference between thread sleep and park_timeout is
            "sleep" unconditionally blocks the calling thread but
            "park_timeout" conditionally blocks the calling thread
            based on the unpark function or time expired duration.
        */
        // thread::park(); // Halt thread_1 to let thread 2 update data.
        thread::park_timeout(Duration::from_secs(4));

        println!("Thread 1 : Printing the updated data");
        println!("Thread 1 : Data: {:?}", *data.lock().unwrap());
    });
    let thread_2 = thread::spawn(move || {
        println!("Thread 2: I am working on updating the data");
        thread::sleep(Duration::from_secs(1));
        *data_clone.lock().unwrap() = 10;
        println!("Thread 2: Data updated completed");
    });

    // Solution 1
    // thread_2.join();
    // thread_1.join();

    // Solution 2
    thread_2.join();
    thread_1.thread().unpark(); // Unpark thread_1 to printed updated data.
    thread_1.join();
}
