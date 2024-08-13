// --------------------------------------------
//          Closures
// --------------------------------------------

/*
    Closure are anonymous functions which we can store in
    variables or pass as an argruments to other functions.

    " Fn(&T) -> U " is the signature of the closures.
*/

struct User {
    name: String,
    age: u8,
    salary: u32,
}

// fn validate_user(name: &str) -> bool {
//     name.len() != 0
// }

fn is_valid_user<V1, V2>(name: &str, age: u8, simple_validator: V1, advance_validator: V2) -> bool 
where
    V1: Fn(&str) -> bool,
    V2: Fn(u8) -> bool,
{
    simple_validator(name) && advance_validator(age)
}

fn main() {
    let person_1 = User {
        name: "Someone".to_string(),
        age: 35,
        salary: 40_000,
    };

    /*
        The input to Closures is mentioned inside Vertical Pipes(|..|).
        Closures body({}) will contain the same code as that of the function.
    */
    let validate_user_simple = |name: &str| { name.len() != 0 };
    let validate_user_advance = |age: u8| { age >= 30 };

    println!("User valididy {}", is_valid_user(
        &person_1.name,
        person_1.age,
        validate_user_simple,
        validate_user_advance
    ));
}