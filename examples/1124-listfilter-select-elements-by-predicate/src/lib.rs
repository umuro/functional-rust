#![allow(dead_code)]
//! List.filter — Select Elements by Predicate
//! See example.ml for OCaml reference
//!
//! OCaml's `List.filter pred xs` keeps only the elements for which `pred` returns true.
//! Rust's `Iterator::filter` is the direct equivalent.

/// Idiomatic Rust: filter a slice using an iterator adapter.
/// Mirrors OCaml: `List.filter pred xs`
pub fn filter<T: Clone, F>(items: &[T], pred: F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    items.iter().filter(|x| pred(x)).cloned().collect()
}

/// Functional/recursive: keep elements matching pred by processing head and tail.
/// Mirrors OCaml pattern matching on the list spine.
pub fn filter_recursive<T: Clone, F>(items: &[T], pred: &F) -> Vec<T>
where
    F: Fn(&T) -> bool,
{
    match items {
        [] => vec![],
        [head, rest @ ..] => {
            let mut tail = filter_recursive(rest, pred);
            if pred(head) {
                // Prepend matching element to maintain original order.
                tail.insert(0, head.clone());
            }
            tail
        }
    }
}

/// Keep only even integers from a slice.
pub fn filter_evens(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|&&x| x % 2 == 0).copied().collect()
}

/// Keep only odd integers from a slice.
pub fn filter_odds(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|&&x| x % 2 != 0).copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_empty() {
        let empty: &[i32] = &[];
        assert_eq!(filter(empty, |_| true), vec![]);
        assert_eq!(filter(empty, |_| false), vec![]);
    }

    #[test]
    fn test_filter_single_match() {
        assert_eq!(filter(&[42_i32], |&x| x > 10), vec![42]);
    }

    #[test]
    fn test_filter_single_no_match() {
        assert_eq!(filter(&[3_i32], |&x| x > 10), vec![]);
    }

    #[test]
    fn test_filter_evens_and_odds() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8_i32];
        assert_eq!(filter_evens(&numbers), vec![2, 4, 6, 8]);
        assert_eq!(filter_odds(&numbers), vec![1, 3, 5, 7]);
    }

    #[test]
    fn test_filter_all_match() {
        let data = [1, 2, 3_i32];
        assert_eq!(filter(&data, |&x| x < 100), vec![1, 2, 3]);
    }

    #[test]
    fn test_filter_none_match() {
        let data = [10, 20, 30_i32];
        assert_eq!(filter(&data, |&x| x < 5), vec![]);
    }

    #[test]
    fn test_filter_recursive_matches_idiomatic() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8_i32];
        let pred = |x: &i32| x % 2 == 0;
        assert_eq!(filter(&numbers, pred), filter_recursive(&numbers, &pred));
    }

    #[test]
    fn test_filter_greater_than_threshold() {
        let data = [1, 2, 3, 4, 5_i32];
        assert_eq!(filter(&data, |&x| x > 3), vec![4, 5]);
    }
}
