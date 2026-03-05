//! 309. The ! (never) type in error handling
//!
//! `!` is the never/bottom type: diverging functions, `Infallible`, exhaustive matches.

use std::convert::Infallible;

/// A function that never returns — return type `!`
fn crash(msg: &str) -> ! {
    panic!("{}", msg)
}

/// `!` coerces to any type — useful in match arms
fn parse_or_crash(s: &str) -> i32 {
    s.parse::<i32>().unwrap_or_else(|e| crash(&format!("fatal parse error: {}", e)))
}

/// Result<T, Infallible> can only be Ok — infallible conversion
fn to_uppercase(s: &str) -> Result<String, Infallible> {
    Ok(s.to_uppercase())
}

/// From<Infallible> coercion: convert Result<T, Infallible> to T
fn infallible_result() {
    let r: Result<i32, Infallible> = Ok(42);
    // Since Infallible has no values, we can exhaustively match with only Ok arm
    let val = match r {
        Ok(v) => v,
        // Err(e) => match e {} // would need this, but Infallible has no variants
    };
    println!("Infallible result: {}", val);
    // Or use unwrap — can never panic since Err is impossible
    let val2: i32 = Ok::<i32, Infallible>(99).unwrap();
    println!("unwrap on Infallible: {}", val2);
}

/// The never type in match arms (! coerces to anything)
fn process(s: &str) -> i32 {
    if let Ok(n) = s.parse::<i32>() {
        n
    } else {
        crash("this input is always a number") // -> ! coerces to i32
    }
}

fn main() {
    println!("parse_or_crash('42') = {}", parse_or_crash("42"));

    let upper: Result<String, Infallible> = to_uppercase("hello");
    println!("Infallible: {:?}", upper);

    // Convert Infallible result to value (always safe)
    let val: String = upper.unwrap_infallible();
    println!("Unwrapped: {}", val);

    infallible_result();

    // Never type in closures
    let _f: fn() -> ! = || panic!("never");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::Infallible;

    #[test]
    fn test_infallible_is_ok() {
        let r: Result<i32, Infallible> = Ok(42);
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 42);
    }

    #[test]
    fn test_to_uppercase_infallible() {
        let r = to_uppercase("rust");
        assert_eq!(r.unwrap(), "RUST");
    }

    #[test]
    fn test_parse_or_crash() {
        assert_eq!(parse_or_crash("100"), 100);
    }

    #[test]
    #[should_panic]
    fn test_crash_panics() {
        crash("intentional crash");
    }
}

// Extension trait for unwrap_infallible
trait UnwrapInfallible<T> {
    fn unwrap_infallible(self) -> T;
}

impl<T> UnwrapInfallible<T> for Result<T, Infallible> {
    fn unwrap_infallible(self) -> T {
        match self {
            Ok(v) => v,
        }
    }
}
