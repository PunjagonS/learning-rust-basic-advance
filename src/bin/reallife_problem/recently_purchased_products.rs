/*  -----------------------------------------------------
    Most Recently Purchased Product
        Description
        - A bussiness is interested in knowing the products
          that has been purchased most recently by a customer.

        Tools
        - Hashmaps + Doubly Linked List
    -----------------------------------------------------
*/

use std::{cell::RefCell, collections::HashMap, rc::Rc};

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    element: i32,
    next: Link,
    prev: Link,
}

impl Node {
    fn new(element: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            element,
            next: None,
            prev: None,
        }))
    }
}

#[derive(Debug)]
struct DoublyLinklist {
    head: Link,
    tail: Link,
}

impl DoublyLinklist {
    fn new() -> Self {
        DoublyLinklist {
            head: None,
            tail: None,
        }
    }

    // A slightly changed version where the function returns the link now.
    pub fn push_back(&mut self, element: i32) -> Link {
        let new_tail = Node::new(element);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }

            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
        self.tail.clone()
    }

    /*
        A slightly changed version of the remove_head
        and now in this case it returns the head which is reset.
    */
    pub fn remove_front(&mut self) -> Option<Link> {
        self.head
            .take()
            // Take the next node
            .map(|old_head| match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take(); // Remove the link to the previous node
                    self.head = Some(new_head); // Update the head to the next node
                    self.head.clone() // Return the new head
                }
                None => {
                    self.tail.take(); // Remove the tail
                    None // Return None
                }
            })
    }

    fn move_to_tail(&mut self, node: &Rc<RefCell<Node>>) {
        let prev = node.borrow().prev.as_ref().map(|a| Rc::clone(a));
        let next = node.borrow().next.as_ref().map(|a| Rc::clone(a));

        match (prev, next) {
            (None, None) => {}
            (Some(_), None) => {}
            (None, Some(next)) => {
                node.borrow_mut().next = None;
                node.borrow_mut().prev = None;
                self.head = Some(next.clone());

                let prev_tail = self.tail.as_ref().unwrap();
                prev_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(prev_tail.clone());
                self.tail = Some(node.clone());
            }
            (Some(prev), Some(next)) => {
                node.borrow_mut().next = None;
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev.clone());

                let prev_tail = self.tail.as_ref().unwrap();
                prev_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(prev_tail.clone());
                self.tail = Some(node.clone());
            }
        }
    }
}

#[derive(Debug)]
struct MRP_Item {
    item_list: DoublyLinklist,
    map: HashMap<i32, Rc<RefCell<Node>>>,
    capacity: usize,
    size: usize,
}

impl MRP_Item {
    fn new(capacity: usize) -> Self {
        MRP_Item {
            item_list: DoublyLinklist::new(),
            map: HashMap::new(),
            capacity,
            size: 0,
        }
    }

    fn purchased(&mut self, prod_id: i32) {
        if let Some(node) = self.map.get(&prod_id) {
            self.item_list.move_to_tail(node);
        } else {
            if self.size >= self.capacity {
                let prev_head = self.item_list.remove_front().unwrap();
                self.map.remove(&prev_head.unwrap().borrow().element);
            }
            let node = self.item_list.push_back(prod_id).unwrap();
            self.map.insert(prod_id, node);
            self.size += 1;
        }
    }

    fn print(&self) {
        let mut traversal = self.item_list.head.clone();
        while !traversal.is_none() {
            let temp = traversal.clone().unwrap();
            print!("{} ", temp.borrow().element);
            traversal = temp.borrow().next.clone();
        }
        println!("");
    }
}

fn main() {
    let mut item_list = MRP_Item::new(4);
    item_list.purchased(10);
    item_list.print();

    item_list.purchased(15);
    item_list.print();

    item_list.purchased(20);
    item_list.print();

    item_list.purchased(25);
    item_list.print();

    item_list.purchased(69);
    item_list.print();
}
