// --------------------------------------------
//          RefCell Smart Pointer
// --------------------------------------------
/*
    RefCell Smart Pointer is to enforce borrowing 
    rule to be CHECK!! at runtime. It means that
    if you break the borrowing rule at runtime, 
    your program will panic and exit.
*/

use std::{cell::RefCell, rc::Rc};

fn main() {
    let mut x = 50;
    let x1 = &x;
    let x2 = &x;
    /*
        Error from rust borrowing rule are being violated.
        Variable x cannot be borrowed as mutable and immutable at 
        the same time.
    */
    // let x3 = &mut x;
    println!("{} {}",x1 ,x2);


    // RefCell comes into play.
    let a = RefCell::new(10);
    let b = a.borrow();
    let c = a.borrow();
    // let d = a.borrow_mut();             // No error at compile time.
    println!("{} {}", b, c);

    let aa = RefCell::new(10);
    /*
        Manual drop 'e','f' before 'g' borrow 'aa' as mut by using drop() 
        or place them be inside the scope.
    */
    {
        let e = aa.borrow();
        let f = aa.borrow();
        // drop(b);
        // drop(c);
    }

    let g = aa.borrow_mut();
    println!("a: {:?}", aa);                   // Cannot see value a cause it be borrow by g

    drop(g);
    println!("a: {:?}", aa);


    /*
        RefCell allow us to borrow mutable reference
        from immutable reference.
    */
    // let xx = 32;
    // let yy = &mut x;                         // Do not allow by default.

    let x = RefCell::new(10);
    let mut y = x.borrow_mut();
    *y = 15;                                    // Dereference(*) y to update its direct value.
    println!("x: {:?}", x);

    drop(y);
    println!("x: {:?}", x);

    // let yy = *x;                              // RefCell cannot be dereferenced.


    /*
        Example: Combine with Rc Smart Pointer
    */
    let a = Rc::new(RefCell::new("C++".to_string()));
    let b = Rc::clone(&a);
    let c = Rc::clone(&a);
    println!("{:?}", a);

    *b.borrow_mut() = String::from("Rust");
    println!("{:?}", c);
}