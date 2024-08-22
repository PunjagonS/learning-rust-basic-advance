// --------------------------------------------
//           Sharing States (Part 1)
// --------------------------------------------

/*
    Combine sharing states with multiple thread and
    using Arc (Atomically Reference Counted) smart pointer 
    to share ownership of mutex instead of Rc smart pointer
    because Arc will make sure that the reference count of
    each clones are updated in a consistance manner between
    the threads.

    Rc will subtract reference count when a clone or owner
    is dropped resulting in cannot make sure that the changes to
    the count will not be interupted by another thread.
*/

use std::sync::Mutex;
use std::thread;
// use std::rc::Rc;
use std::sync::Arc;

struct File {
    text: Vec<String>,
}

fn main() {
    // let file = Rc::new(Mutex::new(File {text: vec![]}));
    let file = Arc::new(Mutex::new(File {text: vec![]}));
    let mut thread_vec = vec![];

    for i in 1..10 {
        // let file = Rc::clone(&file);
        let file = Arc::clone(&file);
        let handle = thread::spawn(move || {
            let mut file_lock = file.lock().unwrap();
            file_lock.text.push(format!("Hell from thread {i}"));
        });
        thread_vec.push(handle);                                    // Store all threads in vec to call join() later.
    }

    // Make sure the threads go to completion by calling join().
    for handle in thread_vec {
        handle.join().unwrap();
    }

    let file_lock = file.lock().unwrap();
    for t in &file_lock.text {
        println!("{t}");
    }
}