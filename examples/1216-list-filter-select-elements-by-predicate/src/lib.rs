#![allow(dead_code)]
//! `List.filter` — select elements from a list that satisfy a predicate.
//!
//! Shows three Rust translations of OCaml's `List.filter`:
//!   * idiomatic iterator chain,
//!   * explicit pattern-matched recursion (closest to OCaml),
//!   * left fold over a slice.
//!
//! The predicate takes `&T` so we can filter without consuming or cloning
//! until the matched elements are collected into a new `Vec<T>`.

/// Idiomatic Rust: `iter().filter().copied().collect()`.
///
/// Borrows the slice, applies the predicate via reference, and materialises
/// the survivors into a fresh `Vec<T>`.  `T: Copy` keeps the translation
/// honest to OCaml, where `int list` values are trivially duplicated.
pub fn filter<T, F>(predicate: F, items: &[T]) -> Vec<T>
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    items.iter().copied().filter(|x| predicate(x)).collect()
}

/// Functional / recursive translation — the direct OCaml parallel.
/// Matches on the slice shape `[] | [h, rest @ ..]`, mirroring
/// `let rec filter p = function [] -> [] | h::t -> ...`.
pub fn filter_recursive<T, F>(predicate: &F, items: &[T]) -> Vec<T>
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    match items {
        [] => Vec::new(),
        [h, rest @ ..] => {
            let mut tail = filter_recursive(predicate, rest);
            if predicate(h) {
                let mut out = Vec::with_capacity(1 + tail.len());
                out.push(*h);
                out.append(&mut tail);
                out
            } else {
                tail
            }
        }
    }
}

/// Fold-based translation — accumulates survivors in a `Vec`.
/// Equivalent to `List.fold_left (fun acc x -> if p x then acc @ [x] else acc) []`,
/// but avoids the quadratic `@` by pushing in place.
pub fn filter_fold<T, F>(predicate: F, items: &[T]) -> Vec<T>
where
    T: Copy,
    F: Fn(&T) -> bool,
{
    items.iter().fold(Vec::new(), |mut acc, x| {
        if predicate(x) {
            acc.push(*x);
        }
        acc
    })
}

// --- Concrete predicates matching the OCaml example ---------------------

pub fn is_even(x: &i32) -> bool {
    x % 2 == 0
}

pub fn is_odd(x: &i32) -> bool {
    x % 2 != 0
}

pub fn is_positive(x: &i32) -> bool {
    *x > 0
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUMBERS: &[i32] = &[1, 2, 3, 4, 5, 6, 7, 8];

    #[test]
    fn test_filter_empty() {
        let empty: [i32; 0] = [];
        assert_eq!(filter(is_even, &empty), Vec::<i32>::new());
    }

    #[test]
    fn test_filter_single_kept() {
        assert_eq!(filter(is_even, &[4]), vec![4]);
    }

    #[test]
    fn test_filter_single_dropped() {
        assert_eq!(filter(is_even, &[3]), Vec::<i32>::new());
    }

    #[test]
    fn test_filter_evens() {
        assert_eq!(filter(is_even, NUMBERS), vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_filter_odds() {
        assert_eq!(filter(is_odd, NUMBERS), vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_filter_positive_with_mixed() {
        let mixed = [-2, -1, 0, 1, 2, 3];
        assert_eq!(filter(is_positive, &mixed), vec![1, 2, 3]);
    }

    #[test]
    fn test_filter_closure_predicate() {
        let big = filter(|x: &i32| *x > 5, NUMBERS);
        assert_eq!(big, vec![6, 7, 8]);
    }

    #[test]
    fn test_filter_recursive_evens() {
        assert_eq!(filter_recursive(&is_even, NUMBERS), vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_filter_recursive_empty() {
        let empty: [i32; 0] = [];
        assert_eq!(filter_recursive(&is_even, &empty), Vec::<i32>::new());
    }

    #[test]
    fn test_filter_fold_odds() {
        assert_eq!(filter_fold(is_odd, NUMBERS), vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_filter_fold_none_match() {
        assert_eq!(filter_fold(|x: &i32| *x > 100, NUMBERS), Vec::<i32>::new());
    }

    #[test]
    fn test_all_implementations_agree() {
        let mixed = [-3, -2, -1, 0, 1, 2, 3, 4];
        let a = filter(is_positive, &mixed);
        let b = filter_recursive(&is_positive, &mixed);
        let c = filter_fold(is_positive, &mixed);
        assert_eq!(a, b);
        assert_eq!(b, c);
        assert_eq!(a, vec![1, 2, 3, 4]);
    }
}
