// pub mod customer;                   // Exposing and making things in module public
// pub mod product;                    // Exposing and making things in module public

mod customer;
mod product;
mod order;
mod shapes;
mod units_testing;
pub mod benchmark_testing;              // Exposing entire benchmark_testing module

mod student;

/*
    Exposing only Structs visible from outside
 */
pub use customer::Customer;             
pub use product::{Product, Category};   
pub use order::Order;
pub use shapes::Circle;

pub use student::Student;