#![allow(clippy::all)]
//! # Example 118: Deref Coercions
//!
//! The Rust compiler automatically dereferences smart pointers through the `Deref`
//! trait so you rarely need explicit conversions. `&String` becomes `&str`,
//! `&Vec<T>` becomes `&[T]`, and `&Box<T>` becomes `&T` — transitively.
//!
//! The idiomatic pattern is: **write functions that accept the most general borrow**
//! (`&str`, `&[T]`) and let callers pass any owning or smart-pointer form for free.

use std::ops::Deref;
use std::sync::Arc;

// ── Approach 1: Idiomatic — accept the most general borrowed form ─────────────

/// Accepts `&str` — callers may pass `&String`, `&Box<String>`, `&Rc<String>`, …
/// The compiler inserts as many `.deref()` calls as needed.
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Accepts `&[i32]` — callers may pass `&Vec<i32>`, `&[i32; N]`, `&Box<Vec<i32>>`, …
pub fn sum(data: &[i32]) -> i32 {
    data.iter().sum()
}

/// Generic first element — works with any slice-like type via deref coercion.
pub fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}

// ── Approach 2: Custom Deref implementation ───────────────────────────────────

/// A newtype wrapper around `Vec<T>`.
///
/// By implementing `Deref<Target = [T]>`, it participates in coercion chains:
/// `&MyVec<T>` → `&[T]` automatically wherever `&[T]` is expected.
pub struct MyVec<T>(Vec<T>);

impl<T> MyVec<T> {
    pub fn new(v: Vec<T>) -> Self {
        Self(v)
    }
}

impl<T> Deref for MyVec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &self.0
    }
}

/// Works because `&MyVec<i32>` coerces to `&[i32]` via our `Deref` impl.
pub fn sum_my_vec(v: &MyVec<i32>) -> i32 {
    sum(v)
}

// ── Approach 3: Rc / Arc chains ───────────────────────────────────────────────

/// Demonstrates `&Rc<String>` → `&String` → `&str` (two steps).
pub fn shout(name: &str) -> String {
    format!("{}!", name.to_uppercase())
}

/// `&Arc<Vec<i32>>` → `&Vec<i32>` → `&[i32]` (two deref steps).
pub fn sum_arc(data: &Arc<Vec<i32>>) -> i32 {
    sum(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    // ── single-step coercions ─────────────────────────────────────────────────

    #[test]
    fn test_greet_str_literal() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
    }

    #[test]
    fn test_greet_owned_string_coerces_to_str() {
        let name = String::from("Bob");
        assert_eq!(greet(&name), "Hello, Bob!"); // &String → &str
    }

    #[test]
    fn test_sum_vec_coerces_to_slice() {
        let v = vec![1, 2, 3, 4];
        assert_eq!(sum(&v), 10); // &Vec<i32> → &[i32]
    }

    #[test]
    fn test_sum_slice_literal() {
        assert_eq!(sum(&[10, 20, 30]), 60);
    }

    #[test]
    fn test_sum_empty() {
        assert_eq!(sum(&[]), 0);
    }

    // ── transitive / two-step coercions ──────────────────────────────────────

    #[test]
    fn test_greet_box_string_two_step_coercion() {
        // &Box<String> → &String → &str  (two automatic deref steps)
        let boxed: Box<String> = Box::new(String::from("Carol"));
        assert_eq!(greet(&boxed), "Hello, Carol!");
    }

    #[test]
    fn test_greet_rc_string_two_step_coercion() {
        // &Rc<String> → &String → &str
        let rc = Rc::new(String::from("Dave"));
        assert_eq!(shout(&rc), "DAVE!"); // uses shout, same coercion
    }

    #[test]
    fn test_sum_arc_vec_two_step_coercion() {
        // &Arc<Vec<i32>> → &Vec<i32> → &[i32]
        let arc = Arc::new(vec![5, 10, 15]);
        assert_eq!(sum_arc(&arc), 30);
    }

    // ── custom Deref ──────────────────────────────────────────────────────────

    #[test]
    fn test_custom_deref_sum() {
        let mv = MyVec::new(vec![5, 10, 15]);
        assert_eq!(sum_my_vec(&mv), 30); // &MyVec<i32> → &[i32]
    }

    #[test]
    fn test_custom_deref_first() {
        let mv = MyVec::new(vec![7, 8, 9]);
        assert_eq!(first(&mv), Some(&7)); // &MyVec<T> → &[T]
    }

    #[test]
    fn test_custom_deref_empty() {
        let mv: MyVec<i32> = MyVec::new(vec![]);
        assert_eq!(first(&mv), None);
    }

    // ── method resolution through deref ──────────────────────────────────────

    #[test]
    fn test_method_called_on_box_resolves_through_deref() {
        // Box<String> has no `.len()` method, but deref to String → str gives us one
        let boxed = Box::new(String::from("hello"));
        assert_eq!(boxed.len(), 5); // deref auto-applied for method calls too
    }
}
