//! 314. Accumulating multiple errors (Validated)
//!
//! Validated accumulates ALL errors, unlike Result which stops at first.

use std::fmt;

/// A Validated type that accumulates errors (applicative, not monadic)
#[derive(Debug, PartialEq)]
enum Validated<T, E> {
    Valid(T),
    Invalid(Vec<E>),
}

impl<T, E> Validated<T, E> {
    fn valid(v: T) -> Self { Validated::Valid(v) }
    fn invalid(e: E) -> Self { Validated::Invalid(vec![e]) }

    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Validated<U, E> {
        match self {
            Validated::Valid(v) => Validated::Valid(f(v)),
            Validated::Invalid(es) => Validated::Invalid(es),
        }
    }

    fn and_then<U, F: FnOnce(T) -> Validated<U, E>>(self, f: F) -> Validated<U, E> {
        match self {
            Validated::Valid(v) => f(v),
            Validated::Invalid(es) => Validated::Invalid(es),
        }
    }

    fn errors(self) -> Vec<E> {
        match self { Validated::Invalid(es) => es, _ => vec![] }
    }
}

/// Combine two Validated values applicatively (accumulate all errors)
fn combine<A, B, E>(a: Validated<A, E>, b: Validated<B, E>) -> Validated<(A, B), E> {
    match (a, b) {
        (Validated::Valid(a), Validated::Valid(b)) => Validated::Valid((a, b)),
        (Validated::Invalid(mut e1), Validated::Invalid(e2)) => {
            e1.extend(e2); Validated::Invalid(e1)
        }
        (Validated::Invalid(e), _) | (_, Validated::Invalid(e)) => Validated::Invalid(e),
    }
}

// Form validation functions
fn validate_name(name: &str) -> Validated<&str, String> {
    if name.is_empty() { return Validated::invalid("name cannot be empty".to_string()); }
    if name.len() > 50 { return Validated::invalid("name too long (max 50)".to_string()); }
    Validated::valid(name)
}

fn validate_email(email: &str) -> Validated<&str, String> {
    if !email.contains('@') {
        return Validated::invalid(format!("'{}' is not a valid email", email));
    }
    Validated::valid(email)
}

fn validate_age(age_str: &str) -> Validated<u8, String> {
    let n: i32 = match age_str.parse() {
        Ok(n) => n,
        Err(_) => return Validated::invalid(format!("'{}' is not a number", age_str)),
    };
    if n < 0 || n > 150 {
        return Validated::invalid(format!("age {} out of range [0, 150]", n));
    }
    Validated::valid(n as u8)
}

fn validate_registration<'a>(name: &'a str, email: &'a str, age_str: &str)
    -> Validated<(&'a str, &'a str, u8), String>
{
    let v_name  = validate_name(name);
    let v_email = validate_email(email);
    let v_age   = validate_age(age_str);
    // Combine all — ALL errors are accumulated
    combine(combine(v_name, v_email), v_age)
        .map(|((n, e), a)| (n, e, a))
}

fn main() {
    // All valid
    match validate_registration("Alice", "alice@example.com", "30") {
        Validated::Valid((n, e, a)) => println!("Valid: {} <{}> age {}", n, e, a),
        Validated::Invalid(errs)   => println!("Errors: {:?}", errs),
    }

    // Multiple errors — ALL accumulated
    match validate_registration("", "not_an_email", "999") {
        Validated::Valid(_) => println!("Valid"),
        Validated::Invalid(errs) => {
            println!("All {} errors:", errs.len());
            for e in &errs { println!("  - {}", e); }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid() {
        let r = validate_registration("Alice", "alice@example.com", "30");
        assert!(matches!(r, Validated::Valid(_)));
    }

    #[test]
    fn test_accumulate_two_errors() {
        let r = validate_registration("", "bad", "25");
        if let Validated::Invalid(errs) = r {
            assert!(errs.len() >= 2, "expected at least 2 errors, got {}", errs.len());
        } else {
            panic!("Expected Invalid");
        }
    }

    #[test]
    fn test_accumulate_three_errors() {
        let r = validate_registration("", "bad", "999");
        if let Validated::Invalid(errs) = r {
            assert_eq!(errs.len(), 3);
        } else {
            panic!("Expected Invalid");
        }
    }
}
