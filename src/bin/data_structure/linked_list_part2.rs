// --------------------------------------------
//          Link List (Part 2)
// --------------------------------------------

#[derive(Debug)]
struct Linklist {
    head: pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: pointer,
}

type pointer = Option<Box<Node>>;

impl Linklist {
    fn new() -> Linklist {
        Linklist { head: None }
    }

    fn add(&mut self, element: i32) {
        // Original
        // match self.head {
        //     None => {
        //         let new_node = Some(Box::new(Node {
        //             element: element,
        //             next: None,
        //         }));
        //         self.head = new_node;
        //     }
        //     /*
        //         Note that the varible inside the 'Some(previos_head)' captures the unwrapped value 
        //         of the option we are matching on(self.head). Furthermore, it takes the value by ownership.

        //         However, with mutable reference(&mut self) we cannot take the ownership of something.

        //         To fix this, we want to leave something in place of the head for some temporary time
        //         while we are updating it by using the special memory handling function called 'take'.
        //     */
        //     Some(previos_head) => {
        //         let new_node = Some(Box::new(Node {
        //             element: element,
        //             next: Some(previos_head),
        //         }));
        //         self.head = new_node;
        //     }
        // }

        /*
            Using the special memory handling function called "take".

            This is the take function signature.
            fn take<T>(dest: &mut) -> T

            It will replace the head with defualt value, since the head
            is an option. So the default value of option, which is the 'None' variant, 
            will be assigned to head.

            Futhermore, the value of head will be returned and will be stored
            in the 'previous_head'. the 'previous_head' now contains the old head.
        */
        let previous_head = self.head.take();       // Export its value to 'previous_head' and left 'None' to itself.
        let new_head = Some(Box::new(Node {
            element: element,
            next: previous_head,
        }));
        self.head = new_head;
    }

    fn remove(&mut self) -> Option<i32> {
        /*
            self.head.take() will return the head and
            will replace the head of the list with 'None'.
        */
        match self.head.take() {                   // If the head is not empty, we will have 'Some(previous_head)'.
            Some(previous_head) => {    // The head is next stored in the 'previous_head'.
                /*
                    Inside the Some() arm, we update the head with the next previous head.
                    This implicitly means that the old head is being removed.
                */
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }

    fn print(&self) {
        let mut list_traversal = &self.head;
        while !list_traversal.is_none() {
            // Use as_ref() to pass as reference to make sure the ownership is not changed.
            println!("{:?}", list_traversal.as_ref().unwrap().element);
            list_traversal = &list_traversal.as_ref().unwrap().next;
        }
    }
}

fn main() {
    let mut list = Linklist::new();
    list.add(5);
    list.add(7);
    list.add(10);
    list.add(15);
    list.add(20);

    // println!("List: {:?}", list);
    list.print();
    println!("{}", list.remove().unwrap());
}