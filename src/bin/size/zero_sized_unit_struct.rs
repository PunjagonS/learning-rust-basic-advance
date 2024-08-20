// --------------------------------------------
//        Zero Sized Types (Unit Struct)
// --------------------------------------------

/*
    A unit struct is a struct with no fields due to 
    absence of associated fields, it has a zero size.

    Unit structs can be used as marker type for making types
    to ensure certain concepts or properties.
*/

struct Admin;

struct User;

trait Authenticate {
    fn authenticate(&self, username: &str, password: &str) -> bool;
}

impl Authenticate for Admin{
    fn authenticate(&self, username: &str, password: &str) -> bool {
        username == "admin" && password == "adminpass"
    }
}

impl Authenticate for User{
    fn authenticate(&self, username: &str, password: &str) -> bool {
        username == "user" && password == "userpass"
    }
}

fn login<T: Authenticate>(role: T, username: &str, password: &str) -> bool {
    role.authenticate(username, password)
}

fn main() {
    let admin = Admin;
    let user = User;
    let admin_login = login(Admin, "admin", "adminpass");
    let admin_login = login(User, "user", "userpass");

    if admin_login {
        println!("Admin login successful!");
    } else {
        println!("User login successful!");
    }

    let x = ();
    let y = x;
    let z = x;

    struct ABC;
    let x = ABC;
    let y = x;

    /*
        Error: ownership move to prevent accidental data duplicate. 
        if they were copyable, it could lead to unexpected behavior 
        and unintended sharing of traits or marker infomation.
    */
    // let z = x;
}