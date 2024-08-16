// --------------------------------------------
//          Link List (Part 1)
// --------------------------------------------

#[derive(Debug)]
struct Linklist {
    head: Node,
}

#[derive(Debug)]
struct Linklist2 {
    // head: Option<Node>,
    // head: Option<Box<Node>>,
    head: pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: pointer,
}

// Define alias for 'Option<Box<Node>>' to equal pointer.
type pointer = Option<Box<Node>>;

fn main() {
    let list = Node {
        element: 1,
        next: None,
    };

    let list2 = Node {
        element: 1,
        next: Some(Box::new(Node {
            element: 2,
            next: None,
        })),
    };

    let list3 = Linklist {
        head: Node {
            element: 1,
            next: None,
        }
    };

    let list4 = Linklist {
        head: Node {
            element: 1,
            next: Some(Box::new(Node {
                element: 2,
                next: Some(Box::new(Node { 
                    element: 3, 
                    next: None,
                })),
            })),
        }
    };

    let list5 = Linklist2 { head: None };

    let list6 = Linklist2{
        head: Some(Box::new(Node 
            { 
                element: 100, 
                next: Some(Box::new(Node 
                    { 
                        element: 200, 
                        next: None,
                    })),
            })),
    };

    println!("{:?}", &list6.head);

    // .unwrap() is needed for 'Option<>' type.
    println!("{:?}", &list6.head.unwrap().next.unwrap().element);
}