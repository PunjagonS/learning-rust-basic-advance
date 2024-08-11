
![Logo](https://www.bleepstatic.com/content/hl-images/2021/04/03/rust-bg.jpeg)


# Rust From Beginner to Expert 2.0

Learning Rust from basic to advance


## Lessons Learned

- Section 3 -> Quickstart
- Section 4 -> Ownership
- Section 5 -> Types && Function
- Section 6 -> Organizing Code
- Section 7 -> Testing Code

### Module Tree Structure

To better understand the structure of your Rust code, you can visualize the module tree using the `cargo-modules` tool. Follow these steps:

1. **Install cargo-modules:**

   ```bash
   cargo install cargo-modules

2. **Generate and view the module structure:**

   ```bash
   cargo modules generate tree --lib

### Testing Code

1. **Command to Run All Tests:**

   ```bash
   cargo test

Description: Runs all tests including unit tests, integration tests, and documentation tests.

2. **Commands to Run Unit Tests:**

   ```bash
   cargo test --doc
   cargo test --lib
   cargo test --lib should_not
   cargo test --lib -- --ignored

3. **Command to Run Integration Tests:**

   ```bash
   cargo test --test order_test

Description: Runs a specific integration test file named order_test.rs.