#![allow(clippy::all)]
//! # Irrefutable vs Refutable Patterns
//!
//! Some patterns always match (irrefutable); others might not (refutable).
//! Rust's syntax reflects this difference and enforces it at compile time.

/// A simple point struct for demonstrating struct destructuring.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Demonstrates irrefutable tuple destructuring.
/// This always succeeds because a tuple always has all its elements.
pub fn destructure_tuple(tuple: (i32, i32, i32)) -> i32 {
    let (a, b, c) = tuple;
    a + b + c
}

/// Demonstrates irrefutable struct destructuring.
pub fn destructure_point(point: Point) -> f64 {
    let Point { x, y } = point;
    x + y
}

/// Function with irrefutable pattern in parameter position.
pub fn add_pair((a, b): (i32, i32)) -> i32 {
    a + b
}

/// Demonstrates refutable pattern with if let.
pub fn extract_some(opt: Option<i32>) -> Option<i32> {
    if let Some(v) = opt {
        Some(v * 2)
    } else {
        None
    }
}

/// Alternative using match (also handles refutable patterns).
pub fn extract_some_match(opt: Option<i32>) -> Option<i32> {
    match opt {
        Some(v) => Some(v * 2),
        None => None,
    }
}

/// Alternative using map (most idiomatic for this case).
pub fn extract_some_map(opt: Option<i32>) -> Option<i32> {
    opt.map(|v| v * 2)
}

/// Demonstrates while let for refutable patterns.
pub fn sum_stack(mut stack: Vec<i32>) -> i32 {
    let mut sum = 0;
    while let Some(v) = stack.pop() {
        sum += v;
    }
    sum
}

/// Alternative using drain (more idiomatic).
pub fn sum_stack_drain(mut stack: Vec<i32>) -> i32 {
    stack.drain(..).sum()
}

/// Process pairs with irrefutable destructuring in for loop.
pub fn process_pairs(pairs: &[(i32, char)]) -> String {
    let mut result = String::new();
    for (n, ch) in pairs {
        result.push_str(&format!("{}{}", n, ch));
    }
    result
}

/// Alternative using iterators.
pub fn process_pairs_iter(pairs: &[(i32, char)]) -> String {
    pairs.iter().map(|(n, ch)| format!("{}{}", n, ch)).collect()
}

/// Demonstrates nested irrefutable destructuring.
pub fn nested_destructure(data: ((i32, i32), (i32, i32))) -> i32 {
    let ((a, b), (c, d)) = data;
    a + b + c + d
}

/// Demonstrates ignoring parts with underscore.
pub fn first_of_triple((first, _, _): (i32, i32, i32)) -> i32 {
    first
}

/// Extract from nested Option using if let chains.
pub fn extract_nested(opt: Option<Option<i32>>) -> Option<i32> {
    if let Some(Some(v)) = opt {
        Some(v)
    } else {
        None
    }
}

/// Alternative using flatten (idiomatic).
pub fn extract_nested_flatten(opt: Option<Option<i32>>) -> Option<i32> {
    opt.flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_destructure_tuple() {
        assert_eq!(destructure_tuple((1, 2, 3)), 6);
        assert_eq!(destructure_tuple((10, 20, 30)), 60);
    }

    #[test]
    fn test_destructure_point() {
        let p = Point::new(3.0, 4.0);
        assert_eq!(destructure_point(p), 7.0);
    }

    #[test]
    fn test_add_pair() {
        assert_eq!(add_pair((5, 7)), 12);
        assert_eq!(add_pair((-1, 1)), 0);
    }

    #[test]
    fn test_extract_some() {
        assert_eq!(extract_some(Some(21)), Some(42));
        assert_eq!(extract_some(None), None);
    }

    #[test]
    fn test_extract_approaches_equivalent() {
        let test_cases = [Some(5), Some(0), Some(-3), None];
        for opt in test_cases {
            assert_eq!(extract_some(opt), extract_some_match(opt));
            assert_eq!(extract_some(opt), extract_some_map(opt));
        }
    }

    #[test]
    fn test_sum_stack() {
        assert_eq!(sum_stack(vec![1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_stack(vec![]), 0);
    }

    #[test]
    fn test_sum_stack_approaches_equivalent() {
        let stacks = [vec![1, 2, 3], vec![10, 20], vec![], vec![42]];
        for stack in stacks {
            assert_eq!(sum_stack(stack.clone()), sum_stack_drain(stack));
        }
    }

    #[test]
    fn test_process_pairs() {
        let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c')];
        assert_eq!(process_pairs(&pairs), "1a2b3c");
    }

    #[test]
    fn test_process_pairs_approaches_equivalent() {
        let pairs = vec![(1, 'a'), (2, 'b')];
        assert_eq!(process_pairs(&pairs), process_pairs_iter(&pairs));
    }

    #[test]
    fn test_nested_destructure() {
        assert_eq!(nested_destructure(((1, 2), (3, 4))), 10);
    }

    #[test]
    fn test_first_of_triple() {
        assert_eq!(first_of_triple((42, 0, 0)), 42);
        assert_eq!(first_of_triple((1, 2, 3)), 1);
    }

    #[test]
    fn test_extract_nested() {
        assert_eq!(extract_nested(Some(Some(42))), Some(42));
        assert_eq!(extract_nested(Some(None)), None);
        assert_eq!(extract_nested(None), None);
    }

    #[test]
    fn test_extract_nested_approaches_equivalent() {
        let cases = [Some(Some(1)), Some(None), None];
        for opt in cases {
            assert_eq!(extract_nested(opt), extract_nested_flatten(opt));
        }
    }
}
