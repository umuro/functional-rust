#![allow(dead_code)]
#![allow(clippy::all)]
// 1012: The Never Type (!)
// Diverging functions, match exhaustiveness, infallible conversions

use std::fmt;

// Approach 1: Diverging functions return !
fn diverge_panic() -> ! {
    panic!("this never returns");
}

fn diverge_loop() -> ! {
    // panic! diverges — satisfies -> ! without an infinite loop
    panic!("for testing we break with panic");
}

// Approach 2: ! in match arms for exhaustiveness
fn handle_infallible(r: Result<i64, std::convert::Infallible>) -> i64 {
    match r {
        Ok(n) => n,
        // Err branch is unreachable — Infallible can't be constructed
        Err(e) => match e {}, // empty match on uninhabited type
    }
}

// Approach 3: Using ! in enums and type positions
#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

// When E = std::convert::Infallible, Err is impossible
fn always_succeeds() -> Result<i64, std::convert::Infallible> {
    Ok(42)
}

// Diverging in match arms
fn classify(n: i64) -> String {
    match n {
        n if n > 0 => format!("positive: {}", n),
        n if n < 0 => format!("negative: {}", n),
        0 => "zero".into(),
        _ => unreachable!(), // returns !, unifies with String
    }
}

// Custom error that can display but also show unreachable patterns
#[derive(Debug)]
enum ParseOrNever {
    BadFormat(String),
}

impl fmt::Display for ParseOrNever {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseOrNever::BadFormat(s) => write!(f, "bad format: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_infallible_result() {
        let result = always_succeeds();
        assert_eq!(handle_infallible(result), 42);
    }

    #[test]
    fn test_classify() {
        assert_eq!(classify(5), "positive: 5");
        assert_eq!(classify(-3), "negative: -3");
        assert_eq!(classify(0), "zero");
    }

    #[test]
    #[should_panic(expected = "this never returns")]
    fn test_diverge_panic() {
        diverge_panic();
    }

    #[test]
    #[should_panic]
    fn test_diverge_loop() {
        diverge_loop();
    }

    #[test]
    fn test_never_coercion() {
        // ! coerces to any type, so diverging expressions
        // can appear in any type context
        let _val: i64 = if true { 42 } else { panic!("never") };
        assert_eq!(_val, 42);
    }

    #[test]
    fn test_infallible_into() {
        // Infallible implements Into<T> for all T... but we can't construct one
        // This is the key insight: you can't create Infallible, so From/Into are vacuously true
        let r: Result<i64, std::convert::Infallible> = Ok(99);
        // unwrap is always safe on infallible results
        assert_eq!(r.unwrap(), 99);
    }
}
