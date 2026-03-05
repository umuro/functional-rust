//! # Accumulating Multiple Errors (Validated)
//!
//! Validated accumulates ALL errors, unlike Result which stops at first.

/// Validated type for error accumulation
#[derive(Debug, PartialEq)]
pub enum Validated<T, E> {
    Valid(T),
    Invalid(Vec<E>),
}

impl<T, E> Validated<T, E> {
    pub fn valid(v: T) -> Self { Validated::Valid(v) }
    pub fn invalid(e: E) -> Self { Validated::Invalid(vec![e]) }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Validated<U, E> {
        match self {
            Validated::Valid(v) => Validated::Valid(f(v)),
            Validated::Invalid(es) => Validated::Invalid(es),
        }
    }
}

pub fn combine<A, B, E>(a: Validated<A, E>, b: Validated<B, E>) -> Validated<(A, B), E> {
    match (a, b) {
        (Validated::Valid(a), Validated::Valid(b)) => Validated::Valid((a, b)),
        (Validated::Invalid(mut e1), Validated::Invalid(e2)) => {
            e1.extend(e2); Validated::Invalid(e1)
        }
        (Validated::Invalid(e), _) | (_, Validated::Invalid(e)) => Validated::Invalid(e),
    }
}

pub fn validate_name(name: &str) -> Validated<String, String> {
    if name.is_empty() { return Validated::invalid("name cannot be empty".into()); }
    Validated::valid(name.to_string())
}

pub fn validate_email(email: &str) -> Validated<String, String> {
    if !email.contains('@') { return Validated::invalid(format!("invalid email: {}", email)); }
    Validated::valid(email.to_string())
}

pub fn validate_age(age_str: &str) -> Validated<u8, String> {
    match age_str.parse::<i32>() {
        Ok(n) if n >= 0 && n <= 150 => Validated::valid(n as u8),
        Ok(n) => Validated::invalid(format!("age {} out of range", n)),
        Err(_) => Validated::invalid(format!("'{}' is not a number", age_str)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid() {
        let name = validate_name("Alice");
        let email = validate_email("alice@example.com");
        let r = combine(name, email);
        assert!(matches!(r, Validated::Valid(_)));
    }

    #[test]
    fn test_accumulate_two_errors() {
        let name = validate_name("");
        let email = validate_email("bad");
        let r = combine(name, email);
        if let Validated::Invalid(errs) = r {
            assert_eq!(errs.len(), 2);
        } else {
            panic!("Expected Invalid");
        }
    }

    #[test]
    fn test_accumulate_three() {
        let name = validate_name("");
        let email = validate_email("bad");
        let age = validate_age("999");
        let r = combine(combine(name, email), age);
        if let Validated::Invalid(errs) = r {
            assert_eq!(errs.len(), 3);
        } else {
            panic!("Expected Invalid");
        }
    }
}
