// --------------------------------------------
//         Question Mark Operator (?)
// --------------------------------------------

/*
    Question Mark (?) is special type of operator, typically
    used along with Result<T, E> enum. More specifically,
    it will grap the value inside the Ok variant in case of Ok(T) variant.
    if the value is inside the Err variant, it will return the Err variant.

    It is convenient way for unwrapping the value inside the Ok variant.
*/

use std::num::ParseIntError;

fn parse_str_to_int(input: &str) -> Result<i32, ParseIntError> {
    /*
        There are a few things happen.

        First, the type of the variable integer changed from
        Result<i32, ParseIntError> to i32 as pointer out earlier
        that it is going to unwrap or grap the inner value of Ok(T) variant.

        Second, on case of the error variant, the value will not be assigned
        to the variable integer. Instead, the error will be returned to the caller.
    */
    let integer = input.parse::<i32>()?;

    println!("Parsing succeeded. {} as i32 is {}", input, integer);
    Ok(integer)
}

fn divisor(divident: f64, divisor: f64) -> Result<f64, String> {
    let answer = match divisor {
        0.0 => Err(String::from("Division by zero")),
        _ => Ok(divident / divisor), // (_) is a wildcard pattern that matches any value.
    };
    // Ok(answer?)

    let correct = answer?;
    /*
        (?) will take the ownership of the value in connection with the result enum
        cause this code below:

        println!("{:?}", answer);

        error: borrow of moved value: `answer`
    */

    println!("This line will not print in case of error {:?}", correct);
    Ok(correct)
}

// Using Option<T> enum
fn divisor_v2(divident: f64, divisor: f64) -> Option<f64> {
    let answer = match divisor {
        0.0 => None,
        _ => Some(divident / divisor),
    };

    let correct = answer?;
    println!("This line will not print in case of error {:?}", correct);
    Some(correct)
}

/*
    Example: Using multiple question mark operator inside a single function.

    This will be useful typically when we want to test a series of functions
    in sequential order, one after the other.
*/

#[derive(Debug)]
enum MathError {
    DivisionError_DivisionByZero,
    LogError_NonPositiveLogarithm,
    SqrtError_NegativeSquareRoot,
}

type MathResult = Result<(), MathError>;

fn division(x: f64, y: f64) -> MathResult {
    if y == 0.0 {
        Err(MathError::DivisionError_DivisionByZero)
    } else {
        println!("The division was successful and the result is {}", x / y);
        Ok(())
    }
}

fn sqrt(x: f64) -> MathResult {
    if x < 0.0 {
        Err(MathError::SqrtError_NegativeSquareRoot)
    } else {
        println!(
            "The square root is successful and has a result of {}",
            x.sqrt()
        );
        Ok(())
    }
}

fn ln(x: f64) -> MathResult {
    if x <= 0.0 {
        Err(MathError::LogError_NonPositiveLogarithm)
    } else {
        println!("The logarithm is successful and has a result of {}", x.ln());
        Ok(())
    }
}

// operations will call 3 sub functions as series.
fn operations(x: f64, y: f64) -> MathResult {
    division(x, y)?;
    sqrt(x)?;
    ln(x)?;
    Ok(())
}

fn main() {
    let random_values = vec!["123", "some1", "some(123)", "abc", "49", "5555"];
    for value in random_values {
        println!("{:?}", parse_str_to_int(value));
    }

    // Using Result<T, E> enum
    println!(
        "Call from main with result equals to {:?}",
        divisor(10.0, 2.0)
    );
    println!(
        "Call from main with result equals to {:?}",
        divisor(4.0, 0.0)
    );
    println!(
        "Call from main with result equals to {:?}",
        divisor(0.0, 3.0)
    );

    // Using Option<T> enum
    println!(
        "Call from main with result equals to {:?}",
        divisor_v2(10.0, 2.0)
    );
    println!(
        "Call from main with result equals to {:?}",
        divisor_v2(4.0, 0.0)
    );
    println!(
        "Call from main with result equals to {:?}",
        divisor_v2(0.0, 3.0)
    );

    // Using multiple question mark operator inside a single function
    let result = operations(0.0, 10.0); // Call multiple functions in series.
    if result.is_ok() {
        println!("The operations were successful");
    } else {
        println!("{:?}", result);
    }
}
