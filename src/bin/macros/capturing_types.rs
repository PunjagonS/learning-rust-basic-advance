// --------------------------------------------
//              Capturing Types
// --------------------------------------------

/*
    The idea behind macros are to make the code more readable
    by extracting out some of the unwanted detailed out of it.

    The macros does not take ownership of anything. We have to
    keep an eye on the expansion only. means that the macros has
    nothing to do with the ownership of the variables, and the
    variables retain their ownership as long as we do not change
    the ownership in the expansion of the code.
*/

macro_rules! input {
    /*
        `ty` is a type capture which can be any rust data type.
        `n` at the last line can be any type that has been matched by
        type capture `ty`.
    */
    ($t: ty) => {
        // let mut n = String::new();
        // std::io::stdin()
        //     .read_line(&mut n)
        //     .expect("Failed to read input");

        // let n: $t = n.trim().parse().expect("Invalid input");
        // n

        /*
            The error occurs above because the `let` statement is not allowed directly within the macro body.
            To fix this, you can wrap the macro body in a extra brackets {}.

            This change ensures that the let statement is within a block, which is valid syntax.

            cmd `cargo expand --bin capturing_types` for inspecting macros.
        */
        {
            let mut n = String::new();
            std::io::stdin()
                .read_line(&mut n)
                .expect("Failed to read input");

            let n: $t = n.trim().parse().expect("Invalid input");
            n
        }
    };
}

macro_rules! add_As {
    ($a: expr, $b: expr, $typ: ty) => {
        $a as $typ + $b as $typ
    };
}

macro_rules! some_macro {
    () => {
        let mut x = 4;
    };
}

/*
    Using another type of capture called `identifiers`
    `identifiers` are something in our program that has
    some NAME ASSOCIATED with them, such as variables,
    functions etc.
*/
macro_rules! some_macro2 {
    ($var: ident) => {
        $var = $var + 1;
    };
}

/*
    Define macros for creating functions for us.

*/
macro_rules! create_func {
    ($func_name:ident, $input: ident, $type_input: ty) => {
        fn $func_name($input: $type_input) {
            println!(
                "You called {:?}() with the input of {:?}",
                stringify!($func_name), // stringify!() converts the macro name to a string
                stringify!($input),
            );
        }
    };
}

macro_rules! create_func_with_output {
    ($func_name:ident, $input: ident, $type_input: ty, $type_output: ty) => {
        fn $func_name($input: $type_input) -> $type_output {
            println!(
                "You called {:?}() with the input of {:?}",
                stringify!($func_name), // stringify!() converts the macro name to a string
                stringify!($input),
            );
            $input
        }
    };
}

/*
    Create a function f1 with input x of type i32.
    (see full code by the macro expansion at the terminal)
*/
create_func!(f1, x, i32);
create_func_with_output!(f2, x, i32, i32);

fn main() {
    // Use case 1:
    print!("Please enter a floating point number: ");
    let some_input = input!(f32);
    println!("You entered: {}", some_input);

    // Use case 2:
    println!("{}", add_As!(15, 2.3, f32));

    // some_macro!();
    // x = x + 1;
    /*
        The macro above will be expanded to:

        fn main() {
            let mut x = 4;
            x = x + 1;
        }

        Follow to code above the compilation should not be error.
        But since the `x` is defined inside the macro world are just
        completely distinct from the variables outside the macro world.
    */

    /*
        Use case 3:
        Using `identifiers` which are name of the variables and functions
        and are substitution for their names during the macro expansion,
        in this case it is `x`.
    */
    let mut x = 4;
    some_macro2!(x);

    /*
        Use case 4:
        Calling the function f1 was created by the macro.
    */
    f1(x);
    f2(x);
}
