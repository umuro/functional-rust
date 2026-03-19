#![allow(clippy::all)]
//! # fold_right — Structural Recursion
//!
//! OCaml's `fold_right` processes a list from right to left:
//!   fold_right f [a; b; c] init = f a (f b (f c init))
//!
//! In Rust, the closest stdlib equivalent is `Iterator::rfold` (on
//! double-ended iterators) or simply `fold` with reversed logic.

// ---------------------------------------------------------------------------
// Approach A: Idiomatic Rust — use iterator combinators
// ---------------------------------------------------------------------------

/// Sum via `iter().sum()`.
pub fn sum_idiomatic(xs: &[i64]) -> i64 {
    xs.iter().sum()
}

/// Product via `iter().product()`.
pub fn product_idiomatic(xs: &[i64]) -> i64 {
    xs.iter().product()
}

/// Concatenation via `iter().collect()` (or `join`).
pub fn concat_idiomatic(xs: &[&str]) -> String {
    // collect on an iterator of &str directly concatenates
    xs.iter().copied().collect()
}

/// Copy a slice into a Vec — trivially `to_vec()`.
pub fn copy_idiomatic(xs: &[i64]) -> Vec<i64> {
    xs.to_vec()
}

// ---------------------------------------------------------------------------
// Approach B: Functional / explicit fold_right (recursive)
// ---------------------------------------------------------------------------

/// A generic right fold, mirroring OCaml's `fold_right`.
///
/// Because Rust doesn't have a cons-list with O(1) pattern matching,
/// we recurse over a slice index. This is *not* tail-recursive — it
/// mirrors OCaml's stack-consuming `fold_right` faithfully.
pub fn fold_right<T, A>(f: impl Fn(&T, A) -> A + Copy, xs: &[T], init: A) -> A {
    // We take `&T` rather than `T` because Rust slices lend references.
    match xs {
        [] => init,
        [head, tail @ ..] => f(head, fold_right(f, tail, init)),
    }
}

pub fn sum_functional(xs: &[i64]) -> i64 {
    fold_right(|x, acc| x + acc, xs, 0)
}

pub fn product_functional(xs: &[i64]) -> i64 {
    fold_right(|x, acc| x * acc, xs, 1)
}

pub fn concat_functional(xs: &[&str]) -> String {
    fold_right(|s, acc: String| format!("{s}{acc}"), xs, String::new())
}

pub fn copy_functional(xs: &[i64]) -> Vec<i64> {
    // Mirrors OCaml: fold_right (fun h t -> h :: t) lst []
    fold_right(
        |&x, mut acc: Vec<i64>| {
            acc.insert(0, x); // prepend — O(n) per call, faithful to OCaml semantics
            acc
        },
        xs,
        Vec::new(),
    )
}

// ---------------------------------------------------------------------------
// Approach C: rfold — Rust's built-in right fold on DoubleEndedIterator
// ---------------------------------------------------------------------------

pub fn sum_rfold(xs: &[i64]) -> i64 {
    xs.iter().rfold(0, |acc, &x| x + acc)
}

pub fn product_rfold(xs: &[i64]) -> i64 {
    xs.iter().rfold(1, |acc, &x| x * acc)
}

pub fn concat_rfold(xs: &[&str]) -> String {
    // rfold processes right-to-left, accumulating left-to-right
    xs.iter()
        .rfold(String::new(), |acc, &s| format!("{s}{acc}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_basic() {
        let xs = [1, 2, 3, 4, 5];
        assert_eq!(sum_idiomatic(&xs), 15);
        assert_eq!(sum_functional(&xs), 15);
        assert_eq!(sum_rfold(&xs), 15);
    }

    #[test]
    fn test_sum_empty() {
        let xs: &[i64] = &[];
        assert_eq!(sum_idiomatic(xs), 0);
        assert_eq!(sum_functional(xs), 0);
        assert_eq!(sum_rfold(xs), 0);
    }

    #[test]
    fn test_product_single() {
        let xs = [42];
        assert_eq!(product_idiomatic(&xs), 42);
        assert_eq!(product_functional(&xs), 42);
        assert_eq!(product_rfold(&xs), 42);
    }

    #[test]
    fn test_product_basic() {
        let xs = [1, 2, 3, 4, 5];
        assert_eq!(product_idiomatic(&xs), 120);
        assert_eq!(product_functional(&xs), 120);
        assert_eq!(product_rfold(&xs), 120);
    }

    #[test]
    fn test_concat() {
        let xs = ["a", "b", "c"];
        assert_eq!(concat_idiomatic(&xs), "abc");
        assert_eq!(concat_functional(&xs), "abc");
        assert_eq!(concat_rfold(&xs), "abc");
    }

    #[test]
    fn test_concat_empty() {
        let xs: &[&str] = &[];
        assert_eq!(concat_idiomatic(xs), "");
        assert_eq!(concat_functional(xs), "");
        assert_eq!(concat_rfold(xs), "");
    }

    #[test]
    fn test_copy() {
        let xs = [10, 20, 30];
        assert_eq!(copy_idiomatic(&xs), vec![10, 20, 30]);
        assert_eq!(copy_functional(&xs), vec![10, 20, 30]);
    }

    #[test]
    fn test_fold_right_generic() {
        // Use fold_right to build a string representation
        let xs = [1, 2, 3];
        let result = fold_right(
            |x, acc: String| format!("{x}::{acc}"),
            &xs,
            "[]".to_string(),
        );
        assert_eq!(result, "1::2::3::[]");
    }
}
