// --------------------------------------------
//          Integration Testing
// --------------------------------------------
/*  
    Create folder "tests" at root package follow rust typical file system
    to let cargo know to look for integration tests inside this directory.
 */

use basic_advance::modules::{Customer, Product, Order, Category};
use helpers::common_setup;

mod helpers;

#[test]
fn test_total_bill_without_discount() {
    common_setup();
    let product = Product::new(1, "Book".to_string(), 19.9, Category::Books);
    let customer = Customer::new(1, "Bob".to_string(), "bb@example.com".to_string());
    let order = Order::new(2, product, customer, 3);

    assert_eq!(format!("{:.2}", order.total_bill()), "65.67");                      // "{:.2}" is to format floating number to 2 decimal place
}


#[test]
fn test_total_bill_with_discount() {
    let product = Product::new(1, "Book".to_string(), 19.99, Category::Books);
    let customer = Customer::new(1, "Bob".to_string(), "bb@example.com".to_string());
    let order = Order::new(2, product, customer, 10);

    assert_eq!(format!("{:.2}", order.total_bill()), "197.90");                      // "{:.2}" is to format floating number to 2 decimal place
}