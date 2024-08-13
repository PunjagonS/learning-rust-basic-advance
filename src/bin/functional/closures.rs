// --------------------------------------------
//          Closures
// --------------------------------------------

/*
    Closure are anonymous functions which we can store in
    variables or pass as an argruments to other functions.
*/

struct User {
    name: String,
    age: u8,
    salary: u32,
}

fn validate_user(name: &str) -> bool {
    name.len() != 0
}

fn main() {
    let person_1 = User {
        name: "Someone".to_string(),
        age: 35,
        salary: 40_000,
    };

    println!("User valididy {}", validate_user(&person_1.name));
}