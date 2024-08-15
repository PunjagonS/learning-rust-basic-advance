// --------------------------------------------
//          Lifetime Elision
// --------------------------------------------

/*
    Lifetime Elision is a feature in rust that allows the compiler
    to automatically infer the lifetimes of the reference in functions
    and method signatures, making it more concise and readable.

    The rust compiler follows 3 lifetime elision rules.
    1. Each parameter that is reference, gets its own lifetime parameter.
    2. If there is exactly one input lifetime parameter, that lifetime is 
        assigned to all output lifetime parameters.
    3. If there are multiple input lifetime parameters, but one of them is 
        '&self' or '&mut' self, the lifetime of self is assigned to all 
        output lifetime parameters.

    After applying the 3 rules, if lifetimes are still ambiguous,
    an error will be thrown, requiring an explixit lifetime annotaions.
*/

fn main() {
    let str_1 = "some str";
    let str_2 = "other str";
    let recieved_str = return_str(&str_1);
    let recieved_str_2 = return_str_4(&str_1, &str_2);
    println!("{recieved_str}");
    println!("{recieved_str_2}");
}

/*
    The original code with Lifetime Elision.
    When we have only a single input lifetime parameter 
    in the function signature, we do not need to explicitly
    annotate the lifetimes.
*/
fn return_str(s_1: &str) -> &str {
    s_1
}

/*
    Actual code compiler resolved form apply 1st rule.
    1. Each parameter that is reference, gets its own lifetime parameter.
*/
fn return_str_2<'a>(s_1: &'a str) -> &str {
    s_1
}

/*
    Actual code compiler resolved form apply 2nd rule.
    2. If there is exactly one input lifetime parameter, 
        that lifetime is assigned to all output lifetime parameters.
*/
fn return_str_3<'a>(s_1: &'a str) -> &'a str {
    s_1
}


/*
    Case: multiple input parameters. Actual code compiler resolved form apply 3rd rule..
    3. If there are multiple input lifetime parameters, but one of them is 
        '&self' or '&mut' self, the lifetime of self is assigned to all 
        output lifetime parameters.

    Error since the compiler cannot determined the lifetime using elision rule, 
    which lifetime of either 1st or 2nd parameter should be returned.

    " fn return_str_4<'a, 'b>(s_1: &'a str, s_2: &'b str) -> &str "

    We have to explicitly add the lifetime annotations.
*/ 
fn return_str_4<'a, 'b>(s_1: &'a str, s_2: &'b str) -> &'a str {
    s_1
}