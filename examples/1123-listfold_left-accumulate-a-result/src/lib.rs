#![allow(clippy::all)]
//! List.fold_left — Accumulate a Result
//! See example.ml for OCaml reference
//!
//! OCaml's `List.fold_left f acc xs` applies `f acc x` for each element left-to-right,
//! threading the accumulator through. Maps to Rust's `Iterator::fold`.

/// Compute the sum of a slice using a left fold.
/// Mirrors OCaml: `List.fold_left ( + ) 0 numbers`
pub fn sum(numbers: &[i64]) -> i64 {
    numbers.iter().fold(0, |acc, &x| acc + x)
}

/// Compute the product of a slice using a left fold.
/// Mirrors OCaml: `List.fold_left ( * ) 1 numbers`
pub fn product(numbers: &[i64]) -> i64 {
    numbers.iter().fold(1, |acc, &x| acc * x)
}

/// Find the maximum value using a left fold.
/// Mirrors OCaml: `List.fold_left max min_int numbers`
pub fn max_val(numbers: &[i64]) -> Option<i64> {
    numbers.iter().copied().reduce(i64::max)
}

/// Generic left fold — mirrors OCaml's `List.fold_left f acc xs`.
pub fn fold_left<T, U, F>(items: &[T], init: U, f: F) -> U
where
    F: Fn(U, &T) -> U,
{
    items.iter().fold(init, f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum(&[]), 0);
        assert_eq!(sum(&[42]), 42);
    }

    #[test]
    fn test_product() {
        assert_eq!(product(&[1, 2, 3, 4, 5]), 120);
        assert_eq!(product(&[]), 1);
        assert_eq!(product(&[7]), 7);
    }

    #[test]
    fn test_max_val() {
        assert_eq!(max_val(&[3, 1, 4, 1, 5, 9, 2, 6]), Some(9));
        assert_eq!(max_val(&[42]), Some(42));
        assert_eq!(max_val(&[]), None);
    }

    #[test]
    fn test_fold_left_string_build() {
        let words = ["hello", " ", "world"];
        let result = fold_left(&words, String::new(), |acc, s| acc + s);
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_fold_left_count() {
        let nums = [1, 2, 3, 4, 5, 6];
        let evens = fold_left(&nums, 0, |acc, &x| if x % 2 == 0 { acc + 1 } else { acc });
        assert_eq!(evens, 3);
    }
}
