#![allow(clippy::all)]
// Example 078: Where Clauses
// Complex where clauses vs inline bounds

use std::fmt::{Debug, Display};
use std::ops::{Add, Mul};

// === Approach 1: Inline bounds (simple cases) ===
fn find_max<T: PartialOrd>(slice: &[T]) -> Option<&T> {
    slice.iter().reduce(|a, b| if a >= b { a } else { b })
}

// === Approach 2: Where clauses (complex constraints) ===
fn transform_and_combine<T, U, A, F, G>(items: &[T], transform: F, combine: G, init: A) -> A
where
    F: Fn(&T) -> U,
    G: Fn(A, U) -> A,
{
    items.iter().fold(init, |acc, x| combine(acc, transform(x)))
}

fn filter_map_fold<T, U, A, P, F, G>(items: &[T], pred: P, transform: F, combine: G, init: A) -> A
where
    P: Fn(&T) -> bool,
    F: Fn(&T) -> U,
    G: Fn(A, U) -> A,
{
    items.iter().fold(init, |acc, x| {
        if pred(x) {
            combine(acc, transform(x))
        } else {
            acc
        }
    })
}

// Where clause shines with multiple related bounds
fn sorted_summary<T>(items: &mut [T]) -> String
where
    T: Ord + Display,
{
    items.sort();
    items
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

// === Approach 3: Complex multi-type where clauses ===
fn bounded_transform<T, F>(items: &[T], transform: F, lo: T, hi: T) -> Vec<T>
where
    T: PartialOrd + Clone,
    F: Fn(&T) -> T,
{
    items
        .iter()
        .map(|x| {
            let y = transform(x);
            if y < lo {
                lo.clone()
            } else if y > hi {
                hi.clone()
            } else {
                y
            }
        })
        .collect()
}

// Return type bounds in where clause
fn numeric_summary<T>(a: T, b: T) -> String
where
    T: Add<Output = T> + Mul<Output = T> + Display + Copy,
{
    let sum = a + b;
    let product = a * b;
    format!("sum={}, product={}", sum, product)
}

// Where clause with lifetime + trait bounds
fn longest_display<'a, T>(a: &'a T, b: &'a T) -> String
where
    T: Display + PartialOrd,
{
    if a >= b {
        format!("{}", a)
    } else {
        format!("{}", b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_and_combine() {
        let r = transform_and_combine(&[1, 2, 3, 4], |x| x * x, |a, b| a + b, 0);
        assert_eq!(r, 30);
    }

    #[test]
    fn test_filter_map_fold() {
        let r = filter_map_fold(
            &[1, 2, 3, 4, 5, 6],
            |x| x % 2 == 0,
            |x| x * x,
            |a, b| a + b,
            0,
        );
        assert_eq!(r, 56); // 4 + 16 + 36
    }

    #[test]
    fn test_sorted_summary() {
        let mut v = vec![3, 1, 4, 1, 5];
        assert_eq!(sorted_summary(&mut v), "1, 1, 3, 4, 5");
    }

    #[test]
    fn test_bounded_transform() {
        let r = bounded_transform(&[1, 2, 3, 4, 5], |x| x * 3, 0, 10);
        assert_eq!(r, vec![3, 6, 9, 10, 10]);
    }

    #[test]
    fn test_numeric_summary() {
        assert_eq!(numeric_summary(3, 4), "sum=7, product=12");
    }

    #[test]
    fn test_longest_display() {
        assert_eq!(longest_display(&10, &20), "20");
        assert_eq!(longest_display(&"zebra", &"apple"), "zebra");
    }

    #[test]
    fn test_empty_slice() {
        let r = transform_and_combine::<i32, i32, i32, _, _>(&[], |x| x * x, |a, b| a + b, 0);
        assert_eq!(r, 0);
    }
}
