# Rust Borrow Checker Error: Immutable Borrow

This repository demonstrates a common error encountered in Rust programming: attempting to modify a vector while it's immutably borrowed.

The `bug.rs` file contains the erroneous code.  The `bugSolution.rs` file shows the corrected version.

This error highlights Rust's ownership and borrowing system, which prevents data races and ensures memory safety. Understanding and avoiding this type of error is crucial for writing safe and efficient Rust code.