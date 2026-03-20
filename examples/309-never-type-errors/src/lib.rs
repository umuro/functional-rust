#![allow(clippy::all)]
//! # The ! (never) Type in Error Handling
//!
//! `!` is the never/bottom type for diverging functions.

use std::convert::Infallible;

/// Function that never returns
pub fn crash(msg: &str) -> ! {
    panic!("{}", msg)
}

/// ! coerces to any type
pub fn parse_or_crash(s: &str) -> i32 {
    s.parse::<i32>()
        .unwrap_or_else(|e| crash(&format!("fatal: {}", e)))
}

/// Infallible conversion - can only be Ok
pub fn to_uppercase(s: &str) -> Result<String, Infallible> {
    Ok(s.to_uppercase())
}

/// Extension trait for unwrap_infallible
pub trait UnwrapInfallible<T> {
    fn unwrap_infallible(self) -> T;
}

impl<T> UnwrapInfallible<T> for Result<T, Infallible> {
    fn unwrap_infallible(self) -> T {
        match self {
            Ok(v) => v,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infallible_is_ok() {
        let r: Result<i32, Infallible> = Ok(42);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 42);
    }

    #[test]
    fn test_to_uppercase() {
        let r = to_uppercase("rust");
        assert_eq!(r.unwrap(), "RUST");
    }

    #[test]
    fn test_unwrap_infallible() {
        let r: Result<String, Infallible> = Ok("hello".to_string());
        assert_eq!(r.unwrap_infallible(), "hello");
    }

    #[test]
    fn test_parse_or_crash() {
        assert_eq!(parse_or_crash("100"), 100);
    }

    #[test]
    #[should_panic]
    fn test_crash_panics() {
        crash("intentional");
    }
}
