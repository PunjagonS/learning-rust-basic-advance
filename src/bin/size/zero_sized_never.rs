// --------------------------------------------
//          Zero Sized Types (Never)
// --------------------------------------------

/*
    Never type represents computation that 
    never resolves to any value. In others words,
    it corressponds to computation that will always
    panic or exit the program.

    The first use case of the never type is when we
    want to signify that the function will not return normally.
*/

// Indicate the compiler we want to use never type.
// Not support on stable version
// #![feature(never_type)]

fn main() {
    
}