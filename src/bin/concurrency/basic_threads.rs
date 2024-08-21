// --------------------------------------------
//              Basic Threads
// --------------------------------------------

/*
    Concurency is the multi tasks be run on random order.
    Parallelism is the multi tasks be run on multi threads at the same time on hardware.
*/

use std::thread;
use std::time::Duration;

fn main() {
    println!("This will be printed.");
    println!("This will also be printed.");
    println!("The concurrency will start after this line.");

    // thread::spawn(|| {
    let t = thread::spawn(|| {
        println!("Hello 1 from the thread");
        println!("Hello 2 from the thread");
        println!("Hello 3 from the thread");
        println!("Hello 4 from the thread");
        println!("Hello 5 from the thread");
        println!("Hello 6 from the thread");
        println!("Hello 7 from the thread");
    });

    // Using sleep() to block main thread and let other threads work(not guaranteed!)
    // thread::sleep(Duration::from_millis(1));

    
    // t.join();                            // Using join() to block main thread (guaranteed!)
    println!("Hello 1 from the main");
    println!("Hello 2 from the main");
    println!("Hello 3 from the main");
    t.join();                               // Using join() to block main thread (guaranteed!)
}