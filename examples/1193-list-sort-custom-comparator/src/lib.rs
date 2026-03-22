#![allow(dead_code)]
//! List.sort — Sort with Custom Comparator
//! See example.ml for OCaml reference
//!
//! OCaml's `List.sort cmp xs` sorts using a comparison function.
//! Rust's `slice::sort_by` is the direct equivalent, returning `Ordering` instead of int.

use std::cmp::Ordering;

/// Sort a slice of strings lexicographically (ascending).
/// Mirrors OCaml: `List.sort String.compare words`
pub fn sort_strings<'a>(words: &[&'a str]) -> Vec<&'a str> {
    let mut result = words.to_vec();
    result.sort();
    result
}

/// Sort strings by length (ascending), ties broken alphabetically.
/// Mirrors OCaml: `List.sort (fun a b -> compare (String.length a) (String.length b)) words`
pub fn sort_by_length<'a>(words: &[&'a str]) -> Vec<&'a str> {
    let mut result = words.to_vec();
    // `.then()` chains a secondary comparator when the primary returns Equal.
    result.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    result
}

/// Sort strings in descending (reverse) lexicographic order.
/// Mirrors OCaml: `List.sort (fun a b -> String.compare b a) words`
pub fn sort_descending<'a>(words: &[&'a str]) -> Vec<&'a str> {
    let mut result = words.to_vec();
    result.sort_by(|a, b| b.cmp(a));
    result
}

/// Generic sort with a custom comparator — mirrors OCaml's `List.sort`.
/// Clones the input first because `sort_by` sorts in place; OCaml returns a new list.
pub fn sort_with<T: Clone, F>(items: &[T], cmp: F) -> Vec<T>
where
    F: Fn(&T, &T) -> Ordering,
{
    let mut result = items.to_vec();
    result.sort_by(|a, b| cmp(a, b));
    result
}

/// Functional/recursive: merge sort mirroring a purely recursive OCaml approach.
pub fn merge_sort<T: Clone + Ord>(items: &[T]) -> Vec<T> {
    if items.len() <= 1 {
        return items.to_vec();
    }
    let mid = items.len() / 2;
    let left = merge_sort(&items[..mid]);
    let right = merge_sort(&items[mid..]);
    merge(left, right)
}

fn merge<T: Ord>(mut left: Vec<T>, mut right: Vec<T>) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    // Drain from the back to avoid O(n) removes from the front.
    left.reverse();
    right.reverse();
    while !left.is_empty() && !right.is_empty() {
        if left.last() <= right.last() {
            result.push(left.pop().unwrap());
        } else {
            result.push(right.pop().unwrap());
        }
    }
    result.extend(left.into_iter().rev());
    result.extend(right.into_iter().rev());
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_strings_empty() {
        let words: &[&str] = &[];
        assert_eq!(sort_strings(words), Vec::<&str>::new());
    }

    #[test]
    fn test_sort_strings_single() {
        assert_eq!(sort_strings(&["only"]), vec!["only"]);
    }

    #[test]
    fn test_sort_strings_lexicographic() {
        let words = ["banana", "apple", "cherry", "date"];
        assert_eq!(
            sort_strings(&words),
            vec!["apple", "banana", "cherry", "date"]
        );
    }

    #[test]
    fn test_sort_by_length() {
        let words = ["banana", "apple", "cherry", "date"];
        // "date"(4) < "apple"(5) < "banana"(6) = "cherry"(6)
        // tie broken alphabetically: "banana" < "cherry"
        assert_eq!(
            sort_by_length(&words),
            vec!["date", "apple", "banana", "cherry"]
        );
    }

    #[test]
    fn test_sort_descending() {
        let words = ["banana", "apple", "cherry", "date"];
        assert_eq!(
            sort_descending(&words),
            vec!["date", "cherry", "banana", "apple"]
        );
    }

    #[test]
    fn test_sort_with_custom_i32() {
        let numbers = [5, 1, 4, 2, 3_i32];
        assert_eq!(
            sort_with(&numbers, |a, b| a.cmp(b)),
            vec![1, 2, 3, 4, 5]
        );
    }

    #[test]
    fn test_merge_sort_empty() {
        let empty: &[i32] = &[];
        assert_eq!(merge_sort(empty), vec![]);
    }

    #[test]
    fn test_merge_sort_multiple() {
        let numbers = [3, 1, 4, 1, 5, 9, 2, 6_i32];
        assert_eq!(merge_sort(&numbers), vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_merge_sort_matches_std_sort() {
        let mut std_sorted = vec![5_i32, 2, 8, 1, 9, 3];
        std_sorted.sort();
        assert_eq!(merge_sort(&[5, 2, 8, 1, 9, 3]), std_sorted);
    }
}
