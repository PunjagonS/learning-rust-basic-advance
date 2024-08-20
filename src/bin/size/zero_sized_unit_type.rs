// --------------------------------------------
//          Zero Sized Types (Unit)
// --------------------------------------------

/*
    The unit type represents a lack of meaningful data 
    or infomation and is denoted by parentheses "()". This has 
    only one possible value, called a unit value.

    Unit values are useful when we have some piece of code
    that does something. but do not return anything meaningful.

    Function which do not return anything explicitly will
    return a unit value.
*/

use std::panic::AssertUnwindSafe;

// fn f1() {}
fn f1()-> () {}         // Desugared form.

fn division_status(divident: f64, divisor: f64) -> Result<(), String> {
    let answer = match  divisor {
        0.0 => Err("Error: Division by zero".to_string()),
        _ => {
            println!("The division is invalid");
            Ok(())
        }
    };
    answer
}

fn main() {
    let x = ();     // x is varible of unit type with unit value.
    let y = f1();   // y store the unit value return from f1().

    let z = println!("Hello, world!");

    /*
        The compiler recognizes that unit type has no size and
        optimizes its interaction with instances of unit type.
        Therefore, pushing of unit values will only update the length 
        of the vector and will not lead to any heap allocations or
        change in the capacity of the vector.
    */
    let mut vec = Vec::with_capacity(0);   // Create vector of unit (zero-sized).
    vec.push(());
    vec.push(());
    vec.push(());
    assert_eq!(3, vec.len());

    println!("{}", vec.capacity());
    /*
        The key reason "vec.capacity = 18446744073709551615(usize::MAX)" is that () is a zero-sized type, 
        meaning that its size is 0 bytes. The capacity function typically returns the number of elements 
        the vector can hold without reallocating. However, when dealing with zero-sized types, 
        the concept of "capacity" becomes meaningless because the vector can theoretically hold 
        an infinite number of () elements without requiring any memory.


        Unit Type                   ||  Never Type
        1. No meaningful value      ||  1. Never produces a value
        2. Function returning unit  ||  2. Function returning never, will never
        always returns normally     ||  returns normally
        3. Single value, which      ||  3. No associated value, and can be
        can not be coerced          ||  coerced to all types.
    */
}