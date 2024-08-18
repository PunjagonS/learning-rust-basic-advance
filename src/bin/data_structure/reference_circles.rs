// --------------------------------------------
//          Reference Circles
// --------------------------------------------

/*
    Example using smart pointer as "Reference Circles"
    cause of memory leak. it is be like Node 1 -> Node 2 -> Node 3
                                          ^_____________________|
                                              Node 3 -> Node 1

    Solution: Using "Weak Rc smart pointer".
    The weak is a special type of RC smart pointer with
    2 key functions called upgrade and downgrade.

    - The Upgrade will attempt to convert it into an RC,
    thereby increasing the strong count by 1.
    - The Downgrade creates a new weak pointer to the
    allocation.

    The new pointer will holds a Non-owning reference to
    the managed allocation means that it does not provide
    a share ownership of the underlying data. Moreover, it
    increases the weak count by one and does not change
    the strong count.

    In summary, the strong references are how you can share
    ownership of a reference counting smart pointer instance.
*/

use std::{cell::RefCell, rc::Rc};
use std::rc::Weak;

#[derive(Debug)]
struct Node {
    next: pointer,
}
type pointer = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node2 {
    next: pointer2,
}
type pointer2 = Option<Weak<RefCell<Node2>>>;

#[derive(Debug)]
struct Node3 {
    value: i32,
    parent: RefCell<Weak<Node3>>,
    children: RefCell<Vec<Rc<Node3>>>,
}

impl Drop for Node {
    /*
        Override standard function drop when Node is out of scope.
        A smart pointer in general will only be dropped if there is 
        a single reference to it(strong_count has a value of 1).

        The drop function is being called in "Reverse Order(LIFO)".
    */
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

impl Drop for Node2 {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    let a = Rc::new(RefCell::new(Node 
        { 
            next: None 
        }));
    println!("a count: {:?}", Rc::strong_count(&a));

    // a -> b
    let b = Rc::new(RefCell::new(Node 
        {
            next: Some(Rc::clone(&a)) 
        }));
    println!("B is created \n a count: {:?}", Rc::strong_count(&a));
    println!("b count: {:?}", Rc::strong_count(&b));

    // a -> b -> c
    let c = Rc::new(RefCell::new(Node 
        {
            next: Some(Rc::clone(&b)) 
        }));

    /* 
        This line create reference circle.

        a -> b -> c
        ^_________|

        Then Node drop function cannot be called.
    */
    // (*a).borrow_mut().next = Some(Rc::clone(&c));

    println!("After creating cycle: \n a count: {:?}", Rc::strong_count(&a));
    println!("b count: {:?}", Rc::strong_count(&b));
    println!("c count: {:?}", Rc::strong_count(&c));

    // Error stack overflow from print 'a' cause of reference circles.
    println!("a {:?}", a);


    ///////////////////////////////////////////////////////////////////////////////////////////////////////////
    ///////////////////////////////////////////////////////////////////////////////////////////////////////////

    
    // Using Weak Rc smart pointer.
    println!();
    println!("Start using Weak Rc smart pointer.");
    let a = Rc::new(RefCell::new(Node2
        { 
            next: None 
        }));
    println!("a count: {:?}, a weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));

    let b = Rc::new(RefCell::new(Node2 
        {
            next: Some(Rc::downgrade(&a)) 
        }));
    println!("B is created \n a count: {:?}, a weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("b count: {:?}, b weak count: {:?}", Rc::strong_count(&b), Rc::weak_count(&b));

    let c = Rc::new(RefCell::new(Node2 
        {
            next: Some(Rc::downgrade(&b)) 
        }));

    (*a).borrow_mut().next = Some(Rc::downgrade(&c));

    println!("After creating cycle: \n a strong count: {:?}, a weak count: {:?}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("b strong count: {:?}, b weak count {:?}", Rc::strong_count(&b), Rc::weak_count(&b));
    println!("c strong count: {:?}, c weak count {:?}", Rc::strong_count(&c), Rc::weak_count(&c));

    println!("a {:?}", a);


    // Example using weak rc to track parents in a tree data structure.
    let leaf = Rc::new(Node3 {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec!()),
    });

    let branch = Rc::new(Node3 {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
}