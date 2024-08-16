// --------------------------------------------
//          RefCell Example
// --------------------------------------------

use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct File {
    active_user: u32,
}

#[derive(Debug)]
struct User {
    file: Rc<File>,
}

#[derive(Debug)]
struct UserRefCell {
    file: Rc<RefCell<File>>,
}

fn main() {
    let mut txt_file = Rc::new(File { active_user: 0 });
    let mut user_1 = User {
        file: Rc::clone(&txt_file),
    };
    /*
        Error: RC smart pointer only allows immutable shared ownership of a value.
        cannot change 'file.active_user' value. Overcome this by using RefCell Smart Pointer.
    */
    // user_1.file.active_user += 1;
    let mut user_2 = User {
        file: Rc::clone(&txt_file),
    };



    // Using RefCell Smart Pointer.
    let mut txt_file_2 = Rc::new(RefCell::new(File{ active_user: 0 }));
    let mut user_3 = UserRefCell {
        file: Rc::clone(&txt_file_2),
    };
    user_3.file.borrow_mut().active_user += 1;
    println!("Active users: {:?}", txt_file_2.borrow_mut().active_user);

    let mut user_4 = UserRefCell {
        file: Rc::clone(&txt_file_2),
    };
    user_4.file.borrow_mut().active_user += 1;
    println!("Active users: {:?}", txt_file_2.borrow_mut().active_user);
}   