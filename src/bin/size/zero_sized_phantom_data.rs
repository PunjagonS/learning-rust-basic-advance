// --------------------------------------------
//       Zero Sized Types (Phantom Data)
// --------------------------------------------

/*
    Phantom data is a marker struct with zero size.
    It helps in expressing the relationships and
    constraints between types without introducing
    any runtime overhead.
*/

/*
    Earlier version using external crate negative_impl
    to opt out of the auto marker traits of send and sync
    for this struct. With this way we can reduce size of structs.
*/
// use negative_impl::negative_impl;
// struct ABC;
// #[negative_impl]
// impl !Send for ABC {}
// #[negative_impl]
// impl !Sync for ABC {}

use std::{marker::PhantomData, mem::size_of, rc::Rc};

struct ABC {
    /*
        Way 2: using Rc type that are neither Send nor Sync.
        This situation is not preferable, because it increases
        the size every struct instance and introduces the need
        to create an Rc smart pointer whenenver an instance of
        the struct is created.
    */
    // ensuring_no_send_sync: Rc<()>,               // Size is 8 bytes.

    /*
        Wrapping Rc with phantom data. Adding a phantom data field 
        to the type tell the compiler that the type acts as though
        it stores a value of certain type.

        This is super helpful in ensuring certain properties of type,
        not causes any execution time overhead. It's a zero sized type
        used for compile time type checking and optimizations, not impact
        runtime performance.
    */
    ensuring_no_send_sync: PhantomData<Rc<()>>,     // Size is 0 byte.
}

fn main() {
    println!("{}", size_of::<ABC>());
}