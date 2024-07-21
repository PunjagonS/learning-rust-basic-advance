// --------------------------------------------
//          - Functions
//          - Code Blocks
// --------------------------------------------

fn main() {
    my_fn("Hello, World!");

    let str = "Function call with a variable";
    my_fn(str);

    let answer = multiplication(10, 20);

    let result = basic_math(10, 20);
    let (multiplication, addition, subtraction, division) = basic_math(40, 50);

    // Code block
    let fullname = {
        let first_name = "Nouman";
        let last_name = "Azam";
        format!("{first_name} {last_name}")
    };
}

fn my_fn(s: &str) {
    println!("{s}");
}

fn multiplication(num1: i32, num2: i32) -> i32 {
    println!("Computing multiplication");
    num1 + num2 // Return statement without semicolon
}

fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32, i32) {
    let sum = num1 + num2;
    let diff = num1 - num2;
    let prod = num1 * num2;
    let div = num1 / num2;

    (sum, diff, prod, div) // Return statement without semicolon
}
