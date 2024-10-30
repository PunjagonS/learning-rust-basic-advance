#![feature(async_closure)]
// --------------------------------------------
//                Async Closures
// --------------------------------------------

/*
    ref: https://blog.rust-lang.org/inside-rust/2024/08/09/async-closures-call-for-testing.html

    With the async closures feature, we can now define and pass async closures
    directly without having to use multiple generic types or explicitly handle
    futures. This simplifies the code for async functions that take closures as arguments,
    making it easier to pass closures that return async values without extra setup.

    Higher-ranked types (or higher-ranked trait bounds in Rust) refer to the use of generics
    that allow for a type’s lifetime to be flexible and adaptable, without needing to specify it upfront.
    For example, when using closures or functions that reference a borrowed reference,
    higher-ranked types can allow the lifetime to vary depending on the context in which it is called,
    rather than locking the reference to a single, fixed lifetime.
*/

use std::{
    future::Future,
    ops::{AsyncFn, AsyncFnOnce},
};

// Here’s how it works:
fn doesnt_exactly_take_an_async_closure<F, Fut>(callback: F)
where
    F: FnOnce() -> Fut, // Before async closures
    Fut: Future<Output = String>,
{
    // Execute the callback and await its result
    let result = tokio::runtime::Runtime::new().unwrap().block_on(callback());
    println!(
        "Result from doesnt_exactly_take_an_async_closure: {}",
        result
    );
}

// with async_closures
// fn takes_an_async_closure<F: async FnOnce() -> String>(callback: F) {
// or alternatively
// Update the function signature to use higher-ranked trait bounds
fn takes_an_async_closure<F: AsyncFnOnce() -> String>(callback: F) {
    let result = tokio::runtime::Runtime::new().unwrap().block_on(callback());
    println!("Result from takes_an_async_closure: {}", result);
}

// Example of a higher-ranked function
fn higher_ranked<F: AsyncFn(&str) -> String>(callback: F) {
    let result = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(callback("Hello from higher_ranked"));
    println!("Result from higher_ranked: {}", result);
}

fn main() {
    // Example usage with `doesnt_exactly_take_an_async_closure`
    doesnt_exactly_take_an_async_closure(|| async {
        String::from("Hello from doesnt_exactly_take_an_async_closure")
    });

    // Example usage with `takes_an_async_closure`
    takes_an_async_closure(|| async { String::from("Hello from takes_an_async_closure") });

    // Example usage with `higher_ranked`
    higher_ranked(async |input| {
        println!("Received input in higher_ranked: {}", input);
        String::from("Processed by higher_ranked")
    });
}
