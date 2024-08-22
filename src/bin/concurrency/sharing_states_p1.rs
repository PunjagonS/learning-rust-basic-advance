// --------------------------------------------
//           Sharing States (Part 1)
// --------------------------------------------

/*
    Multiple threads access on the same data in memory
    with acquiring lock on the data. All this possible
    by using the special type called "Mutex".

    The data wrapped by a mutex can only be accessed by
    a single thread at any given time.
*/

use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    // Solution 1
    // let mut num = m.lock().unwrap();    // Call lock thread
    // *num = 10;                          // Assign value 10 to m (locked)                                              
    // drop(num);                          // Unlock m

    // Solution 2 : using code block for auto unlocked m
    {
        let mut num = m.lock().unwrap();
        println!("m is {:?}", *num);
        *num = 10;                                                                                   
    }

    let lock_m = m.lock().unwrap();     // Get latest value of m and locked
    println!("m is {:?}", *lock_m);
    drop(lock_m);                                        // Unlock m again             

    let lock_m1 = m.lock().unwrap();
    println!("This code is blocked.");
}