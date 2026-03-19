#![allow(clippy::all)]
//! # List Fold Left
//!
//! Demonstrates reducing a list to a single value using left-to-right folding.
//! This example shows both iterator-based and recursive approaches in Rust.

/// Fold left using iterators (idiomatic Rust approach).
///
/// Applies a binary operation cumulatively from left to right,
/// starting with an initial accumulator value.
///
/// # Arguments
/// * `init` - Initial accumulator value
/// * `items` - Slice of items to fold
/// * `f` - Binary operation: (accumulator, item) -> accumulator
///
/// # Example
/// ```
/// use list_fold_left::fold_left_iter;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let sum = fold_left_iter(0, &numbers, |acc, x| acc + x);
/// assert_eq!(sum, 15);
/// ```
pub fn fold_left_iter<T, U, F>(init: U, items: &[T], f: F) -> U
where
    F: Fn(U, &T) -> U,
{
    items.iter().fold(init, f)
}

/// Fold left using recursion (functional style, like OCaml).
///
/// Tail-recursive implementation that manually processes the list.
/// The compiler may optimize this into a loop.
///
/// # Arguments
/// * `init` - Initial accumulator value
/// * `items` - Slice of items to fold
/// * `f` - Binary operation: (accumulator, item) -> accumulator
///
/// # Example
/// ```
/// use list_fold_left::fold_left_recursive;
///
/// let numbers = vec![1, 2, 3, 4, 5];
/// let sum = fold_left_recursive(0, &numbers, |acc, x| acc + x);
/// assert_eq!(sum, 15);
/// ```
pub fn fold_left_recursive<T, U, F>(init: U, items: &[T], f: F) -> U
where
    F: Fn(U, &T) -> U,
{
    match items {
        [] => init,
        [head, tail @ ..] => fold_left_recursive(f(init, head), tail, f),
    }
}

/// Calculate the sum of a list.
///
/// # Example
/// ```
/// use list_fold_left::sum;
/// assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
/// ```
pub fn sum(items: &[i32]) -> i32 {
    fold_left_iter(0, items, |acc, x| acc + x)
}

/// Calculate the product of a list.
///
/// # Example
/// ```
/// use list_fold_left::product;
/// assert_eq!(product(&[1, 2, 3, 4, 5]), 120);
/// ```
pub fn product(items: &[i32]) -> i32 {
    fold_left_iter(1, items, |acc, x| acc * x)
}

/// Find the maximum value in a list.
///
/// Returns `i32::MIN` if the list is empty (matching OCaml's min_int).
///
/// # Example
/// ```
/// use list_fold_left::max_value;
/// assert_eq!(max_value(&[1, 5, 3, 2, 4]), 5);
/// ```
pub fn max_value(items: &[i32]) -> i32 {
    fold_left_iter(i32::MIN, items, |acc, x| if x > &acc { *x } else { acc })
}

/// Find the minimum value in a list.
///
/// Returns `i32::MAX` if the list is empty.
///
/// # Example
/// ```
/// use list_fold_left::min_value;
/// assert_eq!(min_value(&[1, 5, 3, 2, 4]), 1);
/// ```
pub fn min_value(items: &[i32]) -> i32 {
    fold_left_iter(i32::MAX, items, |acc, x| if x < &acc { *x } else { acc })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_left_iter_sum() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(fold_left_iter(0, &numbers, |acc, x| acc + x), 15);
    }

    #[test]
    fn test_fold_left_iter_product() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(fold_left_iter(1, &numbers, |acc, x| acc * x), 120);
    }

    #[test]
    fn test_fold_left_iter_max() {
        let numbers = vec![1, 5, 3, 2, 4];
        assert_eq!(
            fold_left_iter(i32::MIN, &numbers, |acc, x| if x > &acc { *x } else { acc }),
            5
        );
    }

    #[test]
    fn test_fold_left_iter_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(fold_left_iter(0, &empty, |acc, x| acc + x), 0);
        assert_eq!(fold_left_iter(1, &empty, |acc, x| acc * x), 1);
    }

    #[test]
    fn test_fold_left_iter_single() {
        let single = vec![42];
        assert_eq!(fold_left_iter(0, &single, |acc, x| acc + x), 42);
        assert_eq!(fold_left_iter(1, &single, |acc, x| acc * x), 42);
    }

    #[test]
    fn test_fold_left_recursive_sum() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(fold_left_recursive(0, &numbers, |acc, x| acc + x), 15);
    }

    #[test]
    fn test_fold_left_recursive_product() {
        let numbers = vec![1, 2, 3, 4, 5];
        assert_eq!(fold_left_recursive(1, &numbers, |acc, x| acc * x), 120);
    }

    #[test]
    fn test_fold_left_recursive_max() {
        let numbers = vec![1, 5, 3, 2, 4];
        assert_eq!(
            fold_left_recursive(i32::MIN, &numbers, |acc, x| if x > &acc { *x } else { acc }),
            5
        );
    }

    #[test]
    fn test_fold_left_recursive_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(fold_left_recursive(0, &empty, |acc, x| acc + x), 0);
    }

    #[test]
    fn test_fold_left_recursive_single() {
        let single = vec![42];
        assert_eq!(fold_left_recursive(0, &single, |acc, x| acc + x), 42);
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum(&[]), 0);
        assert_eq!(sum(&[42]), 42);
        assert_eq!(sum(&[-5, 10, -3]), 2);
    }

    #[test]
    fn test_product() {
        assert_eq!(product(&[1, 2, 3, 4, 5]), 120);
        assert_eq!(product(&[]), 1);
        assert_eq!(product(&[42]), 42);
        assert_eq!(product(&[2, 3, 4]), 24);
    }

    #[test]
    fn test_max_value() {
        assert_eq!(max_value(&[1, 5, 3, 2, 4]), 5);
        assert_eq!(max_value(&[]), i32::MIN);
        assert_eq!(max_value(&[42]), 42);
        assert_eq!(max_value(&[-5, -1, -10]), -1);
    }

    #[test]
    fn test_min_value() {
        assert_eq!(min_value(&[1, 5, 3, 2, 4]), 1);
        assert_eq!(min_value(&[]), i32::MAX);
        assert_eq!(min_value(&[42]), 42);
        assert_eq!(min_value(&[-5, -1, -10]), -10);
    }

    #[test]
    fn test_fold_left_iter_string_concat() {
        let words = vec!["Hello", " ", "World"];
        let result = fold_left_iter(String::new(), &words, |acc, word| acc + word);
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_fold_left_recursive_string_concat() {
        let words = vec!["Hello", " ", "World"];
        let result = fold_left_recursive(String::new(), &words, |acc, word| acc + word);
        assert_eq!(result, "Hello World");
    }
}
