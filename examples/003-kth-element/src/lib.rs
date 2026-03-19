#![allow(clippy::all)]
// Find the k-th element of a list. The OCaml version uses 1-based indexing.
// We provide both 1-based (matching OCaml) and 0-based (idiomatic Rust) versions.

// ---------------------------------------------------------------------------
// Approach 1: Idiomatic Rust — direct indexing (0-based)
// ---------------------------------------------------------------------------
// Slices support `.get(index)` which returns `Option<&T>` — safe, no panic.
pub fn at<T>(slice: &[T], index: usize) -> Option<&T> {
    // .get() does bounds checking and returns None if out of range
    slice.get(index)
}

// ---------------------------------------------------------------------------
// Approach 2: 1-based indexing (matches OCaml semantics)
// ---------------------------------------------------------------------------
// OCaml's `at k list` uses 1-based indexing. We replicate that here.
pub fn at_one_based<T>(slice: &[T], k: usize) -> Option<&T> {
    if k == 0 {
        None // 1-based: 0 is invalid
    } else {
        slice.get(k - 1)
    }
}

// ---------------------------------------------------------------------------
// Approach 3: Functional — recursive, mirrors OCaml's pattern matching
// ---------------------------------------------------------------------------
// Uses 1-based indexing like the OCaml version.
// Recursion on slices: `[h, rest @ ..]` destructures head and tail.
pub fn at_recursive<T>(slice: &[T], k: usize) -> Option<&T> {
    match (slice, k) {
        ([], _) => None,
        ([h, ..], 1) => Some(h),
        ([_, rest @ ..], k) => at_recursive(rest, k - 1),
    }
}

// ---------------------------------------------------------------------------
// Approach 4: Iterator-based
// ---------------------------------------------------------------------------
pub fn at_iter<T>(slice: &[T], index: usize) -> Option<&T> {
    // .get() is idiomatic for slices; .iter().nth() would also work but clippy
    // prefers direct indexing on slices.
    slice.get(index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let v = [1, 2, 3, 4, 5];
        assert_eq!(at(&v, 2), Some(&3)); // 0-based: index 2 = third element
        assert_eq!(at_one_based(&v, 3), Some(&3)); // 1-based: k=3
        assert_eq!(at_recursive(&v, 3), Some(&3)); // 1-based
        assert_eq!(at_iter(&v, 2), Some(&3)); // 0-based
    }

    #[test]
    fn test_first() {
        let v = [1, 2, 3];
        assert_eq!(at(&v, 0), Some(&1));
        assert_eq!(at_one_based(&v, 1), Some(&1));
        assert_eq!(at_recursive(&v, 1), Some(&1));
    }

    #[test]
    fn test_out_of_bounds() {
        let v = [1, 2, 3];
        assert_eq!(at(&v, 10), None);
        assert_eq!(at_one_based(&v, 10), None);
        assert_eq!(at_recursive(&v, 10), None);
    }

    #[test]
    fn test_empty() {
        assert_eq!(at::<i32>(&[], 0), None);
        assert_eq!(at_one_based::<i32>(&[], 1), None);
        assert_eq!(at_recursive::<i32>(&[], 1), None);
    }

    #[test]
    fn test_one_based_zero() {
        let v = [1, 2, 3];
        assert_eq!(at_one_based(&v, 0), None); // 0 is invalid for 1-based
    }
}
