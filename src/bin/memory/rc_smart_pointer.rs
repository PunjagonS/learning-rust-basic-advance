// --------------------------------------------
//      Reference Counting Smart Pointer
// --------------------------------------------
/*
    Situations which demands to have multiple owners of a value.
    Such as we have list A,B,C and list A is being pointed by both 
    some array index of list B and list C. If we delete list A,
    then pointers in list B and list C is invalid.
*/

use std::rc::Rc;

enum List {
    Cons(i32, Option<Box<List>>)
}

enum ListRC {
    // Replace Box with RC Smart Pointer instead
    // Cons(i32, Option<Box<List>>)
    Cons(i32, Option<Rc<ListRC>>)
}

fn main() {
    let a = List::Cons(1, Some(Box::new(List::Cons(2, None))));
    let b = List::Cons(3, Some(Box::new(a)));                   // Ownership rule list A transfered ownership to list B.
    // let c = List::Cons(4, Some(Box::new(a)));                      // Ownership rule prevent list A transfered ownership to list B.    

    /*
        Resolve problem ownership transfer above with 'Rc'

        Variable A is now an Rc Smart Pointer holding list and
        the reference count(Rc) start at 1.

        At the end of the scope function, all three variables will be dropped.
        The variable are dropped in the last in first out order, meaning that
        the variable which is defined last will be deleted first(drop c -> b -> a)
        and decrease the reference count for each variable dropped.
    */
    let a = Rc::new(ListRC::Cons(1, Some(Rc::new(ListRC::Cons(2, None)))));
    println!("Reference count after a: {}", Rc::strong_count(&a));
    {
        let b = ListRC::Cons(3, Some(Rc::clone(&a)));        // After created B, the reference count(Rc) becomes 2
        println!("Reference count after a: {}", Rc::strong_count(&a));

        let c = ListRC::Cons(4, Some(Rc::clone(&a)));        // After created C, the reference count(Rc) becomes 3
        println!("Reference count after a: {}", Rc::strong_count(&a));
    }
    println!("Reference count after scope: {}", Rc::strong_count(&a));
}