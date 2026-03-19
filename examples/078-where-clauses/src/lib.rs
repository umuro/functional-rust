#![allow(clippy::all)]
// 078: Where Clauses
// Complex trait bounds using where syntax

use std::fmt::Display;
use std::ops::{Add, Mul};

// Approach 1: Where clause for readability
fn print_if_equal<T>(a: &T, b: &T) -> String
where
    T: Display + PartialEq,
{
    if a == b {
        format!("{} == {}", a, b)
    } else {
        format!("{} != {}", a, b)
    }
}

// Approach 2: Multiple type params with where
fn zip_with<A, B, C, F>(a: &[A], b: &[B], f: F) -> Vec<C>
where
    A: Clone,
    B: Clone,
    F: Fn(A, B) -> C,
{
    a.iter()
        .cloned()
        .zip(b.iter().cloned())
        .map(|(x, y)| f(x, y))
        .collect()
}

// Approach 3: Associated type bounds
fn sum_items<I>(iter: I) -> I::Item
where
    I: Iterator,
    I::Item: Add<Output = I::Item> + Default,
{
    iter.fold(I::Item::default(), |acc, x| acc + x)
}

fn dot_product<T>(a: &[T], b: &[T]) -> T
where
    T: Add<Output = T> + Mul<Output = T> + Default + Copy,
{
    a.iter()
        .zip(b.iter())
        .fold(T::default(), |acc, (&x, &y)| acc + x * y)
}

// Complex: display collection of displayable items
fn display_collection<I>(iter: I) -> String
where
    I: IntoIterator,
    I::Item: Display,
{
    let items: Vec<String> = iter.into_iter().map(|x| format!("{}", x)).collect();
    format!("[{}]", items.join(", "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_if_equal() {
        assert_eq!(print_if_equal(&5, &5), "5 == 5");
        assert_eq!(print_if_equal(&3, &4), "3 != 4");
    }

    #[test]
    fn test_zip_with() {
        assert_eq!(
            zip_with(&[1, 2, 3], &[4, 5, 6], |a, b| a + b),
            vec![5, 7, 9]
        );
        assert_eq!(zip_with(&[1, 2], &[3, 4], |a, b| a * b), vec![3, 8]);
    }

    #[test]
    fn test_sum_items() {
        assert_eq!(sum_items(vec![1, 2, 3, 4, 5].into_iter()), 15);
    }

    #[test]
    fn test_dot_product() {
        assert_eq!(dot_product(&[1, 2, 3], &[4, 5, 6]), 32);
    }

    #[test]
    fn test_display_collection() {
        assert_eq!(display_collection(vec![1, 2, 3]), "[1, 2, 3]");
    }
}
