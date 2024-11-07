![Logo](https://jav69-public.s3.ap-southeast-1.amazonaws.com/rust_design.jpg)

# Rust From Beginner to Expert 2.0

Learning Rust from basic to advance

## Lessons Learned

- Section 3 -> Quickstart
- Section 4 -> Ownership
- Section 5 -> Types && Function
- Section 6 -> Organizing Code
- Section 7 -> Testing Code
- Section 9 -> Flexibility and Abstraction with Generics and Traits
- Section 10 -> Functional Programming Aspects
- Section 11 -> Memory Management Features
- Section 12 -> Implementing Typical Data Structures
- Section 13 -> Useful Patterns for Handling Structs
- Section 15 -> Understanding Size in Rust
- Section 16 -> Concurrency
- Section 17 -> Macros
- Section 18 -> Real Life Problems
- Section 20 -> Web Programming
- Section 21 -> Text Processing, File and Directory Handling
- Section 22 -> Miscellaneous Topics

### Module Tree Structure

To better understand the structure of your Rust code, you can visualize the module tree using the `cargo-modules` tool. Follow these steps:

1. **Install cargo-modules:**

   ```bash
   cargo install cargo-modules
   ```

2. **Generate and view the module structure:**

   ```bash
   cargo modules generate tree --lib
   ```

### Using Nightly Rust

To install and switch to the nightly version of Rust, follow these steps:

1. **Install `rustup` using Homebrew (if not already installed):**

   ```bash
   brew install rustup-init
   ```

2. **Initialize `rustup` with the nightly toolchain**

   ```bash
   rustup-init --default-toolchain nightly -y
   ```

   This will install `rustup`, the Rust toolchain installer, and set the default toolchain to the nightly version.

3. **Switch to the nightly toolchain:**

   ```bash
   rustup default nightly
   ```

4. **Switch to the stable toolchain:**

   ```bash
   rustup default stable
   ```

### Expanding Macros

To expand macros for a specific binary target, you can use the `cargo expand` command. This is useful for debugging and understanding how macros are transformed into code.

1. **Install cargo-expand:**

   ```bash
   cargo install cargo-expand
   ```

2. **Expand macros for a specific binary target:**

   ```bash
   cargo expand --bin <binary_name>
   ```

   Replace `<binary_name>` with the name of the binary you want to expand. For example, to expand the `capturing_types` binary:

   ```bash
   cargo expand --bin capturing_types
   ```

### Testing Code

1. **Command to Run All Tests:**

   ```bash
   cargo test
   ```

   Description: Runs all tests including unit tests, integration tests, and documentation tests.

2. **Command to Run Documentation Tests:**

   ```bash
   cargo test --doc
   ```

   Description: Runs tests embedded in the documentation.

3. **Command to Run Library Tests:**

   ```bash
   cargo test --lib
   ```

   Description: Runs tests in the library crate.

4. **Command to Run Specific Library Test:**

   ```bash
   cargo test --lib should_not
   ```

   Description: Runs a specific test named `should_not` in the library crate.

5. **Command to Run Ignored Tests:**

   ```bash
   cargo test --lib -- --ignored
   ```

   Description: Runs tests in the library crate that are marked as ignored.

6. **Command to Run Integration Test:**

   ```bash
   cargo test --test order_test
   ```

   Description: Runs a specific integration test file named `order_test.rs`.

7. **Command to Run Integration Test with Output:**

   ```bash
   cargo test --test quick_dev -- --nocapture
   ```

   Description: Runs a specific integration test file named `quick_dev.rs` and displays the output during the test execution.

8. **Command to Run Benchmark Test:**

   ```bash
   cargo bench
   ```

   Description: Runs benchmark tests.