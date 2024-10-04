// --------------------------------------------
//       String Concatenate and Ownership
// --------------------------------------------

/*
    Using Plus Operator `+` for concatenate string is only available
    for `String` with `String Slice &str` type.
*/

fn main() {
    let s1 = "Hello".to_string();
    let s2 = "World";
    let s3 = "!";
    let s4 = s1 + s2 + s3;
    // println!("{}", s1); // Error: s1 transferred ownership to s3
    println!("{}", s4);

    let s1 = "Hello".to_string();
    let s2 = "World".to_string();
    let s3 = "!".to_string();
    let s4 = s1 + &s2 + &s3;
    println!("{}", s4);
    println!("{} {} {}", s4, s2, s3);
}
