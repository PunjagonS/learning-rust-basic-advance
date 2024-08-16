// --------------------------------------------
//          Doubly Linked List
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

    // Case: 1
    // --------------------------------
    //        Head         Tail
    // None <-- 2 --> 3 --> None
    // None     2 <-- 3     None
    // --------------------------------

    // Case: 2
    // --------------------------------
    //        Head = None
    //        Tail = None
    // --------------------------------

    fn remove(&mut self) -> Option<i32> {
        if self.head.is_none() {
            println!("List is empty so we cannot remove.");
            None
        } else {
            let removed_val = self.head.as_ref().unwrap().borrow().element;
            self.head.take()
                .map(|old_head| match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev = None;
                        self.head = Some(new_head);
                        self.head.clone()
                    }
                    None => {
                        self.tail = None;
                        println!("List is empty after removal");
                        None
                    }
                });
            Some(removed_val)
        }
    }

    fn print(&self) {
        let mut traversal = self.head.clone();
        while !traversal.is_none() {
            println!("{}", traversal.as_ref().unwrap().borrow().element);
            traversal = traversal.unwrap().borrow().next.clone();
        }
    }
}

type pointer = Option<Rc<RefCell<Node>>>;

fn main() {
    let mut list = DoublyLinklist::new();

    list.add(30);
    list.add(31);
    list.add(32);
    list.add(33);
    list.print();

    list.remove();
    println!("After Removal");
    list.print();
}