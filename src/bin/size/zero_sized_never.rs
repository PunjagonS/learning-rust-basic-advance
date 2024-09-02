// --------------------------------------------
//       Zero Sized Types (`!` Never Type)
// --------------------------------------------

/*
    `!` is never type, it represents computation that
    never resolves to any value.

    Functions returning `!` are called diverging functions.
    They can only panic, exit the program or result in
    an infinite loop.

    `!` nevver types means no value, and no value cannot be assigned
    to something which needs a value.
*/

#![feature(never_type)] // Indicate the compiler we want to use never type (not support on stable version).
fn unrecoverable_state() -> ! {
    panic!("This function will not return normally with something valid");
}

/*
    Another use case of the never type is that it can be used
    to permit us to designate particular states as unreachable
    at the type level.
*/
// fn function() -> Result<i32, String> {} // Normal form return string if error.
// fn function_1() -> Result<i32, !> {} // This function promises to be never error. (if error happen program will panic or exit)
// fn function_2() -> Result<!, i32> {} // This function promises to be never success. (if success program will panic or exit)

// Own custom never type.
enum NeverType {}

fn main() {
    /*
        The first use case of the never type is when we
        want to signify that the function will not return normally.
    */

    // unrecoverable_state();
    // let x= !; // Cannot directly assign `!` to variable.
    // let x: ! = unrecoverable_state(); // This will not compile time error.
    // let x: !; // Can create a variable of never type.
    let x = panic!();

    /*
        `!` never types can be used in match arms that are guaranteed
        to be unreachable.

        This code below works because never type can be coerced to any other type.
    */
    let x = match "123".parse::<i32>() {
        Ok(x) => x,
        Err(_) => panic!(),
    };

    // let x: ! = return;
    let x: String = return;
    let counter = 0;

    /*
        In this case never type is treated as equivalentto unit.
        All standard documentation considered the break, continue, return,
        to result in a never type.
    */
    let result = loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    };
}
