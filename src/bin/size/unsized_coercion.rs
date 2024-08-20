// --------------------------------------------
//             Unsized Coercion
// --------------------------------------------

/*
    Unsized Coercion occurs when a sized type is transformed into 
    an unsized type. Similar to Deref Coercion.

    The Deref Coercion occurs for automatic conversions of a reference
    to a type into a reference  to another type, specifically when
    using methods and functions that expect a certain type. It occurs when 
    a type gets coerced into another type following a deref operation.
*/

use core::slice;

fn str_slice_fn(s: &str) {}


/*
    Functions and methods are now more flexible,
    enhances the usability of code, and avoids
    unnessary duplication of the code.
*/
fn array_slice_fn<T>(s: &[T]) {}        // &[T] require ref input && unknown size.
trait Some_Trait {
    fn method(&self);
}

impl<T> Some_Trait for [T] {            // &[T] require ref input && unknown size.
    fn method(&self) {}
    /*
        Can now call "method" on
        1) any &[T]
        2) Vec<T>
        3) [T; N]
    */
}

fn main() {
    /*
        Deref Coercion example:
        string type coerced into string slice.
    */
    let some_strig = String::from("String");
    str_slice_fn(&some_strig);

    // Unsized Coercion example:
    let slice = &[1];
    let vec = vec![1];
    let array = [1,2,3];      // Known size.

    array_slice_fn(slice);              // Meet function required.
    array_slice_fn(&vec);               // Deref Coercion.
    array_slice_fn(&array);             // Unsized Coercion(coercion from sized to unsized).

    // Using: trait instead.
    slice.method();                     // Meet function required.
    vec.method();                       // Deref Coercion.
    array.method();                     // Unsized Coercion
}