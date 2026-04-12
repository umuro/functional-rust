#![allow(dead_code)]
//! List.fold_left — Accumulate a Result
//! See example.ml for OCaml reference
//!
//! OCaml's `List.fold_left f acc xs` applies `f acc x` for each element left-to-right,
//! threading the accumulator through. Rust's `Iterator::fold` is the direct equivalent.

/// Generic left fold — mirrors OCaml's `List.fold_left f acc xs`.
/// Threads `init` through every element using `f`.
pub fn fold_left<T, U, F>(items: &[T], init: U, f: F) -> U
where
    F: Fn(U, &T) -> U,
{
    items.iter().fold(init, f)
}

/// Idiomatic Rust: sum using the specialized `.sum()` adapter.
/// More efficient than a manual fold; Rust knows how to optimize it.
pub fn sum_idiomatic(numbers: &[i64]) -> i64 {
    numbers.iter().copied().sum()
}

/// Idiomatic Rust: product using the specialized `.product()` adapter.
pub fn product_idiomatic(numbers: &[i64]) -> i64 {
    numbers.iter().copied().product()
}

/// Functional style: sum via an explicit fold, mirroring OCaml's `List.fold_left (+) 0 xs`.
pub fn sum_fold(numbers: &[i64]) -> i64 {
    fold_left(numbers, 0, |acc, &x| acc + x)
}

/// Functional style: product via an explicit fold, mirroring OCaml's `List.fold_left (*) 1 xs`.
pub fn product_fold(numbers: &[i64]) -> i64 {
    fold_left(numbers, 1, |acc, &x| acc * x)
}

/// Find the maximum value. Returns `None` for an empty slice.
/// Uses `Iterator::reduce` (fold without a seed) so the empty case is explicit in the type.
pub fn max_val(numbers: &[i64]) -> Option<i64> {
    numbers.iter().copied().reduce(i64::max)
}

/// Recursive fold: mirrors OCaml's `let rec fold_left_rec f acc = function ...`
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
        assert_eq!(sum_idiomatic(&[]), 0);
        assert_eq!(sum_fold(&[]), 0);
    }

    #[test]
    fn test_sum_single() {
        assert_eq!(sum_idiomatic(&[42]), 42);
        assert_eq!(sum_fold(&[42]), 42);
    }

    #[test]
    fn test_sum_multiple() {
        assert_eq!(sum_idiomatic(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_fold(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_product_multiple() {
        assert_eq!(product_idiomatic(&[1, 2, 3, 4, 5]), 120);
        assert_eq!(product_fold(&[1, 2, 3, 4, 5]), 120);
    }

    #[test]
    fn test_max_val() {
        assert_eq!(max_val(&[3, 1, 4, 1, 5, 9, 2, 6]), Some(9));
        assert_eq!(max_val(&[]), None);
    }

    #[test]
    fn test_fold_left_string_build() {
        let words = ["hello", " ", "world"];
        let result = fold_left(&words, String::new(), |acc, s| acc + s);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_fold_left_count_evens() {
        let nums = [1, 2, 3, 4, 5, 6_i64];
        let evens = fold_left(&nums, 0, |acc, &x| if x % 2 == 0 { acc + 1 } else { acc });
        assert_eq!(evens, 3);
    }

    #[test]
    fn test_fold_left_recursive_sum() {
        let numbers = [1_i64, 2, 3, 4, 5];
        let result = fold_left_recursive(&numbers, 0, &|acc, &x| acc + x);
        assert_eq!(result, 15);
    }
}
