// --------------------------------------------
//   Message Passing Through Channels (Part 1)
// --------------------------------------------

/*
    The threads need some mechanism to communicate to
    solve out complex problems. 
    There 2 stategies: 1. message passing 2. sharing states

    Message Passing using channels concept consist of
    transmitter and reciever.
*/

use std::{clone, sync::mpsc, thread};

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {                 // "move" needed for take ownership of tx.
        let mut i = "5".to_string();
        println!("Sending value {i}");
        tx.send(i).unwrap();
        // println!("Val is: {i}");            // i is dropped after send to other thread (except primitives).    
    });

    /*
        "recv()" this function blocks the thread caling it
        from dropped before recieved the value from "tx" thread.
        This will prevent panic in case of error.
    */
    let recieved_val = rx.recv().unwrap();
    println!("Recieved {recieved_val}");

    // Example multiple thread sending out messages to recieving thread.
    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            println!("Sending value {i}");
            tx_clone.send(i).unwrap();
        });
    }

    // Drop tx to prevent program keep running.
    drop(tx);

    // Work only for single message.
    // let recieved_val = rx.recv().unwrap();
    // println!("Recieved {recieved_val}");

    // For recieved multiple values.
    for message in rx {
        println!("Recieved {message}")
    }
}