#![allow(clippy::all)]
//! Deref and Deref Coercions
//!
//! Automatic reference conversions that let smart pointers and owned types
//! work seamlessly with borrowed slices and str.

use std::fmt;
use std::ops::{Deref, DerefMut};

/// A custom smart pointer implementing Deref.
///
/// This demonstrates how any type can behave like a reference to its inner value.
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    /// Creates a new MyBox wrapping the given value.
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }

    /// Consumes the box and returns the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: fmt::Display> fmt::Display for MyBox<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyBox({})", self.0)
    }
}

/// Approach 1: Accept borrowed str, works with String, &str, Box<String>, etc.
///
/// This demonstrates how deref coercion enables flexible APIs.
pub fn string_length(s: &str) -> usize {
    s.len()
}

/// Approach 2: Accept borrowed slice, works with Vec<T>, arrays, Box<Vec<T>>, etc.
pub fn slice_sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

/// Approach 3: Generic function that works with anything that derefs to T.
///
/// The `AsRef<T>` trait is similar to Deref but more explicit about intent.
pub fn generic_len<S: AsRef<str>>(s: S) -> usize {
    s.as_ref().len()
}

/// Demonstrates multi-level deref chain: Box<String> -> String -> str.
pub fn process_boxed_string(boxed: &Box<String>) -> String {
    // Deref chain: &Box<String> -> &String -> &str
    // We can call str methods directly!
    boxed.to_uppercase()
}

/// Demonstrates mutable deref: pushing to a boxed Vec.
pub fn push_to_boxed_vec(boxed: &mut Box<Vec<i32>>, value: i32) {
    // DerefMut allows mutable access
    boxed.push(value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mybox_deref() {
        let b = MyBox::new(42);
        // Explicit deref
        assert_eq!(*b, 42);
    }

    #[test]
    fn test_mybox_into_inner() {
        let b = MyBox::new(String::from("hello"));
        let s = b.into_inner();
        assert_eq!(s, "hello");
    }

    #[test]
    fn test_string_coercion() {
        let owned = String::from("hello");
        // &String coerces to &str
        assert_eq!(string_length(&owned), 5);
    }

    #[test]
    fn test_boxed_string_coercion() {
        let boxed = Box::new(String::from("world"));
        // &Box<String> coerces through: Box -> String -> str
        assert_eq!(string_length(&boxed), 5);
    }

    #[test]
    fn test_mybox_string_coercion() {
        let my_box = MyBox::new(String::from("custom"));
        // &MyBox<String> -> &String -> &str (two-level coercion)
        assert_eq!(string_length(&my_box), 6);
    }

    #[test]
    fn test_vec_to_slice_coercion() {
        let v = vec![1, 2, 3, 4, 5];
        // &Vec<i32> coerces to &[i32]
        assert_eq!(slice_sum(&v), 15);
    }

    #[test]
    fn test_boxed_vec_coercion() {
        let boxed_vec = Box::new(vec![10, 20, 30]);
        // &Box<Vec<i32>> coerces through: Box -> Vec -> [i32]
        assert_eq!(slice_sum(&boxed_vec), 60);
    }

    #[test]
    fn test_generic_asref() {
        assert_eq!(generic_len("literal"), 7);
        assert_eq!(generic_len(String::from("owned")), 5);
        assert_eq!(generic_len(&String::from("borrowed")), 8);
    }

    #[test]
    fn test_process_boxed_string() {
        let boxed = Box::new(String::from("hello"));
        assert_eq!(process_boxed_string(&boxed), "HELLO");
    }

    #[test]
    fn test_deref_mut() {
        let mut b = MyBox::new(vec![1, 2, 3]);
        b.push(4); // DerefMut allows calling Vec methods
        assert_eq!(*b, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_push_to_boxed_vec() {
        let mut boxed = Box::new(vec![1, 2]);
        push_to_boxed_vec(&mut boxed, 3);
        assert_eq!(*boxed, vec![1, 2, 3]);
    }

    #[test]
    fn test_method_resolution_through_deref() {
        let boxed = MyBox::new(String::from("test"));
        // .len() is resolved through deref chain: MyBox -> String -> str
        assert_eq!(boxed.len(), 4);
    }
}
