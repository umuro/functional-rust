#![allow(clippy::all)]
//! Tap Pattern for Side Effects
//!
//! Inspect values in a pipeline without disrupting the data flow.

/// Tap: run a side effect, then return the value unchanged.
pub fn tap<T, F: FnOnce(&T)>(value: T, f: F) -> T {
    f(&value);
    value
}

/// Tap with a mutable reference.
pub fn tap_mut<T, F: FnOnce(&mut T)>(mut value: T, f: F) -> T {
    f(&mut value);
    value
}

/// Extension trait to enable chained .tap() calls.
pub trait Tap: Sized {
    fn tap(self, f: impl FnOnce(&Self)) -> Self {
        f(&self);
        self
    }

    fn tap_mut(mut self, f: impl FnOnce(&mut Self)) -> Self {
        f(&mut self);
        self
    }

    fn tap_dbg(self, label: &str) -> Self
    where
        Self: std::fmt::Debug,
    {
        eprintln!("{}: {:?}", label, &self);
        self
    }
}

impl<T> Tap for T {}

/// Debug tap that prints the value.
pub fn tap_debug<T: std::fmt::Debug>(value: T) -> T {
    eprintln!("DEBUG: {:?}", value);
    value
}

/// Conditional tap.
pub fn tap_if<T, F: FnOnce(&T)>(value: T, condition: bool, f: F) -> T {
    if condition {
        f(&value);
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn test_tap_function() {
        let log = RefCell::new(Vec::new());
        let result = tap(42, |x| log.borrow_mut().push(*x));
        assert_eq!(result, 42);
        assert_eq!(*log.borrow(), vec![42]);
    }

    #[test]
    fn test_tap_mut_function() {
        let result = tap_mut(vec![1, 2, 3], |v| v.push(4));
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_tap_trait() {
        let log = RefCell::new(Vec::new());
        let result = 42.tap(|x| log.borrow_mut().push(*x));
        assert_eq!(result, 42);
        assert_eq!(*log.borrow(), vec![42]);
    }

    #[test]
    fn test_tap_mut_trait() {
        let result = vec![1, 2, 3].tap_mut(|v| v.push(4));
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_tap_chain() {
        let log = RefCell::new(Vec::new());

        let result = 10
            .tap(|x| log.borrow_mut().push(format!("start: {}", x)))
            .tap(|x| log.borrow_mut().push(format!("value: {}", x)));

        assert_eq!(result, 10);
        assert_eq!(log.borrow().len(), 2);
    }

    #[test]
    fn test_tap_in_pipeline() {
        let log = RefCell::new(Vec::new());

        let result: i32 = [1, 2, 3, 4, 5]
            .iter()
            .map(|&x| x * 2)
            .map(|x| tap(x, |v| log.borrow_mut().push(*v)))
            .sum();

        assert_eq!(result, 30);
        assert_eq!(*log.borrow(), vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_tap_if() {
        let mut called = false;
        let _ = tap_if(42, true, |_| called = true);
        assert!(called);

        called = false;
        let _ = tap_if(42, false, |_| called = true);
        assert!(!called);
    }

    #[test]
    fn test_tap_debug() {
        // Just verify it compiles and returns the value
        let result = tap_debug(vec![1, 2, 3]);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_tap_preserves_value() {
        let original = String::from("hello");
        let result = original.tap(|s| {
            // Expensive debug operation
            let _ = s.len();
        });
        assert_eq!(result, "hello");
    }
}
