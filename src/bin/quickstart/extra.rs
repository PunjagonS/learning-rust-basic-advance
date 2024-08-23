// --------------------------------------------
//         Bonus
//         - Comments
//         - More on Printting
//         - Inputs
//         - Variables Convenetions
//         - Statistics
// --------------------------------------------

use std::f32::consts::PI as PI_STANDARD;
use std::io::stdin;

fn main() {
    // The cuurrent line is a comment line
    // This is a single line comment

    /* This is a
    multi-line
    comment
     */

    print!("This is a print command");
    print!("This is going to be printed on the same line");

    /* Escape sequences
    \n : Newline character.
    \t : Tab space.
    \r : Carriage return.
    \" : Double quote.
    \' : Single quote.
    \\ : Backward slash.
     */

    println!("\n Will be printed after one empty line");
    println!("\t A tab space at start");
    println!("This will be overwritten \r This text will only appear on the screen");

    println!("Prints double qoutes \", Prints backslash \\");

    println!(
        "I am doing {2} from {1} years and i {0} it",
        "like", 20, "programming"
    );

    println!(
        "{language} is a system programing language which is cool to {activity} in.",
        activity = "code",
        language = "Rust"
    );

    let mut n = String::new();
    stdin().read_line(&mut n).expect("Failed to read input.");

    let n: f64 = n.trim().parse().expect("Invalid input");
    println!("You entered: {n}");

    let _number_one: f32 = 45.5; // Skip the warning of unused variable by adding underscore
    let x: i32 = 1_000_000; // 1 million

    static WELCOME: &str = "Welcome to Rust Programming";
    // const PI: f32 = 3.14159;
    const PI: f32 = PI_STANDARD;

    let a = PI; // Constant pass by value
    let b = PI; // Constant pass by value

    let c = WELCOME; // Static pass by reference
    let d = WELCOME; // Static pass by reference
}
