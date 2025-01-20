# Rust Out-of-Bounds Vector Access Bug
This repository demonstrates a common error in Rust: accessing an index out of bounds in a vector.  The code attempts to access an element beyond the vector's length, causing a panic.  The solution shows how to handle this using safe Rust techniques.

## Bug
The `bug.rs` file contains the erroneous code, which will panic at runtime due to an index out of bounds.

## Solution
The `bugSolution.rs` file provides a corrected version using appropriate error handling to prevent panics.

## How to Run
1. Clone this repository.
2. Navigate to the directory.
3. Run `rustc bug.rs && ./bug` to see the panic.
4. Run `rustc bugSolution.rs && ./bugSolution` to see the correct behavior.