// --------------------------------------------
//          Things must not do on Rust
// --------------------------------------------

/*
    1: `impl Into<String>`
    - `impl Into<String>` is a trait that allows a type to be converted into a `String`.
    - However, it introduces an overhead of calling the `.into()` method on the returned value.

    2: `use Enum::*;`
    - `use Enum::*;` is a shorthand for importing all variants of an enum.
    - However, it can lead to confusion, runtime error and lack of maintenance
    to aware the enum is changed.

    3: `&Option<T>` -> No
       `Option<&T>` -> Yes

    All of these below are the way more api flexible
    4: `&Vec<T>` -> No
       `&[T]` -> Yes

    5. `&String` -> No
       `&str` -> Yes

    6. `&Box<T>` -> No
       `&T` -> Yes

    All of these below are the way more shorter
    7. `vector<T> const&` -> No
       `span<const T>` -> Yes

    8. `string const&` -> No
       `string_view` -> Yes

    9. `unique_ptr<T> const&` -> No
       `T*` -> Yes
*/

// Ex 1: Must not do this
// This fn can return anything that implement from `Into<String>`
fn greet() -> impl Into<String> {
    "hello"
}
// Ex 1: Use this instead
fn greet_improved() -> String {
    "hello".into()
}

// Ex 1: Must not do this
fn a_nice_api(s: impl Into<String>) {
    s.into()
        .trim()
        .to_uppercase()
        .replace("OLD", "NEW")
        .chars()
        .rev()
        .collect::<String>();
}
// Ex 1: Use this instead
fn a_nice_api_improved(s: &str) {
    s.trim()
        .to_uppercase()
        .replace("OLD", "NEW")
        .chars()
        .rev()
        .collect::<String>();
}

// Ex 2: Must not do this
use basic_advance::modules::enums::MyLongEnum::*;
// Ex 2: Use this instead
use basic_advance::modules::enums::MyLongEnum as E;

fn main() {
    // Ex 1: cause overhead `.into()`
    /////////////////////////////////////////////////////////////////////////////////////////////////
    let greet_length = greet().into().len();
    let greet_message = greet().into();
    if greet().into() == "hello" {
        println!("greet is hello");
    }

    // After improvement no overhead of `.into()`
    let greet_length = greet_improved().len();
    let greet_message = greet_improved();
    if greet_improved() == "hello" {
        println!("greet is hello");
    }
    /////////////////////////////////////////////////////////////////////////////////////////////////

    // Ex 2: cause error move to runtime and hard to investigate
    /////////////////////////////////////////////////////////////////////////////////////////////////
    let my_enum = A;
    match my_enum {
        A => println!("A"),
        B => println!("B"),
        /*
            C is be removed but still compile no error
            because no one would know this code is not valid
            until runtime error.
        */
        C => println!("C"),
    }

    // After improvement
    let my_enum = E::B;
    match my_enum {
        E::A => println!("A"),
        E::B => println!("B"),
        // E::C => println!("C"), // Compiler warning no variant C
    }
    /////////////////////////////////////////////////////////////////////////////////////////////////
}
