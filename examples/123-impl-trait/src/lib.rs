#![allow(clippy::all)]
//! Example 123: impl Trait in Function Signatures
//!
//! `impl Trait` in argument position: syntactic sugar for generics —
//! the compiler monomorphizes one concrete type per call site.
//!
//! `impl Trait` in return position: opaque return type — the caller sees
//! only the trait bound; the concrete type is hidden and chosen by the
//! function body. This lets you return unnameable types like closures and
//! complex iterator chains without heap-boxing them.

use std::fmt::Display;

// ---------------------------------------------------------------------------
// Approach 1: impl Trait in argument position
// Equivalent to fn stringify_all<T: Display>(items: &[T]) -> Vec<String>
// ---------------------------------------------------------------------------

pub fn stringify_all(items: &[impl Display]) -> Vec<String> {
    items.iter().map(|x| x.to_string()).collect()
}

// Generic version — identical semantics, more explicit syntax
pub fn stringify_all_generic<T: Display>(items: &[T]) -> Vec<String> {
    items.iter().map(|x| x.to_string()).collect()
}

// ---------------------------------------------------------------------------
// Approach 2: impl Trait in return position (opaque return type)
//
// The concrete type (a closure `impl Fn(i32) -> i32`) is unnameable, so
// we return `impl Fn(i32) -> i32` instead of boxing it with `Box<dyn Fn>`.
// ---------------------------------------------------------------------------

pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n
}

// ---------------------------------------------------------------------------
// Approach 3: Returning an opaque iterator
//
// The concrete type of the chain below is unwritable by hand.
// `impl Iterator<Item = u32>` lets the caller iterate without caring.
// ---------------------------------------------------------------------------

pub fn even_squares(limit: u32) -> impl Iterator<Item = u32> {
    (0..limit).filter(|n| n % 2 == 0).map(|n| n * n)
}

// ---------------------------------------------------------------------------
// Approach 4: Multiple trait bounds in argument position
// ---------------------------------------------------------------------------

pub fn print_and_count(items: &[impl Display + std::fmt::Debug]) -> usize {
    items.len()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stringify_ints() {
        assert_eq!(stringify_all(&[1, 2, 3]), vec!["1", "2", "3"]);
    }

    #[test]
    fn test_stringify_floats() {
        assert_eq!(
            stringify_all(&[1.5_f64, 2.5, 3.5]),
            vec!["1.5", "2.5", "3.5"]
        );
    }

    #[test]
    fn test_stringify_generic_same_as_impl_trait() {
        let a = stringify_all(&[10, 20, 30]);
        let b = stringify_all_generic(&[10, 20, 30]);
        assert_eq!(a, b);
    }

    #[test]
    fn test_make_adder_basic() {
        let add5 = make_adder(5);
        assert_eq!(add5(10), 15);
        assert_eq!(add5(0), 5);
        assert_eq!(add5(-3), 2);
    }

    #[test]
    fn test_make_adder_independent_closures() {
        let add3 = make_adder(3);
        let add7 = make_adder(7);
        assert_eq!(add3(10) + add7(10), 30);
    }

    #[test]
    fn test_even_squares_basic() {
        let result: Vec<u32> = even_squares(7).collect();
        // even numbers < 7: 0, 2, 4, 6  → squares: 0, 4, 16, 36
        assert_eq!(result, vec![0, 4, 16, 36]);
    }

    #[test]
    fn test_even_squares_empty() {
        let result: Vec<u32> = even_squares(0).collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_print_and_count() {
        assert_eq!(print_and_count(&[1, 2, 3, 4]), 4);
        assert_eq!(print_and_count(&["a", "b"]), 2);
    }
}
