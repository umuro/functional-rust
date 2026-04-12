#![allow(dead_code)]
//! List.sort — Sort with Custom Comparator
//! See example.ml for OCaml reference
//!
//! OCaml's `List.sort cmp xs` sorts using a comparison function that returns negative/zero/positive.
//! Rust's `slice::sort_by` takes an `Ordering`-returning comparator.

/// Sort a slice of strings lexicographically (ascending).
/// Mirrors OCaml: `List.sort String.compare words`
pub fn sort_strings<'a>(words: &[&'a str]) -> Vec<&'a str> {
    let mut result = words.to_vec();
    result.sort();
    result
}

/// Sort by string length (ascending), ties broken by lexicographic order.
/// Mirrors OCaml: `List.sort (fun a b -> compare (String.length a) (String.length b)) words`
pub fn sort_by_length<'a>(words: &[&'a str]) -> Vec<&'a str> {
    let mut result = words.to_vec();
    result.sort_by(|a, b| a.len().cmp(&b.len()).then(a.cmp(b)));
    result
}

/// Sort in descending lexicographic order.
/// Mirrors OCaml: `List.sort (fun a b -> String.compare b a) words`
pub fn sort_descending<'a>(words: &[&'a str]) -> Vec<&'a str> {
    let mut result = words.to_vec();
    result.sort_by(|a, b| b.cmp(a));
    result
}

/// Generic sort with a custom comparator — mirrors OCaml's `List.sort f xs`.
pub fn sort_with<T: Clone, F>(items: &[T], compare: F) -> Vec<T>
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let mut result = items.to_vec();
    result.sort_by(compare);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_strings() {
        let words = vec!["banana", "apple", "cherry", "date"];
        assert_eq!(sort_strings(&words), vec!["apple", "banana", "cherry", "date"]);
    }

    #[test]
    fn test_sort_by_length() {
        let words = vec!["banana", "apple", "cherry", "date"];
        // "date"(4), "apple"(5), "banana"(6), "cherry"(6) — ties broken alphabetically
        assert_eq!(sort_by_length(&words), vec!["date", "apple", "banana", "cherry"]);
    }

    #[test]
    fn test_sort_descending() {
        let words = vec!["banana", "apple", "cherry", "date"];
        assert_eq!(sort_descending(&words), vec!["date", "cherry", "banana", "apple"]);
    }

    #[test]
    fn test_sort_empty() {
        let empty: Vec<&str> = vec![];
        assert_eq!(sort_strings(&empty), Vec::<&str>::new());
    }

    #[test]
    fn test_sort_with_custom() {
        let nums = vec![3i32, 1, 4, 1, 5, 9, 2, 6];
        let sorted = sort_with(&nums, |a, b| a.cmp(b));
        assert_eq!(sorted, vec![1, 1, 2, 3, 4, 5, 6, 9]);
    }

    #[test]
    fn test_sort_with_reverse() {
        let nums = vec![3i32, 1, 4, 1, 5];
        let sorted = sort_with(&nums, |a, b| b.cmp(a));
        assert_eq!(sorted, vec![5, 4, 3, 1, 1]);
    }
}
