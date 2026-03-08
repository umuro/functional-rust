//! List filtering: keep only elements satisfying a condition.
//!
//! This module demonstrates two approaches to filtering lists in Rust:
//! 1. **Idiomatic Rust** using iterators (lazy, composable, zero-copy)
//! 2. **Recursive** functional style (mimics OCaml's List.filter)

/// Filter a vector using iterators (idiomatic Rust).
///
/// Returns a new Vec containing only elements where the predicate returns true.
/// Uses the built-in `filter()` method for efficiency.
///
/// # Example
/// ```
/// use list_filter::filter_iter;
/// let numbers = vec![1, 2, 3, 4, 5];
/// let evens = filter_iter(&numbers, |x| x % 2 == 0);
/// assert_eq!(evens, vec![2, 4]);
/// ```
pub fn filter_iter<T: Clone>(items: &[T], predicate: impl Fn(&T) -> bool) -> Vec<T> {
    items.iter().filter(|x| predicate(x)).cloned().collect()
}

/// Filter a slice in-place using iterators.
///
/// Retains only elements where the predicate returns true.
/// Modifies the original vector.
///
/// # Example
/// ```
/// use list_filter::filter_in_place;
/// let mut numbers = vec![1, 2, 3, 4, 5];
/// filter_in_place(&mut numbers, |x| x % 2 == 0);
/// assert_eq!(numbers, vec![2, 4]);
/// ```
pub fn filter_in_place<T>(items: &mut Vec<T>, predicate: impl Fn(&T) -> bool) {
    items.retain(|x| predicate(x));
}

/// Filter a vector using recursion (functional, OCaml-style).
///
/// Returns a new Vec by recursively filtering the input.
/// Demonstrates functional programming idioms in Rust.
///
/// # Example
/// ```
/// use list_filter::filter_recursive;
/// let numbers = vec![1, 2, 3, 4, 5];
/// let odds = filter_recursive(&numbers, |x| x % 2 != 0);
/// assert_eq!(odds, vec![1, 3, 5]);
/// ```
pub fn filter_recursive<T: Clone>(items: &[T], predicate: impl Fn(&T) -> bool + Copy) -> Vec<T> {
    match items {
        [] => Vec::new(),
        [head, tail @ ..] => {
            let mut rest = filter_recursive(tail, predicate);
            if predicate(head) {
                let mut result = vec![head.clone()];
                result.append(&mut rest);
                result
            } else {
                rest
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_iter_multiple_elements() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let evens = filter_iter(&numbers, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_filter_iter_odds() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let odds = filter_iter(&numbers, |x| x % 2 != 0);
        assert_eq!(odds, vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_filter_iter_empty() {
        let numbers: Vec<i32> = Vec::new();
        let result = filter_iter(&numbers, |x| x % 2 == 0);
        assert_eq!(result, Vec::new());
    }

    #[test]
    fn test_filter_iter_single_element_matches() {
        let numbers = vec![4];
        let result = filter_iter(&numbers, |x| x % 2 == 0);
        assert_eq!(result, vec![4]);
    }

    #[test]
    fn test_filter_iter_single_element_no_match() {
        let numbers = vec![3];
        let result = filter_iter(&numbers, |x| x % 2 == 0);
        assert_eq!(result, Vec::new());
    }

    #[test]
    fn test_filter_iter_all_match() {
        let numbers = vec![2, 4, 6, 8];
        let result = filter_iter(&numbers, |x| x % 2 == 0);
        assert_eq!(result, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_filter_iter_none_match() {
        let numbers = vec![1, 3, 5, 7];
        let result = filter_iter(&numbers, |x| x % 2 == 0);
        assert_eq!(result, Vec::new());
    }

    #[test]
    fn test_filter_in_place_multiple_elements() {
        let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
        filter_in_place(&mut numbers, |x| x % 2 == 0);
        assert_eq!(numbers, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_filter_in_place_empty() {
        let mut numbers: Vec<i32> = Vec::new();
        filter_in_place(&mut numbers, |x| x % 2 == 0);
        assert_eq!(numbers, Vec::new());
    }

    #[test]
    fn test_filter_in_place_single_match() {
        let mut numbers = vec![4];
        filter_in_place(&mut numbers, |x| x % 2 == 0);
        assert_eq!(numbers, vec![4]);
    }

    #[test]
    fn test_filter_in_place_single_no_match() {
        let mut numbers = vec![3];
        filter_in_place(&mut numbers, |x| x % 2 == 0);
        assert_eq!(numbers, Vec::new());
    }

    #[test]
    fn test_filter_recursive_multiple_elements() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let evens = filter_recursive(&numbers, |x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_filter_recursive_odds() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let odds = filter_recursive(&numbers, |x| x % 2 != 0);
        assert_eq!(odds, vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_filter_recursive_empty() {
        let numbers: Vec<i32> = Vec::new();
        let result = filter_recursive(&numbers, |x| x % 2 == 0);
        assert_eq!(result, Vec::new());
    }

    #[test]
    fn test_filter_recursive_single_element_matches() {
        let numbers = vec![4];
        let result = filter_recursive(&numbers, |x| x % 2 == 0);
        assert_eq!(result, vec![4]);
    }

    #[test]
    fn test_filter_recursive_single_element_no_match() {
        let numbers = vec![3];
        let result = filter_recursive(&numbers, |x| x % 2 == 0);
        assert_eq!(result, Vec::new());
    }

    #[test]
    fn test_filter_recursive_all_match() {
        let numbers = vec![2, 4, 6, 8];
        let result = filter_recursive(&numbers, |x| x % 2 == 0);
        assert_eq!(result, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_filter_recursive_none_match() {
        let numbers = vec![1, 3, 5, 7];
        let result = filter_recursive(&numbers, |x| x % 2 == 0);
        assert_eq!(result, Vec::new());
    }

    #[test]
    fn test_filter_iter_with_strings() {
        let words = vec!["hello", "world", "a", "rust"];
        let long_words = filter_iter(&words, |w| w.len() > 1);
        assert_eq!(long_words, vec!["hello", "world", "rust"]);
    }

    #[test]
    fn test_filter_recursive_with_strings() {
        let words = vec!["hello", "world", "a", "rust"];
        let long_words = filter_recursive(&words, |w| w.len() > 1);
        assert_eq!(long_words, vec!["hello", "world", "rust"]);
    }

    #[test]
    fn test_filter_iter_and_recursive_equivalent() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let evens_iter = filter_iter(&numbers, |x| x % 2 == 0);
        let evens_recursive = filter_recursive(&numbers, |x| x % 2 == 0);
        assert_eq!(evens_iter, evens_recursive);
    }

    #[test]
    fn test_filter_with_complex_predicate() {
        let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let result = filter_iter(&numbers, |x| x > &3 && x < &8);
        assert_eq!(result, vec![4, 5, 6, 7]);
    }
}
