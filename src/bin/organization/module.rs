// --------------------------------------------
//          Code Organization
// --------------------------------------------

/*  
    Packages
    - Managed through Cargo commands
    - Highest level of code organization
    - Contains Crates

    Crates
    - A compilation unit
    - Can be a binary crate or library crate
    - Contains Modules

    Modules
    - Controls at a finer level, the structure, visibility and privacy

    ** Rules for package **
    - Must have at least one crate
    - At most 1 library crate
    - Any number of binary crates
 */

use basic_advance::modules::{Customer, Product, Category, Order};

fn main() {
    // let product = Product{
    //     id: 1,
    //     name: String::from("Laptop"),
    //     price: 799.99,
    //     category: Category::Electronics,
    // };

    let product = Product::new(1, "Laptop".to_string(), 799.99, Category::Electronics);
    let customer = Customer::new(1, "Alice". to_string(), "alice@example.com".to_string());
    let order = Order::new(1, product, customer, 2);

    println!("Total cost is: &{}", order.total_bill());
}