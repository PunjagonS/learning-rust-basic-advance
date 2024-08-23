// --------------------------------------------
//   Message Passing Through Channels (Part 2)
// --------------------------------------------

/*
*/

use std::{clone, sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let x = "some_value".to_string();
        println!("Sending value {x}");
        thread::sleep(Duration::from_secs(3)); // Make rx blocked.
        tx.send(x).unwrap();
    });
    // let recieved_val = rx.recv().unwrap();      // Be blocked from waiting value.
    // println!("I am Blocked.");

    // Loop until message is recieved.
    let mut recieved_status = false;
    while recieved_status != true {
        match rx.try_recv() {
            Ok(recieved_value) => {
                println!("Recieved value is: {recieved_value}");
                recieved_status = true;
            }
            Err(_) => println!("I am doing some other stuff."),
        }
    }
}
