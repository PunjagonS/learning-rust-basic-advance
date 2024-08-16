// --------------------------------------------
//          Doubly Linked List (Part 1)
// --------------------------------------------

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    element: i32,
    next: pointer,
    prev: pointer,
}

impl Node {
    fn new(element: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { 
            element: element, 
            next: None, 
            prev: None,
        }))
    }
}

#[derive(Debug)]
struct DoublyLinklist {
    head: pointer,
    tail: pointer,
}

impl DoublyLinklist {
    fn new() -> Self {
        DoublyLinklist {
            head: None,
            tail: None,
        }
    }

    fn add(&mut self, element: i32) {
        // let new_head = Rc::new(RefCell::new(Node {
        //     element: element,
        //     next: None,
        //     prev: None,
        // }));
        let new_head = Node::new(element);

        // Place new node always at the begining.
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }

            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }
}

type pointer = Option<Rc<RefCell<Node>>>;

fn main() {
    
}