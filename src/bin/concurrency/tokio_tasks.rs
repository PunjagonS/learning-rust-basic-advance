// --------------------------------------------
//               Tokio Tasks
// --------------------------------------------

/*
    Tokio tasks make our async code run concurrently.

    A task is a lightweight, non-blocking unit of execution and
    allow top level futures to execute concurrently.

    By default, Tokio uses the thread pool to execute tasks
    on multiple threads.
*/

use std::time::Duration;
use tokio::time::sleep;

async fn printing(i: i32) {
    /*
        The sleep function in tokio is similar to the thread sleep function,
        execpt that it will stop the current future from executing for
        a given duration only not entire thread, mean that other tasks from
        the same thread can be invoked to execute (non-blocking IO).
    */
    sleep(Duration::from_secs(1)).await;
    println!("Task {i}");
}

//#[tokio::main(flavor = "current_thread")] // Force tokio to run all tasks on single thread sequentially.
#[tokio::main]
async fn main() {
    let mut handles = vec![];

    for i in 0..3 {
        let handle = tokio::spawn(async move {
            println!("Task {i}, printing, first time.");
            printing(i).await;
            println!("Task {i}, printing, second time.");
            printing(i).await;
            println!("Task {i}, completed.");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("All tasks are now completed.");
}
