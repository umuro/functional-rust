#![allow(clippy::all)]
// Example 054: Applicative Validation
// Accumulate ALL errors instead of short-circuiting on first

#[derive(Debug, PartialEq, Clone)]
enum Validated<T, E> {
    Valid(T),
    Invalid(Vec<E>),
}

impl<T, E> Validated<T, E> {
    fn pure(x: T) -> Self {
        Validated::Valid(x)
    }

    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Validated<U, E> {
        match self {
            Validated::Valid(x) => Validated::Valid(f(x)),
            Validated::Invalid(es) => Validated::Invalid(es),
        }
    }
}

// Approach 1: Apply that accumulates errors
fn apply<A, B, E, F: FnOnce(A) -> B>(vf: Validated<F, E>, va: Validated<A, E>) -> Validated<B, E> {
    match (vf, va) {
        (Validated::Valid(f), Validated::Valid(a)) => Validated::Valid(f(a)),
        (Validated::Invalid(mut e1), Validated::Invalid(e2)) => {
            e1.extend(e2);
            Validated::Invalid(e1)
        }
        (Validated::Invalid(e), _) | (_, Validated::Invalid(e)) => Validated::Invalid(e),
    }
}

// Approach 2: lift2/lift3 for validation
fn lift2<A, B, C, E, F: FnOnce(A, B) -> C>(
    f: F,
    a: Validated<A, E>,
    b: Validated<B, E>,
) -> Validated<C, E> {
    match (a, b) {
        (Validated::Valid(a), Validated::Valid(b)) => Validated::Valid(f(a, b)),
        (Validated::Invalid(mut e1), Validated::Invalid(e2)) => {
            e1.extend(e2);
            Validated::Invalid(e1)
        }
        (Validated::Invalid(e), _) | (_, Validated::Invalid(e)) => Validated::Invalid(e),
    }
}

fn lift3<A, B, C, D, E, F: FnOnce(A, B, C) -> D>(
    f: F,
    a: Validated<A, E>,
    b: Validated<B, E>,
    c: Validated<C, E>,
) -> Validated<D, E> {
    let mut errors = Vec::new();
    let a = match a {
        Validated::Valid(v) => Some(v),
        Validated::Invalid(e) => {
            errors.extend(e);
            None
        }
    };
    let b = match b {
        Validated::Valid(v) => Some(v),
        Validated::Invalid(e) => {
            errors.extend(e);
            None
        }
    };
    let c = match c {
        Validated::Valid(v) => Some(v),
        Validated::Invalid(e) => {
            errors.extend(e);
            None
        }
    };
    if errors.is_empty() {
        Validated::Valid(f(a.unwrap(), b.unwrap(), c.unwrap()))
    } else {
        Validated::Invalid(errors)
    }
}

// Approach 3: Validate a user record
#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: i32,
    email: String,
}

fn validate_name(s: &str) -> Validated<String, String> {
    if !s.is_empty() {
        Validated::Valid(s.to_string())
    } else {
        Validated::Invalid(vec!["Name cannot be empty".to_string()])
    }
}

fn validate_age(n: i32) -> Validated<i32, String> {
    if (0..=150).contains(&n) {
        Validated::Valid(n)
    } else {
        Validated::Invalid(vec!["Age must be between 0 and 150".to_string()])
    }
}

fn validate_email(s: &str) -> Validated<String, String> {
    if s.contains('@') {
        Validated::Valid(s.to_string())
    } else {
        Validated::Invalid(vec!["Email must contain @".to_string()])
    }
}

fn validate_user(name: &str, age: i32, email: &str) -> Validated<User, String> {
    lift3(
        |name, age, email| User { name, age, email },
        validate_name(name),
        validate_age(age),
        validate_email(email),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_user() {
        let u = validate_user("Alice", 30, "alice@example.com");
        assert_eq!(
            u,
            Validated::Valid(User {
                name: "Alice".into(),
                age: 30,
                email: "alice@example.com".into(),
            })
        );
    }

    #[test]
    fn test_single_error() {
        let u = validate_user("", 30, "alice@example.com");
        assert_eq!(u, Validated::Invalid(vec!["Name cannot be empty".into()]));
    }

    #[test]
    fn test_all_errors_accumulated() {
        let u = validate_user("", -5, "bad");
        match u {
            Validated::Invalid(errors) => {
                assert_eq!(errors.len(), 3);
                assert!(errors[0].contains("Name"));
                assert!(errors[1].contains("Age"));
                assert!(errors[2].contains("Email"));
            }
            _ => panic!("Expected Invalid"),
        }
    }

    #[test]
    fn test_lift2_both_valid() {
        let r = lift2(
            |a, b| a + b,
            Validated::<i32, &str>::Valid(1),
            Validated::Valid(2),
        );
        assert_eq!(r, Validated::Valid(3));
    }

    #[test]
    fn test_lift2_errors_accumulated() {
        let r = lift2(
            |a: i32, b: i32| a + b,
            Validated::Invalid(vec!["e1"]),
            Validated::Invalid(vec!["e2"]),
        );
        assert_eq!(r, Validated::Invalid(vec!["e1", "e2"]));
    }

    #[test]
    fn test_apply_accumulates() {
        let vf: Validated<fn(i32) -> i32, &str> = Validated::Invalid(vec!["e1"]);
        let va: Validated<i32, &str> = Validated::Invalid(vec!["e2"]);
        let r = apply(vf, va);
        assert_eq!(r, Validated::Invalid(vec!["e1", "e2"]));
    }
}
