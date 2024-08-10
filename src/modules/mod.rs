// pub mod customer;                   // Exposing and making things in module public
// pub mod product;                    // Exposing and making things in module public

mod customer;
mod product;
mod order;

/*
    Exposing only Customer and Product structs visible from outside
 */
pub use customer::Customer;             
pub use product::{Product, Category};               