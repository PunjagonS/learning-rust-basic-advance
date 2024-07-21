// --------------------------------------------
//      - Scalar Data Types
//          - Integers
//          - Floats
//          - Char
//          - Boolean
// --------------------------------------------

fn main() {
    // Unsigned integers (positive numbers only)
    let unsigned_num: u8 = 5; // u16, u32, u64, u128

    // Signed integers
    let signed_num: i8 = -5; // i16, i32, i64, i128

    // Floating point numbers
    let float_num: f32 = 3.14; // f64 (default)

    // Platform specific integers ()
    let arch_1: isize = 5; // isize repreesents a pointer-sized with unsigned integers
    let arch_2: usize = 5; // usize repreesents a pointer-sized with signed integers

    // Characters
    let char = 'a';

    // Boolean
    let b = true;

    // Type aliasing
    type Age = u8;
    let peter_age: Age = 25;

    // Type Conversion
    let a = 5;
    let b: f64 = a as f64;
}
