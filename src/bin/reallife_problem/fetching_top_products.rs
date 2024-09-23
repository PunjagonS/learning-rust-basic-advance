/*  -----------------------------------------------------
    Fetching Top Products
        Description
        - We are given link lists corresponding to top ranked products
        in different countries. We need to combine all these link lists
        into one consolidated link list containing the ranks in an descending
        order.

        Tools
        - Linkedlist + Iterators
    -----------------------------------------------------
*/

use std::fmt::Debug;

#[derive(Debug)]
struct Linklist<T: Debug> {
    head: Pointer<T>,
}

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Pointer<T>,
}

type Pointer<T> = Option<Box<Node<T>>>; // Pointer is a type alias for Option<Box<Node<T>>>.

impl<T: Debug> Linklist<T> {
    fn new() -> Linklist<T> {
        Linklist { head: None }
    }

    fn add(&mut self, element: T) {
        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element,
            next: previous_head,
        }));
        self.head = new_head;
    }

    fn remove(&mut self) -> Option<T> {
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }

    fn printing(&self) {
        let mut list_traversal = &self.head;
        println!();

        while true {
            match list_traversal {
                Some(node) => {
                    print!("{:?} ", node.element);
                    list_traversal = &list_traversal.as_ref().unwrap().next;
                }
                None => break,
            }
        }
    }

    fn reverse(&mut self) {
        if self.head.is_none() || self.head.as_ref().unwrap().next.is_none() {
            return;
        }

        let mut prev = None;
        let mut current_node = self.head.take();
        while current_node.is_some() {
            /*
                as_mut() returns a mutable reference to the value inside the Option,
                and then we use .unwrap() to get the value inside the Box.
            */
            let next = current_node.as_mut().unwrap().next.take();
            current_node.as_mut().unwrap().next = prev.take();
            prev = current_node.take();
            current_node = next;
        }
        self.head = prev.take();
    }
}

fn sort_lists(vec_list: &mut Vec<Linklist<i32>>) -> Linklist<i32> {
    let mut sorted_list = Linklist::new();
    let mut values: Vec<i32> = Vec::new();

    while true {
        let values = vec_list
            .into_iter()
            .map(|x| x.head.as_ref().unwrap().element) // Get the element at the head of each linklist
            .collect::<Vec<i32>>();

        let min_val = *values.iter().min().unwrap();
        let min_index = values.iter().position(|x| *x == min_val).unwrap();
        sorted_list.add(min_val);
        vec_list[min_index].remove(); // Remove the head of the linklist

        if vec_list[min_index].head.is_none() {
            vec_list.remove(min_index); // Remove the linklist if it is empty
        }

        if vec_list.len() == 0 {
            break;
        }
    }
    sorted_list
}

fn main() {
    let mut list1 = Linklist::new();
    list1.add(45);
    list1.add(40);
    list1.add(35);
    list1.add(23);
    list1.add(11);

    let mut list2 = Linklist::new();
    list2.add(60);
    list2.add(44);

    let mut list3 = Linklist::new();
    list3.add(85);
    list3.add(20);
    list3.add(15);

    let mut result = sort_lists(&mut vec![list1, list2, list3]);
    result.printing();

    result.reverse();
    result.printing();
}
