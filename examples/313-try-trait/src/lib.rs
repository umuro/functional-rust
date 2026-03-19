#![allow(clippy::all)]
//! # The Try Trait and Custom ? Behavior
//!
//! Validated type that accumulates errors instead of short-circuiting.

/// Validated type - accumulates errors applicatively
#[derive(Debug, PartialEq)]
pub enum Validated<T, E> {
    Ok(T),
    Err(Vec<E>),
}

impl<T, E> Validated<T, E> {
    pub fn ok(v: T) -> Self {
        Validated::Ok(v)
    }
    pub fn err(e: E) -> Self {
        Validated::Err(vec![e])
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Validated<U, E> {
        match self {
            Validated::Ok(v) => Validated::Ok(f(v)),
            Validated::Err(es) => Validated::Err(es),
        }
    }

    pub fn and<U>(self, other: Validated<U, E>) -> Validated<(T, U), E> {
        match (self, other) {
            (Validated::Ok(a), Validated::Ok(b)) => Validated::Ok((a, b)),
            (Validated::Err(mut e1), Validated::Err(e2)) => {
                e1.extend(e2);
                Validated::Err(e1)
            }
            (Validated::Err(e), _) | (_, Validated::Err(e)) => Validated::Err(e),
        }
    }

    pub fn is_ok(&self) -> bool {
        matches!(self, Validated::Ok(_))
    }
}

pub fn validate_age(age: i32) -> Validated<i32, String> {
    if age >= 0 && age <= 150 {
        Validated::ok(age)
    } else {
        Validated::err(format!("age {} is out of range", age))
    }
}

pub fn validate_name(name: &str) -> Validated<String, String> {
    if name.len() >= 2 && name.len() <= 50 {
        Validated::ok(name.to_string())
    } else {
        Validated::err(format!("name '{}' is invalid", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validated_ok() {
        assert_eq!(validate_age(25), Validated::Ok(25));
    }

    #[test]
    fn test_validated_err() {
        assert!(matches!(validate_age(999), Validated::Err(_)));
    }

    #[test]
    fn test_accumulate_errors() {
        let r = validate_age(999).and(validate_name("X"));
        if let Validated::Err(errs) = r {
            assert_eq!(errs.len(), 2);
        } else {
            panic!("Expected Err");
        }
    }

    #[test]
    fn test_and_both_ok() {
        let r = validate_age(25).and(validate_name("Alice"));
        assert!(r.is_ok());
    }

    #[test]
    fn test_map() {
        let r = validate_age(25).map(|a| a * 2);
        assert_eq!(r, Validated::Ok(50));
    }
}
