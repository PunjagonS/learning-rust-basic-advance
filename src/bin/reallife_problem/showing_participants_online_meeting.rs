/*  -----------------------------------------------------------------------------------
    Displaying Participants of an Online Meeting
        Description
        - Retrieving the list of paginated view of participants
          of an online meeting.

        Tools
        - Binary Search Tree (BST) + Stack


    Solution
    + push_all_left_node_to_stack()
        - When called for a node, will always push the left nodes of that node
          onto the stack.
    + next_name()
        - Will return the next name from the tree.
        - It will be called 10 times.
        - It does 2 things:
            - Pops the top element the stack and returns its value.
            - Call the push_all_left_node_to_stack on the right child of removed element.
    + next_page()
        - Calls `next_name()` 10 times and returns the resultant names.
    -----------------------------------------------------------------------------------
*/
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Node {
    value: String,
    left: Link,
    right: Link,
}

impl Node {
    fn new(value: String) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: String) {
        /*
            In order to compare 2 values, we will use their alphabetical and lexicographical order.
            to determine whether one value is greater than the other, we will use the built-in
            comparison operator `>` for strings.
        */
        if value > self.value {
            match &self.right {
                None => self.right = Some(Rc::new(RefCell::new(Self::new(value)))),
                Some(node) => node.borrow_mut().insert(value),
            }
        } else {
            match &self.left {
                None => self.left = Some(Rc::new(RefCell::new(Self::new(value)))),
                Some(node) => node.borrow_mut().insert(value),
            }
        }
    }
}

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct BinarySearchTree {
    root: Node,
}

impl BinarySearchTree {
    fn new(value: String) -> Self {
        BinarySearchTree {
            root: Node::new(value),
        }
    }

    fn insert(&mut self, value: String) {
        self.root.insert(value);
    }
}

struct DisplayLobby {
    stack: Vec<Rc<RefCell<Node>>>,
}

impl DisplayLobby {
    fn new(root: Link) -> Self {
        let mut stack = Vec::new();
        Self::push_all_left_node_to_stack(root.clone(), &mut stack);
        DisplayLobby { stack }
    }

    fn push_all_left_node_to_stack(mut current_node: Link, stack: &mut Vec<Rc<RefCell<Node>>>) {
        /*
            repeatedly traversing the left children of the nodes in the binary search tree
            and pushing each node onto the stack.

            loop until `Some(node)` is false (None).
        */
        while let Some(node) = current_node.clone() {
            stack.push(node.clone());
            current_node = node.borrow().left.clone();
        }
    }

    fn next_page(&mut self) -> Vec<String> {
        let mut names = Vec::new();
        for _ in 0..10 {
            if !self.stack.is_empty() {
                names.push(self.next_name());
            } else {
                break;
            }
        }

        names
    }

    fn next_name(&mut self) -> String {
        let top_element = self.stack.pop().unwrap();
        let name = &top_element.borrow().value;

        /*
            Push all the left nodes of the right child to the stack
            and do all this until no right child of each node exists.
        */
        let mut right_node = top_element.borrow().right.clone();
        Self::push_all_left_node_to_stack(right_node, &mut self.stack);

        name.to_string()
    }
}

fn main() {
    let mut bst = BinarySearchTree::new("Jeanette".to_string());
    let names = vec![
        "Latasha",
        "Elvira",
        "Caryl",
        "Antionetter",
        "Cassie",
        "Charity",
        "Lyn",
        "Lia",
        "Anya",
        "Albert",
        "Cherlyn",
        "Lala",
        "Kandince",
        "Iliana",
        "Nouman",
        "Azam",
    ];

    for name in names {
        bst.insert(name.to_string());
    }

    let mut display = DisplayLobby::new(Some(Rc::new(RefCell::new(bst.root))));
    println!("Participants on the first page: {:?}", display.next_page());
    println!("Participants on the second page: {:?}", display.next_page());
}
