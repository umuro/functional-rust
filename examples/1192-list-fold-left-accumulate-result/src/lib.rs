#![allow(dead_code)]
//! List.fold_left — Accumulate a Result
//! See example.ml for OCaml reference
//!
//! OCaml's `List.fold_left f acc xs` applies `f acc x` for each element left-to-right,
//! threading the accumulator through. Rust's `Iterator::fold` is the direct equivalent.

/// Generic left fold: mirrors OCaml's `List.fold_left f acc xs`.
/// Takes the accumulator first, then the combining function — matching OCaml's argument order.
pub fn fold_left<T, U, F>(items: &[T], init: U, f: F) -> U
where
    F: Fn(U, &T) -> U,
{
    items.iter().fold(init, f)
}

/// Compute the sum of a slice.
/// Mirrors OCaml: `List.fold_left ( + ) 0 numbers`
pub fn sum(numbers: &[i64]) -> i64 {
    fold_left(numbers, 0, |acc, &x| acc + x)
}

/// Compute the product of a slice.
/// Mirrors OCaml: `List.fold_left ( * ) 1 numbers`
pub fn product(numbers: &[i64]) -> i64 {
    fold_left(numbers, 1, |acc, &x| acc * x)
}

/// Find the maximum value. Returns `None` for an empty slice.
/// Uses `Iterator::reduce` (fold without seed) so the empty case is explicit in the type.
pub fn max_val(numbers: &[i64]) -> Option<i64> {
    numbers.iter().copied().reduce(|a, b| a.max(b))
}

/// Functional/recursive fold: mirrors the OCaml tail-recursive implementation.
pub fn fold_left_recursive<T, U, F>(items: &[T], acc: U, f: &F) -> U
where
    F: Fn(U, &T) -> U,
{
    match items {
        [] => acc,
        [head, rest @ ..] => {
            let new_acc = f(acc, head);
            fold_left_recursive(rest, new_acc, f)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_empty() {
        assert_eq!(sum(&[]), 0);
    }

    #[test]
    fn test_sum_single() {
        assert_eq!(sum(&[42]), 42);
    }

    #[test]
    fn test_sum_multiple() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_product_empty() {
        assert_eq!(product(&[]), 1);
    }

    #[test]
    fn test_product_single() {
        assert_eq!(product(&[7]), 7);
    }

    #[test]
    fn test_product_multiple() {
        assert_eq!(product(&[1, 2, 3, 4, 5]), 120);
    }

    #[test]
    fn test_max_val_empty() {
        assert_eq!(max_val(&[]), None);
    }

    #[test]
    fn test_max_val_single() {
        assert_eq!(max_val(&[99]), Some(99));
    }

    #[test]
    fn test_max_val_multiple() {
        assert_eq!(max_val(&[3, 1, 4, 1, 5, 9, 2, 6]), Some(9));
    }

    #[test]
    fn test_fold_left_string_concat() {
        let words = ["hello", " ", "world"];
        let result = fold_left(&words, String::new(), |acc, s| acc + s);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_fold_left_recursive_sum() {
        let numbers = [1_i64, 2, 3, 4, 5];
        let result = fold_left_recursive(&numbers, 0, &|acc, &x| acc + x);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_fold_left_recursive_matches_idiomatic() {
        let numbers = [10_i64, 20, 30];
        let f = |acc: i64, &x: &i64| acc + x;
        assert_eq!(
            fold_left(&numbers, 0, &f),
            fold_left_recursive(&numbers, 0, &f)
        );
    }
}
