#![allow(clippy::all)]
//! List.partition — Split by Predicate
//! See example.ml for OCaml reference
//!
//! OCaml's `List.partition pred xs` returns `(matches, non_matches)` in a single pass.
//! Rust's `Iterator::partition` does the same, collecting into two separate `Vec`s.

/// Split a slice into two vecs: elements satisfying `pred` and those that don't.
/// Mirrors OCaml: `List.partition (fun x -> x <= 5) numbers`
pub fn partition<T: Clone, F>(items: &[T], pred: F) -> (Vec<T>, Vec<T>)
where
    F: Fn(&T) -> bool,
{
    items.iter().cloned().partition(|x| pred(x))
}

/// Partition integers into (small, big) where small means `<= threshold`.
pub fn partition_threshold(numbers: &[i32], threshold: i32) -> (Vec<i32>, Vec<i32>) {
    partition(numbers, |&x| x <= threshold)
}

/// Partition strings by length: (short, long) where short means `len <= max_len`.
pub fn partition_by_length<'a>(words: &[&'a str], max_len: usize) -> (Vec<&'a str>, Vec<&'a str>) {
    partition(words, |s| s.len() <= max_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_numbers() {
        let numbers: Vec<i32> = (1..=10).collect();
        let (small, big) = partition_threshold(&numbers, 5);
        assert_eq!(small, vec![1, 2, 3, 4, 5]);
        assert_eq!(big, vec![6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_partition_empty() {
        let (small, big) = partition_threshold(&[], 5);
        assert!(small.is_empty());
        assert!(big.is_empty());
    }

    #[test]
    fn test_partition_all_match() {
        let nums = vec![1i32, 2, 3];
        let (small, big) = partition_threshold(&nums, 10);
        assert_eq!(small, vec![1, 2, 3]);
        assert!(big.is_empty());
    }

    #[test]
    fn test_partition_none_match() {
        let nums = vec![6i32, 7, 8];
        let (small, big) = partition_threshold(&nums, 5);
        assert!(small.is_empty());
        assert_eq!(big, vec![6, 7, 8]);
    }

    #[test]
    fn test_partition_by_length() {
        let words = vec!["hi", "hello", "ok", "world", "rust"];
        let (short, long) = partition_by_length(&words, 3);
        assert_eq!(short, vec!["hi", "ok"]);
        assert_eq!(long, vec!["hello", "world", "rust"]);
    }

    #[test]
    fn test_partition_evens_odds() {
        let nums: Vec<i32> = (1..=6).collect();
        let (evens, odds) = partition(&nums, |&x| x % 2 == 0);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }
}
