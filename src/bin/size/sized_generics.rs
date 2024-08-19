// --------------------------------------------
//         ?Sized and Generic Parameters
// --------------------------------------------

/*
    1. Must have a single unsized field.
    2. The unsized field must be the last field.
*/

use std::fmt::Debug;

struct UnSizedStruct {
    sized_field_1: i32,
    unsized_field: [i32],
    // unsized_field_2: [i32],         // Break 1nd rule.
    // sized_field_2: i32,             // Break 2nd rule.
}

struct UnSizedStruct2<T: ?Sized> {
    sized_field_1: i32,
    unsized_field: T,
}

// fn print_fn<T: Debug> (t: T) {           // Sugared
fn print_fn<T: Debug + Sized> (t: &T) {     // Desugared
    println!("{:?}", t);
}

fn print_fn2<T: Debug + ?Sized> (t: &T) {    // Using option sized trait.
    println!("{:?}", t);
}

fn main() {
    // let x = UnSizedStruct {
    //     sized_field_1: 3,
    //     unsized_field: [3],
    // };

    let x = UnSizedStruct2 {
        sized_field_1: 3,
        unsized_field: [3],
    };

    let x = "my name";  // Slice string is copy type.

    /*
        Parameter Type:         T           &T          &T
        Passed Input Type:      &str        &str        &&str
        Resolve to:             T = &str    T = str     T = &str  
    */
    // print_fn(x);             // Resolve to T = str
    print_fn(&x);               // Resolve to T = &str 

    /*
        Using: option sized trait, let compiler know size of value
        at runtime instead.
    */
    print_fn2(x);
}