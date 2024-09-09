// --------------------------------------------
//              Function Pointers
// --------------------------------------------

/*
    Function Pointers are concrete type, it works similar to closures,
    except that they don't capture the variables in their environment.

    "fn(..) -> .." is the signature of the Function Pointers.
*/

struct User {
    name: String,
    age: u8,
    salary: u32,
}

//// Convert to function pointers concrete type
// fn is_valid_user<V1, V2>(name: &str, age: u8, simple_validator: V1, advance_validator: V2) -> bool
// where
//     V1: Fn(&str) -> bool,
//     V2: Fn(u8) -> bool,
// {
//     simple_validator(name) && advance_validator(age)
// }

fn is_valid_user(
    name: &str,
    banned_user: &str,
    age: u8,
    simple_validator: fn(&str, &str) -> bool,
    advance_validator: fn(u8) -> bool,
) -> bool {
    simple_validator(name, banned_user) && advance_validator(age)
}

fn validate_user_simple(name: &str, banned_user: &str) -> bool {
    name.len() != 0 && name != banned_user
}

fn validate_user_advance(age: u8) -> bool {
    age >= 30
}

fn main() {
    let person_1 = User {
        name: "Someone".to_string(),
        age: 35,
        salary: 40_000,
    };
    let banned_user = "banned_user";

    // Replace closeures with function pointers
    // let validate_user_simple = |name: &str| name.len() != 0;
    // let validate_user_advance = |age: u8| age >= 30;

    println!(
        "User valididy {}",
        is_valid_user(
            &person_1.name,
            &banned_user,
            person_1.age,
            validate_user_simple,
            validate_user_advance
        )
    );
}
