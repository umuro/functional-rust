//! # fold_left — Tail-Recursive Accumulator
//!
//! OCaml's `fold_left` processes a list left to right with an accumulator:
//!   fold_left f init [a; b; c] = f (f (f init a) b) c
//!
//! This is tail-recursive in OCaml (and maps directly to Rust's `Iterator::fold`).

// ---------------------------------------------------------------------------
// Approach A: Idiomatic Rust — iterator methods
// ---------------------------------------------------------------------------

pub fn sum_idiomatic(xs: &[i64]) -> i64 {
    xs.iter().sum()
}

pub fn product_idiomatic(xs: &[i64]) -> i64 {
    xs.iter().product()
}

/// Maximum element. Returns `None` for empty slices.
/// Uses `iter().copied().max()` which returns `Option<i64>`.
pub fn maximum_idiomatic(xs: &[i64]) -> Option<i64> {
    // Unlike OCaml's version which panics on empty list, we return Option
    xs.iter().copied().max()
}

pub fn reverse_idiomatic(xs: &[i64]) -> Vec<i64> {
    let mut v = xs.to_vec();
    v.reverse();
    v
}

// ---------------------------------------------------------------------------
// Approach B: Functional / explicit fold_left (recursive, tail-recursive style)
// ---------------------------------------------------------------------------

/// A generic left fold mirroring OCaml's `fold_left`.
///
/// Rust doesn't guarantee TCO, but the structure is identical to OCaml's
/// tail-recursive fold_left. For large inputs, the iterative version is safer.
pub fn fold_left<T, A>(f: impl Fn(A, &T) -> A, mut acc: A, xs: &[T]) -> A {
    for x in xs {
        acc = f(acc, x);
    }
    acc
}

pub fn sum_functional(xs: &[i64]) -> i64 {
    fold_left(|acc, &x| acc + x, 0, xs)
}

pub fn product_functional(xs: &[i64]) -> i64 {
    fold_left(|acc, &x| acc * x, 1, xs)
}

pub fn maximum_functional(xs: &[i64]) -> Option<i64> {
    // Safe version: use first element as initial accumulator
    let (&first, rest) = xs.split_first()?;
    Some(fold_left(
        |acc, &x| if x > acc { x } else { acc },
        first,
        rest,
    ))
}

pub fn reverse_functional(xs: &[i64]) -> Vec<i64> {
    // Mirrors OCaml: fold_left (fun acc x -> x :: acc) [] lst
    fold_left(
        |mut acc: Vec<i64>, &x| {
            acc.push(x); // push + final reverse, or insert(0, x) like OCaml's cons
            acc
        },
        Vec::new(),
        xs,
    )
    .into_iter()
    .rev()
    .collect()
}

// ---------------------------------------------------------------------------
// Approach C: Iterator::fold — Rust's built-in left fold
// ---------------------------------------------------------------------------

/// Sum using Iterator::fold — explicitly showing the fold call.
/// We add a type annotation to show fold's signature clearly.
pub fn sum_iter_fold(xs: &[i64]) -> i64 {
    xs.iter().copied().fold(0i64, i64::wrapping_add)
}

/// Product using Iterator::fold.
pub fn product_iter_fold(xs: &[i64]) -> i64 {
    xs.iter().copied().fold(1i64, i64::wrapping_mul)
}

pub fn reverse_iter_fold(xs: &[i64]) -> Vec<i64> {
    // fold left, prepending each element
    xs.iter().fold(Vec::new(), |mut acc, &x| {
        acc.insert(0, x);
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_basic() {
        let xs = [3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(sum_idiomatic(&xs), 31);
        assert_eq!(sum_functional(&xs), 31);
        assert_eq!(sum_iter_fold(&xs), 31);
    }

    #[test]
    fn test_sum_empty() {
        let xs: &[i64] = &[];
        assert_eq!(sum_idiomatic(xs), 0);
        assert_eq!(sum_functional(xs), 0);
        assert_eq!(sum_iter_fold(xs), 0);
    }

    #[test]
    fn test_product_single() {
        let xs = [7];
        assert_eq!(product_idiomatic(&xs), 7);
        assert_eq!(product_functional(&xs), 7);
        assert_eq!(product_iter_fold(&xs), 7);
    }

    #[test]
    fn test_product_basic() {
        let xs = [3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(product_idiomatic(&xs), 6480);
        assert_eq!(product_functional(&xs), 6480);
        assert_eq!(product_iter_fold(&xs), 6480);
    }

    #[test]
    fn test_maximum() {
        let xs = [3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(maximum_idiomatic(&xs), Some(9));
        assert_eq!(maximum_functional(&xs), Some(9));
    }

    #[test]
    fn test_maximum_empty() {
        let xs: &[i64] = &[];
        assert_eq!(maximum_idiomatic(xs), None);
        assert_eq!(maximum_functional(xs), None);
    }

    #[test]
    fn test_maximum_single() {
        assert_eq!(maximum_idiomatic(&[42]), Some(42));
        assert_eq!(maximum_functional(&[42]), Some(42));
    }

    #[test]
    fn test_reverse() {
        let xs = [3, 1, 4, 1, 5, 9, 2, 6];
        let expected = vec![6, 2, 9, 5, 1, 4, 1, 3];
        assert_eq!(reverse_idiomatic(&xs), expected);
        assert_eq!(reverse_functional(&xs), expected);
        assert_eq!(reverse_iter_fold(&xs), expected);
    }

    #[test]
    fn test_reverse_empty() {
        let xs: &[i64] = &[];
        let empty: Vec<i64> = vec![];
        assert_eq!(reverse_idiomatic(xs), empty);
        assert_eq!(reverse_functional(xs), empty);
        assert_eq!(reverse_iter_fold(xs), empty);
    }

    #[test]
    fn test_fold_left_generic() {
        // Build a string: "((init+3)+1)+4"
        let xs = [3, 1, 4];
        let result = fold_left(|acc, x| format!("({acc}+{x})"), "init".to_string(), &xs);
        assert_eq!(result, "(((init+3)+1)+4)");
    }
}
