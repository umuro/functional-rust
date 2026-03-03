//! # Option Type — Safe List Maximum
//!
//! OCaml's `option` type maps directly to Rust's `Option<T>`.
//! Both use `Some`/`None` variants to represent presence/absence of a value,
//! avoiding null pointer exceptions entirely.

// ---------------------------------------------------------------------------
// Approach A: Idiomatic Rust — iterator methods
// ---------------------------------------------------------------------------

/// Returns the maximum element, or `None` if the slice is empty.
///
/// `iter().max()` returns `Option<&T>` — we use `.copied()` to get `Option<T>`
/// for `Copy` types, avoiding the reference indirection.
pub fn list_max_idiomatic(xs: &[i32]) -> Option<i32> {
    xs.iter().copied().max()
}

/// Safe head — returns the first element or `None`.
///
/// Rust's slice method `.first()` returns `Option<&T>`.
pub fn safe_head_idiomatic(xs: &[i32]) -> Option<i32> {
    xs.first().copied()
}

/// Map over an Option — `Option::map` is built into Rust's stdlib.
pub fn double_max_idiomatic(xs: &[i32]) -> Option<i32> {
    list_max_idiomatic(xs).map(|x| x * 2)
}

// ---------------------------------------------------------------------------
// Approach B: Functional / recursive — mirrors OCaml closely
// ---------------------------------------------------------------------------

/// Recursive list_max mirroring OCaml's pattern matching version.
///
/// Uses slice patterns `[head, tail @ ..]` which correspond to
/// OCaml's `h :: t` destructuring.
pub fn list_max_recursive(xs: &[i32]) -> Option<i32> {
    match xs {
        [] => None,
        [head, tail @ ..] => match list_max_recursive(tail) {
            None => Some(*head),
            Some(m) => Some(if *head > m { *head } else { m }),
        },
    }
}

/// Safe head using slice pattern matching.
pub fn safe_head_recursive(xs: &[i32]) -> Option<i32> {
    match xs {
        [] => None,
        [h, ..] => Some(*h),
    }
}

/// Manual option_map mirroring OCaml's version.
///
/// In practice you'd always use `Option::map`, but this shows
/// the pattern matching equivalent.
#[allow(clippy::manual_map)] // Pedagogical: showing the pattern matching equivalent of Option::map
pub fn option_map<T, U>(f: impl FnOnce(T) -> U, opt: Option<T>) -> Option<U> {
    match opt {
        None => None,
        Some(x) => Some(f(x)),
    }
}

// ---------------------------------------------------------------------------
// Approach C: fold-based — using fold to find maximum
// ---------------------------------------------------------------------------

/// Maximum via fold — processes left to right with an accumulator.
pub fn list_max_fold(xs: &[i32]) -> Option<i32> {
    let (&first, rest) = xs.split_first()?;
    Some(rest.iter().fold(first, |acc, &x| acc.max(x)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_basic() {
        let xs = [3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(list_max_idiomatic(&xs), Some(9));
        assert_eq!(list_max_recursive(&xs), Some(9));
        assert_eq!(list_max_fold(&xs), Some(9));
    }

    #[test]
    fn test_max_empty() {
        let xs: &[i32] = &[];
        assert_eq!(list_max_idiomatic(xs), None);
        assert_eq!(list_max_recursive(xs), None);
        assert_eq!(list_max_fold(xs), None);
    }

    #[test]
    fn test_max_single() {
        assert_eq!(list_max_idiomatic(&[42]), Some(42));
        assert_eq!(list_max_recursive(&[42]), Some(42));
        assert_eq!(list_max_fold(&[42]), Some(42));
    }

    #[test]
    fn test_max_negative() {
        let xs = [-5, -1, -10, -3];
        assert_eq!(list_max_idiomatic(&xs), Some(-1));
        assert_eq!(list_max_recursive(&xs), Some(-1));
        assert_eq!(list_max_fold(&xs), Some(-1));
    }

    #[test]
    fn test_safe_head() {
        assert_eq!(safe_head_idiomatic(&[1, 2, 3]), Some(1));
        assert_eq!(safe_head_recursive(&[1, 2, 3]), Some(1));
        assert_eq!(safe_head_idiomatic(&[]), None);
        assert_eq!(safe_head_recursive(&[]), None);
    }

    #[test]
    fn test_double_max() {
        let xs = [3, 1, 4, 1, 5, 9, 2, 6];
        assert_eq!(double_max_idiomatic(&xs), Some(18));
    }

    #[test]
    fn test_double_max_empty() {
        assert_eq!(double_max_idiomatic(&[]), None);
    }

    #[test]
    fn test_option_map_manual() {
        assert_eq!(option_map(|x: i32| x * 2, Some(5)), Some(10));
        assert_eq!(option_map(|x: i32| x * 2, None), None);
    }
}
