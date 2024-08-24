// --------------------------------------------
//            Declarative Macros
// --------------------------------------------

/*
    Macros provide a way of writing code that generates
    other code, which is known as metaprogramming.

    Macros are divided into 2 types:
    1. Declarative Macros
    2. Procedural Macros, which consist of custom derive, attribute-like, and function-like macros.
*/

// Basic syntax of macros using pattern matching:
// macro_rules! macro_name {
//     (....) => {....};
//     (....) => {....};
//     (....) => {....};
// }

macro_rules! our_macro {
    () => {
        1 + 1
    };
    // Using matching expression for input
    (somthing 4 u dear u32 @_@) => {
        println!("You found nonesense here");
    };
    // Using 2 matching expressions for inputs
    ($e1: expr, $e2: expr) => {
        $e1 + $e2
    };
    // Using 3 matching expressions for inputs
    //($e1: expr, $e2: expr, $e3: expr) => {
    ($e1: expr, $e2: expr; $e3: expr) => {
        $e1 * ($e2 + $e3)
    }; // Semicolon is optional
}

fn main() {
    our_macro!();
    println!("{}", our_macro!());

    our_macro!(somthing 4 u dear u32 @_@);

    println!("{}", our_macro!(2, 2));

    // println!("{}", our_macro!(5, 6, 3));
    println!("{}", our_macro!(5, 6; 3));
    // println!("{}", our_macro!("something", 2; "nothing"));   // Error invalid input

    // Option to call marcros with 3 types brackets
    our_macro!(5, 6; 3); // ()
    our_macro![5, 6; 3]; // []
    our_macro! {5, 6; 3}; // {}
}
