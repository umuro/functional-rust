#![allow(dead_code)]
//! `List.sort` — sort a list with a custom comparator.
//!
//! OCaml's `List.sort : ('a -> 'a -> int) -> 'a list -> 'a list` takes a
//! three-way comparator (negative / zero / positive) and returns a **new**
//! sorted list. Rust's slice API is a close match but differs in two ways:
//!
//!   * the comparator returns `std::cmp::Ordering` instead of an `int`,
//!   * slice `sort_by` sorts **in place** on an owned `Vec<T>`; to mirror
//!     OCaml's "return a new list" shape, we clone the input before sorting.
//!
//! Three translations are shown:
//!   * `sort_by_comparator` — direct OCaml parallel with a generic `Fn`,
//!   * `sort_by_key_fn` — the idiomatic Rust shortcut when the comparator
//!     reduces to "compare some key extracted from each element",
//!   * specialised helpers `sort_alphabetical` / `sort_by_length` that match
//!     the OCaml example exactly.

use std::cmp::Ordering;

/// Direct OCaml parallel: take a three-way comparator and return a new
/// sorted `Vec<T>`. The input slice is borrowed; the output is owned.
///
/// `T: Clone` is required because we copy the input elements into a fresh
/// `Vec` before sorting — OCaml's `List.sort` likewise returns a new list
/// without touching the original.
pub fn sort_by_comparator<T, F>(items: &[T], cmp: F) -> Vec<T>
where
    T: Clone,
    F: FnMut(&T, &T) -> Ordering,
{
    let mut out = items.to_vec();
    out.sort_by(cmp);
    out
}

/// Idiomatic shortcut when the comparator is "compare some key of each
/// element". `K: Ord` replaces the hand-written `Ordering`, and the key
/// function is evaluated once per element (sort is `O(n log n)` comparisons,
/// but key extraction is `O(n)`).
pub fn sort_by_key_fn<T, K, F>(items: &[T], key: F) -> Vec<T>
where
    T: Clone,
    K: Ord,
    F: FnMut(&T) -> K,
{
    let mut out = items.to_vec();
    out.sort_by_key(key);
    out
}

/// Sort strings alphabetically — the Rust parallel of
/// `List.sort String.compare words`.
///
/// `String::cmp` is lexicographic byte order; for ASCII this matches
/// `String.compare`. For unicode-aware locale sorting, use the
/// `unicode-collation` crate (out of scope here).
pub fn sort_alphabetical(words: &[String]) -> Vec<String> {
    sort_by_comparator(words, |a, b| a.cmp(b))
}

/// Sort strings by length — the Rust parallel of
/// `List.sort (fun a b -> compare (String.length a) (String.length b)) words`.
///
/// Uses `sort_by_key` rather than `sort_by` because the comparator is a
/// pure key extraction — the idiomatic Rust form.
pub fn sort_by_length(words: &[String]) -> Vec<String> {
    sort_by_key_fn(words, |s| s.len())
}

/// Stable sort by length, then alphabetically as a tie-break. Demonstrates
/// chained comparators — OCaml would write `compare (len a, a) (len b, b)`
/// using the lexicographic tuple compare.
pub fn sort_by_length_then_alpha(words: &[String]) -> Vec<String> {
    sort_by_comparator(words, |a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)))
}

/// Generic descending sort — wraps any comparator and reverses its result.
/// OCaml would write `fun a b -> compare b a`; Rust reverses the
/// `Ordering` via `.reverse()`.
pub fn sort_descending<T, F>(items: &[T], mut cmp: F) -> Vec<T>
where
    T: Clone,
    F: FnMut(&T, &T) -> Ordering,
{
    sort_by_comparator(items, move |a, b| cmp(a, b).reverse())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn words() -> Vec<String> {
        ["banana", "apple", "cherry", "date"]
            .iter()
            .map(|s| (*s).to_string())
            .collect()
    }

    // --- alphabetical -------------------------------------------------

    #[test]
    fn test_alphabetical_matches_ocaml() {
        assert_eq!(
            sort_alphabetical(&words()),
            vec!["apple", "banana", "cherry", "date"]
        );
    }

    #[test]
    fn test_alphabetical_empty() {
        let empty: Vec<String> = Vec::new();
        assert_eq!(sort_alphabetical(&empty), Vec::<String>::new());
    }

    #[test]
    fn test_alphabetical_single() {
        let one = vec!["solo".to_string()];
        assert_eq!(sort_alphabetical(&one), vec!["solo"]);
    }

    #[test]
    fn test_alphabetical_does_not_mutate_input() {
        let input = words();
        let _sorted = sort_alphabetical(&input);
        // input retains original order — sort returned a fresh Vec
        assert_eq!(
            input,
            vec!["banana", "apple", "cherry", "date"]
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>()
        );
    }

    // --- by length ----------------------------------------------------

    #[test]
    fn test_by_length_matches_ocaml() {
        // "date" (4) and "apple" (5) — "banana" and "cherry" both 6.
        let got = sort_by_length(&words());
        let lengths: Vec<usize> = got.iter().map(String::len).collect();
        assert_eq!(lengths, vec![4, 5, 6, 6]);
        // "date" must precede "apple" (strict length order)
        assert_eq!(got[0], "date");
        assert_eq!(got[1], "apple");
    }

    #[test]
    fn test_by_length_is_stable_for_ties() {
        // sort_by_key uses a stable sort — elements of equal length keep
        // their original relative order. "banana" appears before "cherry"
        // in the input, so it must stay before in the output.
        let got = sort_by_length(&words());
        assert_eq!(got[2], "banana");
        assert_eq!(got[3], "cherry");
    }

    // --- chained comparator -----------------------------------------

    #[test]
    fn test_length_then_alpha() {
        let got = sort_by_length_then_alpha(&words());
        // "banana" and "cherry" tie on length 6 — alpha tie-break puts
        // "banana" before "cherry" (same as stable sort in this case).
        assert_eq!(got, vec!["date", "apple", "banana", "cherry"]);
    }

    #[test]
    fn test_length_then_alpha_breaks_ties() {
        let input: Vec<String> = ["dog", "cat", "ant", "bee"]
            .iter()
            .map(|s| (*s).to_string())
            .collect();
        // all length 3 — falls through to alphabetical
        assert_eq!(
            sort_by_length_then_alpha(&input),
            vec!["ant", "bee", "cat", "dog"]
        );
    }

    // --- descending -------------------------------------------------

    #[test]
    fn test_descending_alphabetical() {
        let got = sort_descending(&words(), |a, b| a.cmp(b));
        assert_eq!(got, vec!["date", "cherry", "banana", "apple"]);
    }

    #[test]
    fn test_descending_by_length() {
        let got = sort_descending(&words(), |a, b| a.len().cmp(&b.len()));
        let lengths: Vec<usize> = got.iter().map(String::len).collect();
        assert_eq!(lengths, vec![6, 6, 5, 4]);
    }

    // --- generic sort_by_comparator on integers ---------------------

    #[test]
    fn test_generic_comparator_on_integers() {
        let xs = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let got = sort_by_comparator(&xs, i32::cmp);
        assert_eq!(got, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_generic_key_fn_on_integers_abs() {
        let xs = vec![-3, 1, -4, 2, -5];
        let got = sort_by_key_fn(&xs, |x: &i32| x.abs());
        assert_eq!(got, vec![1, 2, -3, -4, -5]);
    }
}
