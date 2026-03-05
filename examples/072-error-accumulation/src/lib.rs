// Error accumulation: collect ALL errors, not just the first.
// Unlike Result which short-circuits, Validation gathers a list of errors.
// Models the Applicative Validation pattern from functional programming.

/// Solution 1: Idiomatic Rust — enum that mirrors the OCaml `validation` type.
#[derive(Debug, PartialEq)]
pub enum Validation<T, E> {
    Ok(T),
    Errors(Vec<E>),
}

impl<T, E> Validation<T, E> {
    pub fn ok(value: T) -> Self {
        Validation::Ok(value)
    }

    /// Functor: transform the success value.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Validation<U, E> {
        match self {
            Validation::Ok(x) => Validation::Ok(f(x)),
            Validation::Errors(es) => Validation::Errors(es),
        }
    }

    /// Applicative apply: combine errors from both sides.
    pub fn apply<U, F>(self, arg: Validation<T, E>) -> Validation<U, E>
    where
        Self: Into<Validation<F, E>>,
        F: FnOnce(T) -> U,
    {
        match (self.into(), arg) {
            (Validation::Ok(f), Validation::Ok(x)) => Validation::Ok(f(x)),
            (Validation::Ok(_), Validation::Errors(es)) => Validation::Errors(es),
            (Validation::Errors(es), Validation::Ok(_)) => Validation::Errors(es),
            (Validation::Errors(mut e1), Validation::Errors(e2)) => {
                e1.extend(e2);
                Validation::Errors(e1)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Validators (mirroring the OCaml example)
// ---------------------------------------------------------------------------

pub fn validate_name(s: &str) -> Validation<&str, String> {
    if !s.is_empty() {
        Validation::Ok(s)
    } else {
        Validation::Errors(vec!["name cannot be empty".to_string()])
    }
}

pub fn validate_age(n: i32) -> Validation<i32, String> {
    if (18..=120).contains(&n) {
        Validation::Ok(n)
    } else {
        Validation::Errors(vec![format!("age {n} out of range (18-120)")])
    }
}

pub fn validate_email(s: &str) -> Validation<&str, String> {
    if s.contains('@') {
        Validation::Ok(s)
    } else {
        Validation::Errors(vec!["email must contain @".to_string()])
    }
}

/// Solution 2: Functional style — accumulate using a fold over validators.
pub fn accumulate<T: Clone, E: Clone>(
    validators: &[fn(T) -> Validation<T, E>],
    input: T,
) -> Validation<T, E> {
    let mut errors: Vec<E> = vec![];
    let mut last_ok: Option<T> = None;

    for validator in validators {
        match validator(input.clone()) {
            Validation::Ok(v) => last_ok = Some(v),
            Validation::Errors(es) => errors.extend(es),
        }
    }

    if errors.is_empty() {
        Validation::Ok(last_ok.unwrap())
    } else {
        Validation::Errors(errors)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_name_valid() {
        assert_eq!(validate_name("Alice"), Validation::Ok("Alice"));
    }

    #[test]
    fn test_validate_name_empty() {
        assert_eq!(
            validate_name(""),
            Validation::Errors(vec!["name cannot be empty".to_string()])
        );
    }

    #[test]
    fn test_validate_age_valid() {
        assert_eq!(validate_age(30), Validation::Ok(30));
    }

    #[test]
    fn test_validate_age_too_young() {
        assert_eq!(
            validate_age(15),
            Validation::Errors(vec!["age 15 out of range (18-120)".to_string()])
        );
    }

    #[test]
    fn test_validate_email_valid() {
        assert_eq!(
            validate_email("alice@example.com"),
            Validation::Ok("alice@example.com")
        );
    }

    #[test]
    fn test_validate_email_invalid() {
        assert_eq!(
            validate_email("bad-email"),
            Validation::Errors(vec!["email must contain @".to_string()])
        );
    }

    #[test]
    fn test_errors_accumulate() {
        // All three validators fail — all three errors must be collected.
        let name_err = validate_name("");
        let age_err = validate_age(15);
        let email_err = validate_email("bad-email");

        let mut all_errors: Vec<String> = vec![];
        if let Validation::Errors(es) = name_err { all_errors.extend(es); }
        if let Validation::Errors(es) = age_err { all_errors.extend(es); }
        if let Validation::Errors(es) = email_err { all_errors.extend(es); }

        assert_eq!(all_errors.len(), 3);
        assert!(all_errors[0].contains("name"));
        assert!(all_errors[1].contains("age"));
        assert!(all_errors[2].contains("email"));
    }

    #[test]
    fn test_map_ok() {
        let v: Validation<i32, String> = Validation::Ok(3);
        assert_eq!(v.map(|x| x * 2), Validation::Ok(6));
    }
}
