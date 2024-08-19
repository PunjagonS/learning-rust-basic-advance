// --------------------------------------------
//            Size in Rust
// --------------------------------------------

/*
    - Sized Types is its size can be determined at compile time.

    - Unsized Types also called "Dynamic Resize Type"
        is its size cannot be determined at compile time.
*/

use std::mem::size_of;

trait SomeTrait {}

fn main() {
    println!("i32 size is: {}", size_of::<i32>());
    println!("(i32,i32) size is: {}", size_of::<(i32, i32)>());
    println!("[i32: 3] size is: {}", size_of::<[i32; 3]>());

    struct Point {
        x: bool,
        y: i64,
    }
    println!("Struct size is: {}", size_of::<Point>());
    println!("i32 reference is: {}", size_of::<&i32>());
    println!("i32 muatble reference is: {}", size_of::<&mut i32>());
    println!("Box<i32> is: {}", size_of::<Box<i32>>());
    println!("fn(i32) -> i32 is: {}", size_of::<fn(i32) -> i32>());

    // Unsized Types
    // println!("[i32] size is: {}", size_of::<[i32]>());                   // Error: compiler cannot determine size of slice at compile time.
    println!("[i32] pointer size is: {}", size_of::<&[i32]>());             // Determine size of pointer instead.
    // println!("str size is: {}", size_of::<str>());                       // Error: compiler cannot determine size of slice at compile time.
    // let a: [i32];                                                        // Error: compiler cannot determine size of slice at compile time.
    let a: [i32; 3];                                                        // Fix slice size at compile time.

    // println!("The trait object size is: {}", size_of::<dyn SomeTrait>()) // Error: compiler cannot determine size of slice at compile time.
    println!("The trait object pointer size is: {}", size_of::<&dyn SomeTrait>())   // Determine size of pointer instead.
}