#![allow(clippy::all)]
// Compute the length of a list. OCaml shows naive vs tail-recursive versions.
// In Rust, `.len()` is O(1) for slices/Vec — but we implement manual versions
// for educational comparison.

// ---------------------------------------------------------------------------
// Approach 1: Idiomatic Rust — .len() (O(1) for slices)
// ---------------------------------------------------------------------------
// Slices store their length alongside the pointer, so this is trivial.
pub fn length<T>(slice: &[T]) -> usize {
    slice.len()
}

// ---------------------------------------------------------------------------
// Approach 2: Fold — functional accumulator pattern
// ---------------------------------------------------------------------------
// Mirrors OCaml's tail-recursive `aux n = function` with an explicit fold.
pub fn length_fold<T>(slice: &[T]) -> usize {
    // fold is inherently iterative in Rust (no stack growth)
    slice.iter().fold(0, |acc, _| acc + 1)
}

// ---------------------------------------------------------------------------
// Approach 3: Recursive — mirrors OCaml's naive version
// ---------------------------------------------------------------------------
// Educational only: Rust doesn't guarantee TCO, and slices make this awkward.
pub fn length_recursive<T>(slice: &[T]) -> usize {
    match slice {
        [] => 0,
        [_, rest @ ..] => 1 + length_recursive(rest),
    }
}

// ---------------------------------------------------------------------------
// Approach 4: Tail-recursive style — mirrors OCaml's production version
// ---------------------------------------------------------------------------
pub fn length_tail<T>(slice: &[T]) -> usize {
    fn aux<T>(n: usize, slice: &[T]) -> usize {
        match slice {
            [] => n,
            [_, rest @ ..] => aux(n + 1, rest),
        }
    }
    aux(0, slice)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(length::<i32>(&[]), 0);
        assert_eq!(length_fold::<i32>(&[]), 0);
        assert_eq!(length_recursive::<i32>(&[]), 0);
        assert_eq!(length_tail::<i32>(&[]), 0);
    }

    #[test]
    fn test_basic() {
        let v = [1, 2, 3, 4];
        assert_eq!(length(&v), 4);
        assert_eq!(length_fold(&v), 4);
        assert_eq!(length_recursive(&v), 4);
        assert_eq!(length_tail(&v), 4);
    }

    #[test]
    fn test_single() {
        assert_eq!(length(&[42]), 1);
        assert_eq!(length_fold(&[42]), 1);
    }

    #[test]
    fn test_large() {
        let v: Vec<i32> = (0..10000).collect();
        assert_eq!(length(&v), 10000);
        assert_eq!(length_fold(&v), 10000);
        // Skip recursive for large — may stack overflow
    }

    #[test]
    fn test_strings() {
        let v = ["hello", "world", "foo"];
        assert_eq!(length(&v), 3);
    }
}
