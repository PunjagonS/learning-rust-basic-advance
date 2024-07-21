// --------------------------------------------
//          Ownership Basics
// --------------------------------------------

/*
1. Each value has a variable that's its "owner".
2, A value can have only one owner at a time.
3. If the owner goes out of scope, the value will be dropped.
 */

fn main() {
    let s1 = String::from("world");
    let s2 = s1;
    //println!("s1 is :{}", s1); // Error: value borrowed here after move to s2 (s1 -> s2)

    let s3 = s2.clone();
    println!("s3 is :{}", s3); // No error

    {
        let s4 = 5; // s4 is live within this scope
    }
    //println!("s4 is :{}", s4); // Error: s4 not found in this scope

    /*
    Ownship and borrow rule not applicable to primitive types
    nteger, float, boolean, character are entirely stored on the stack
    with no reference to heap memory so they are copied and not moved by default
     */
    let x = 32;
    let y = x;
    println!("x is :{}", x); // No error
}
