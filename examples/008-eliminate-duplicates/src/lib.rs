#![allow(clippy::all)]
//! # Eliminate Consecutive Duplicates
//! OCaml 99 Problems #8 — Remove consecutive duplicate elements from a list.

/// Idiomatic Rust: use `dedup` on a mutable Vec (in-place, O(n)).
/// This modifies the vector — Rust's ownership model makes mutation explicit.
pub fn compress_mut<T: PartialEq>(list: &mut Vec<T>) {
    list.dedup();
}

/// Functional style: build a new Vec by filtering consecutive duplicates.
/// Mirrors the OCaml recursive approach but uses iterators.
pub fn compress<T: PartialEq + Clone>(list: &[T]) -> Vec<T> {
    if list.is_empty() {
        return vec![];
    }
    let mut result = vec![list[0].clone()];
    for item in &list[1..] {
        if result.last() != Some(item) {
            result.push(item.clone());
        }
    }
    result
}

/// Iterator-based with windows — the most declarative approach.
/// Uses `windows(2)` to compare adjacent pairs.
pub fn compress_iter<T: PartialEq + Clone>(list: &[T]) -> Vec<T> {
    if list.is_empty() {
        return vec![];
    }
    let mut result: Vec<T> = list
        .windows(2)
        .filter(|w| w[0] != w[1])
        .map(|w| w[0].clone())
        .collect();
    // Always include the last element
    if let Some(last) = list.last() {
        result.push(last.clone());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consecutive_duplicates() {
        let input = vec!["a", "a", "a", "b", "c", "c", "d", "e", "e", "e"];
        assert_eq!(compress(&input), vec!["a", "b", "c", "d", "e"]);
        assert_eq!(compress_iter(&input), vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn test_empty() {
        assert_eq!(compress::<i32>(&[]), Vec::<i32>::new());
        assert_eq!(compress_iter::<i32>(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_single() {
        assert_eq!(compress(&[1]), vec![1]);
        assert_eq!(compress_iter(&[1]), vec![1]);
    }

    #[test]
    fn test_no_duplicates() {
        assert_eq!(compress(&[1, 2, 3, 4]), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(compress(&[5, 5, 5, 5]), vec![5]);
    }

    #[test]
    fn test_mut_version() {
        let mut v = vec![1, 1, 2, 2, 3];
        compress_mut(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }
}
