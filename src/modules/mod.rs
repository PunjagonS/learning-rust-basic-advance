// pub mod customer;                   // Exposing and making things in module public
// pub mod product;                    // Exposing and making things in module public

// Exposing entire benchmark_testing module
pub mod benchmark_testing;
mod customer;
mod order;
mod product;
mod shapes;
mod units_testing;

mod student;

// mod enums;
pub mod enums;

/*
   Exposing only Structs visible from outside
*/
pub use customer::Customer;
pub use order::Order;
pub use product::{Category, Product};
pub use shapes::Circle;

pub use student::Student;

// pub use enums::MyLongEnum;
