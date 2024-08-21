// --------------------------------------------
//           Ownership and Threads
// --------------------------------------------

/*
    Prerequiste: Closures FnOnce
*/

use std::{thread, time::Duration};

fn main() {
    /*
        This code will be executed in a spawn thread.
        
        This closure is not updateing x, so rust will infer that
        inside closure it will be used immutably. Rust cannot
        tell how long the spawn thread may live. The spawn thread
        may live LONGER than main thread. That will lead to problem,
        because this closure borrows x immutably. At the end of the main,
        the variable x will go out of scope, which means that 
        the reference inside the closure will be Dangling Reference.

        To fix this, so we use "move" to transfer ownership of x to
        inside closure.
    */
    let x = "some string".to_string();
    // thread::spawn(|| {           // Error closure need to take ownership of x.
    thread::spawn(move || {
        println!("{x}");
    });
    // println!("{x}");


    /*
        The primitives are not move but rather copied.
        In this case, "move" keyword makes another coppy
        of x which resides inside the thread not borrow.
    */
    let x = 5;
    thread::spawn(move || {
        println!("{x}");
    });
    println!("{x}");    

    
    /*
        The "move" keyword may not required if the closure is implementing
        FnOnce straight.

        A closure implements an FnOnce straight if it is taking ownership
        of variable from its environment or context.
    */
    let x = "some string".to_string();
    // thread::spawn(move || {
    thread::spawn( || {
        let y = x;    // Taking the ownership of x inside the closure.                  
        println!("{y}");
    });
    // println!("{x}");
    /*
        In summary, the code will still compile without the "move" keyword.
        This is because the rust infers that the closure is implementing
        FnOnce, since it is using the variable from its environment 
        through transfer of ownership. 

        Threads in rust are isolated from each other 
        automatically due to ownership

        This ensure that data races will never occur.
    */

    thread::sleep(Duration::from_millis(1));
}