#![allow(clippy::all)]
//! Blanket Implementations
//!
//! Implement a trait for all types that satisfy a bound.

use std::fmt;

/// Trait with summarization capability
pub trait Summary {
    fn summarize(&self) -> String;
}

/// Blanket impl: anything that is Display also gets Summary
impl<T: fmt::Display> Summary for T {
    fn summarize(&self) -> String {
        format!("Summary: {}", self)
    }
}

/// Another example: double the string representation
pub trait DoubleString {
    fn double_string(&self) -> String;
}

impl<T: fmt::Display> DoubleString for T {
    fn double_string(&self) -> String {
        let s = self.to_string();
        format!("{}{}", s, s)
    }
}

/// Blanket impl for Into
pub trait IntoJson {
    fn into_json(&self) -> String;
}

impl<T: fmt::Debug> IntoJson for T {
    fn into_json(&self) -> String {
        format!("{{\"{:?}\"}}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blanket_summary() {
        assert_eq!(42i32.summarize(), "Summary: 42");
        assert_eq!("hi".summarize(), "Summary: hi");
    }

    #[test]
    fn test_double_string() {
        assert_eq!(7u32.double_string(), "77");
        assert_eq!("abc".double_string(), "abcabc");
    }

    #[test]
    fn test_float_summary() {
        assert_eq!(3.14f64.summarize(), "Summary: 3.14");
    }

    #[test]
    fn test_into_json() {
        let val = vec![1, 2, 3];
        let json = val.into_json();
        assert!(json.contains("[1, 2, 3]"));
    }

    #[test]
    fn test_blanket_for_option() {
        let opt = Some(42);
        // Option<T> where T: Display implements Display
        // so our blanket impl applies
        assert!(opt.into_json().contains("Some"));
    }
}
