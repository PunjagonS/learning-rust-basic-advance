// --------------------------------------------
//             Repeating Patterns
// --------------------------------------------

macro_rules! string_concat {
    () => {
        String::new()
    };

    ($some_string:expr) => {{
        let mut temp_str = String::new();
        temp_str.push_str($some_string);
        temp_str
    }};

    /*
        Bad solution because we tend to duplicate function
        like this if we have more than 2 arguments to concat.
    */
    // ($some_s1: expr, $some_s2: expr) => {{
    //     let mut temp_str = String::new();
    //     temp_str.push_str($some_s1);
    //     temp_str.push_str($some_s2);

    //     temp_str
    // }};

    /*
        Using delimeter `,` followed by repeatitve operator
        which can be `*`, `+`, `?`

        `*` means 0 or more
        `+` means 1 or more
        `?` means 0 or 1
    */
    ($($some_s: expr), *) => {{
    // ($($some_s: expr) *) => {{   // In case of no delimeter
        let mut temp_str = String::new();
        $(
            temp_str.push_str($some_s);
        )*

        temp_str
    }};
}

macro_rules! vec_mac {
    ($($element: expr), *) => {{
        let mut some_vec = Vec::new();
        $(some_vec.push($element);)*

        some_vec
    }};
}

fn main() {
    let str_null = string_concat!();
    let str_one = string_concat!("First");
    let str_double = string_concat!("First", "Second");
    // let str_double = string_concat!("First" "Second"); // In case of no delimeter
    let string_vec = vec_mac!("Nouman", "Ahmad", "Khan");
}
