//! Example demonstrating fold_left in Rust.
//!
//! This mirrors the OCaml example:
//! ```ocaml
//! let numbers = [1; 2; 3; 4; 5]
//! let sum = List.fold_left ( + ) 0 numbers
//! let product = List.fold_left ( * ) 1 numbers
//! let max_val = List.fold_left max min_int numbers
//! let () = Printf.printf "Sum: %d, Product: %d, Max: %d\n" sum product max_val
//! ```

use list_fold_left::{fold_left_iter, fold_left_recursive, max_value, product, sum};

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Using iterator-based fold_left (idiomatic Rust)
    let sum_iter = fold_left_iter(0, &numbers, |acc, x| acc + x);
    let product_iter = fold_left_iter(1, &numbers, |acc, x| acc * x);
    let max_iter = fold_left_iter(i32::MIN, &numbers, |acc, x| if x > &acc { *x } else { acc });

    // Using recursive fold_left (functional style)
    let sum_rec = fold_left_recursive(0, &numbers, |acc, x| acc + x);
    let product_rec = fold_left_recursive(1, &numbers, |acc, x| acc * x);
    let max_rec = fold_left_recursive(i32::MIN, &numbers, |acc, x| if x > &acc { *x } else { acc });

    // Using convenience functions
    let sum_fn = sum(&numbers);
    let product_fn = product(&numbers);
    let max_fn = max_value(&numbers);

    println!("=== Iterator-based fold_left ===");
    println!(
        "Sum: {}, Product: {}, Max: {}",
        sum_iter, product_iter, max_iter
    );

    println!("\n=== Recursive fold_left ===");
    println!(
        "Sum: {}, Product: {}, Max: {}",
        sum_rec, product_rec, max_rec
    );

    println!("\n=== Using convenience functions ===");
    println!("Sum: {}, Product: {}, Max: {}", sum_fn, product_fn, max_fn);

    println!("\n=== Edge cases ===");
    let empty: Vec<i32> = vec![];
    println!(
        "Empty list - sum: {}, product: {}, max: {}",
        sum(&empty),
        product(&empty),
        max_value(&empty)
    );

    let single = vec![42];
    println!(
        "Single element [42] - sum: {}, product: {}, max: {}",
        sum(&single),
        product(&single),
        max_value(&single)
    );

    let negative = vec![-5, 10, -3];
    println!(
        "Negative values [-5, 10, -3] - sum: {}, product: {}, max: {}",
        sum(&negative),
        product(&negative),
        max_value(&negative)
    );
}
