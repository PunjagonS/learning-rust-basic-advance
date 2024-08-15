// --------------------------------------------
//          Box Smart Pointer
// --------------------------------------------

/*
        Simple Pointer            ||      Smart Pointers
    ----------------------------------------------------------------
    - Just stores memory address  ||  - Special capabilities
    - Indicated by &              ||  - Not just simple references
    - Also called references      ||
    - No special capabilities     ||
 */

/*
    Compiler go through each of its variants of Conveyance type
    to determined which variant needs the most space that is 'i32'.

    Final variant, Walk does not take any memory, 
    so it will be auto assign to most memory used, 'i32'.
*/
enum Conveyance {
    Car(i32),
    Train(i32),
    Airplane(i32),
    Walk
    // Walk(i32),           // Compiler perspective.
}

 #[derive(Debug)]
 enum List {
    /*
        Error List recursive itself make compiler confused that
        how much space to be allocated for a variable list at compile time.
    */
    // Cons(i32, List),

    /*
        Solution: put the cons variant behide some type of pointer.
        Wrap List with Box Smart Pointer to store the value of List to heap memory,
        A size of box equal to the size of a simple pointer which is of fixed size.
    */
    Cons(i32, Box<List>),
    Nil,
 }

fn main() {
    let x = 0.625;             
    let y = Box::new(x);                                                // Pointing to some heap memory containing the value.
    let z = &x;                                                             // Pointing to some memory location on the stack.

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));

    println!("{:?}", list);
}