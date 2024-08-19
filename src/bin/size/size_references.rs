// --------------------------------------------
//     Pointers to Sized vs Unsized Types
// --------------------------------------------

use std::mem::size_of;

trait Shape {
    fn println(&self);
}

#[derive(Debug)]
struct Circle;

#[derive(Debug)]
struct Rectangle;

impl Shape for Circle {
    fn println(&self) {
        println!("{:?}", self);
    }
}

impl Shape for Rectangle {
    fn println(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    /*
        &[i32; 3]: This is a reference to an array of a fixed size.
        The reference itself is just a pointer to the array's memory,
        so its size is the size of a pointer, typically 8 bytes on a 64-bit system.
    */
    println!(
        "Size of a reference to sized type: {}",
        size_of::<&[i32; 3]>()
    );

    /*
        &[i32]: This is a reference to a slice, which is an unsized type.
        A slice reference in Rust is a "fat pointer," consisting of a pointer to
        the data and the length of the slice. This means it requires
        two pieces of information: the pointer and the length,
        making it 16 bytes on a 64-bit system.
    */
    println!(
        "Size of a reference to unsized type: {}",
        size_of::<&[i32]>()
    );

    let num_1: &[i32; 3] = &[10, 12, 30];
    let num_2: &[i32] = &[10, 12, 30];

    let mut sum = 0;
    for num in num_1 {
        sum += num;
    }

    for num in num_2 {
        sum += num;
    }

    println!("Size of &Circle is: {}", size_of::<&Circle>());
    println!("Size of &Rectangle is: {}", size_of::<&Rectangle>());
    println!("Size of &dyn Shape is: {}", size_of::<&dyn Shape>());     // Trait reference is double size of struct.
}