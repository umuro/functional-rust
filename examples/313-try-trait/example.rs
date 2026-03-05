//! 313. The Try trait and custom ? behavior
//!
//! The Try trait defines `?` behavior. This shows the conceptual pattern
//! and stable workarounds using `and_then` chains.

// ---- Stable approach: custom type with and_then ----

/// A "Validated" type that accumulates errors instead of short-circuiting
#[derive(Debug, PartialEq)]
enum Validated<T, E> {
    Ok(T),
    Err(Vec<E>),
}

impl<T, E> Validated<T, E> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Validated<U, E> {
        match self {
            Validated::Ok(v) => Validated::Ok(f(v)),
            Validated::Err(es) => Validated::Err(es),
        }
    }

    /// Combine two Validated values, accumulating errors
    fn and<U>(self, other: Validated<U, E>) -> Validated<(T, U), E> {
        match (self, other) {
            (Validated::Ok(a), Validated::Ok(b)) => Validated::Ok((a, b)),
            (Validated::Err(mut e1), Validated::Err(e2)) => {
                e1.extend(e2);
                Validated::Err(e1)
            }
            (Validated::Err(e), _) | (_, Validated::Err(e)) => Validated::Err(e),
        }
    }

    fn is_ok(&self) -> bool { matches!(self, Validated::Ok(_)) }
}

fn validate_age(age: i32) -> Validated<i32, String> {
    if age >= 0 && age <= 150 {
        Validated::Ok(age)
    } else {
        Validated::Err(vec![format!("age {} is out of range", age)])
    }
}

fn validate_name(name: &str) -> Validated<String, String> {
    if name.len() >= 2 && name.len() <= 50 {
        Validated::Ok(name.to_string())
    } else {
        Validated::Err(vec![format!("name '{}' is invalid length", name)])
    }
}

fn validate_email(email: &str) -> Validated<String, String> {
    if email.contains('@') {
        Validated::Ok(email.to_string())
    } else {
        Validated::Err(vec![format!("'{}' is not a valid email", email)])
    }
}

fn main() {
    // All valid
    let result = validate_age(25)
        .and(validate_name("Alice"))
        .and(validate_email("alice@example.com"))
        .map(|((age, name), email)| format!("{} (age {}) <{}>", name, age, email));
    println!("Valid: {:?}", result);

    // Multiple errors accumulated
    let result = validate_age(999)
        .and(validate_name("X"))
        .and(validate_email("not_an_email"))
        .map(|((age, name), email)| format!("{} {} {}", age, name, email));
    println!("With errors: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validated_ok() {
        let r = validate_age(25);
        assert_eq!(r, Validated::Ok(25));
    }

    #[test]
    fn test_validated_err() {
        let r = validate_age(999);
        assert!(matches!(r, Validated::Err(_)));
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
        assert_eq!(r, Validated::Ok((25, "Alice".to_string())));
    }
}
