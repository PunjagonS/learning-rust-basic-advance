![Logo](https://www.bleepstatic.com/content/hl-images/2021/04/03/rust-bg.jpeg)

# Rust From Beginner to Expert 2.0

Learning Rust from basic to advanced.

## Lessons Learned

- Section 3 -> Quickstart
- Section 4 -> Ownership
- Section 5 -> Types & Functions
- Section 6 -> Organizing Code
- Section 7 -> Testing Code
- Section 9 -> Flexibility and Abstraction with Generics and Traits
- Section 10 -> Functional Programming Aspects
- Section 11 -> Memory Management Features
- Section 12 -> Implementing Typical Data Structures
- Section 13 -> Useful Patterns for Handling Structs
- Section 15 -> Understanding Size in Rust
- Section 16 -> Concurrency

### Using Nightly Rust

To switch to the nightly version of Rust, follow these steps:

1. **Install the nightly toolchain:**

   ```bash
   rustup toolchain install nightly
   ```

2. **Set the nightly toolchain as the default:**

   ```bash
   rustup default nightly
   ```

Alternatively, to use the nightly toolchain for a specific project:

1. **Override the toolchain for the current directory:**

   ```bash
   rustup override set nightly
   ```

### Module Tree Structure

To better understand the structure of your Rust code, you can visualize the module tree using the `cargo-modules` tool. Follow these steps:

1. **Update Rust:**

   ```bash
   rustup update
   ```

2. **Install cargo-modules:**

   ```bash
   cargo install cargo-modules
   ```

3. **Generate and view the module structure:**

   ```bash
   cargo modules structure --lib
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

2. **Commands to Run Unit Tests:**

   ```bash
   cargo test --doc
   ```

   Description: Runs documentation tests.

   ```bash
   cargo test --lib
   ```

   Description: Runs all unit tests in the library.

   ```bash
   cargo test --lib should_not
   ```

   Description: Runs a specific unit test named `should_not` in the library.

   ```bash
   cargo test --lib -- --ignored
   ```

   Description: Runs all unit tests in the library that are marked as ignored.

3. **Command to Run Integration Test:**

   ```bash
   cargo test --test order_test
   ```

   Description: Runs a specific integration test file named `order_test.rs`.

4. **Command to Run Benchmark Test:**

   ```bash
   cargo bench
   ```

   Description: Runs benchmark tests.