//! OCaml → Rust: List Filtering Example
//! Demonstrates idiomatic Rust approaches to filtering lists.

use list_filter::{filter_in_place, filter_iter, filter_recursive};

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];

    // Idiomatic Rust: using iterators
    let evens = filter_iter(&numbers, |x| x % 2 == 0);
    let odds = filter_iter(&numbers, |x| x % 2 != 0);

    println!("Evens: {}", format_vec(&evens));
    println!("Odds: {}", format_vec(&odds));

    // Recursive functional style (OCaml-like)
    let evens_recursive = filter_recursive(&numbers, |x| x % 2 == 0);
    let odds_recursive = filter_recursive(&numbers, |x| x % 2 != 0);

    println!("Evens (recursive): {}", format_vec(&evens_recursive));
    println!("Odds (recursive): {}", format_vec(&odds_recursive));

    // In-place filtering
    let mut numbers_copy = numbers.clone();
    filter_in_place(&mut numbers_copy, |x| x % 2 == 0);
    println!("Evens (in-place): {}", format_vec(&numbers_copy));

    // Complex predicate
    let between_3_and_7 = filter_iter(&numbers, |x| x > &3 && x < &8);
    println!("Between 3 and 7: {}", format_vec(&between_3_and_7));
}

/// Format a vector as a comma-separated string
fn format_vec(vec: &[i32]) -> String {
    vec.iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}
