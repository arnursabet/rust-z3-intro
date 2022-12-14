# Rust Z3 API Tutorial Source Code

Refer to the [Rust Z3 API Tutorial](https://arnur.netlify.app/z3-rust-intro) for the article.

This is the first tutorial on the basics of the unofficial Rust Z3 binding. It covers the following topics:
* Getting Started
* Boolean Logic
* Arithmetic
* Machine Arithmetic
* Functions
* Solvers
* Some Rust Hacks
* Satisfiability and Validity
* Applications: puzzles, install problem, linear programming

## Running 
To run the code examples, uncomment one of the lines in the main file.
```rust
fn main() {
    getting_started::run1();
    // getting_started::run2();
    // getting_started::run3();
    // getting_started::run4();
    // getting_started::run5();

    // boolean_logic::run1();
    // boolean_logic::run2();

    // solvers::run1();
    // solvers::run2();
    // solvers::run3();

    // arithmetic::run1();
    // arithmetic::run2();

    // machine_arithmetic::run1();
    // machine_arithmetic::run2();
    // machine_arithmetic::run3();

    // functions::run1();
    // functions::run2();

    // satisfiability_validity::run1();

    // list_compr::run1();

    // puzzles::dog_cat_mouse();
    // puzzles::sudoku();
    // puzzles::eight_queens();

    // install_problem::run();
}

```
Then, in your terminal:

```
cargo run
```

