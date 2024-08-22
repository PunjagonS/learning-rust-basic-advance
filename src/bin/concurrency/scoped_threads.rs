// --------------------------------------------
//               Scoped Threads
// --------------------------------------------

/*
    New feature for ensure the thread is gauranteed
    to have finished its execution, so there are no references
    to vector we can therefore reuse it.
*/

use std::thread;

fn main() {
    let mut vec = vec![1, 2, 3];
    // thread::spawn(move || {
    //     println!("vec: {:?}", vec);
    // });
    // println!("vec: {:?}", vec);          // Error vec is dropped.

    thread::scope(|some_scope| {
        some_scope.spawn(|| {
            println!("Thread inside scope.");
            println!("vec: {:?}", vec);
        });

        some_scope.spawn(|| {
            println!("Another thread inside scope.");
            /*
                Error borrowed cause thread 1 may live longer than this thread.
                and this thread try to borrow as mutable conflict to
                thread above.
            */
            // vec.push(5);
            println!("vec: {:?}", vec);
        });
    });

    println!("Thread scope finished.");
    vec.push(5);
    println!("vec: {:?}", vec);
}