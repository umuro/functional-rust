// 1015: Validation Errors — Accumulating All Errors
// Not short-circuiting: collect ALL validation failures

#[derive(Debug, Clone, PartialEq)]
struct FieldError {
    field: String,
    message: String,
}

impl std::fmt::Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)
    }
}

// Approach 1: Collect errors from each validator
fn validate_name(name: &str) -> Vec<FieldError> {
    let mut errors = Vec::new();
    if name.is_empty() {
        errors.push(FieldError {
            field: "name".into(),
            message: "required".into(),
        });
    }
    if name.len() > 50 {
        errors.push(FieldError {
            field: "name".into(),
            message: "too long".into(),
        });
    }
    errors
}

fn validate_age(age: i32) -> Vec<FieldError> {
    let mut errors = Vec::new();
    if age < 0 {
        errors.push(FieldError {
            field: "age".into(),
            message: "negative".into(),
        });
    }
    if age > 150 {
        errors.push(FieldError {
            field: "age".into(),
            message: "unreasonable".into(),
        });
    }
    errors
}

fn validate_email(email: &str) -> Vec<FieldError> {
    let mut errors = Vec::new();
    if email.is_empty() {
        errors.push(FieldError {
            field: "email".into(),
            message: "required".into(),
        });
    }
    if !email.contains('@') {
        errors.push(FieldError {
            field: "email".into(),
            message: "missing @".into(),
        });
    }
    errors
}

#[derive(Debug, PartialEq)]
struct ValidForm {
    name: String,
    age: i32,
    email: String,
}

fn validate_form(name: &str, age: i32, email: &str) -> Result<ValidForm, Vec<FieldError>> {
    let mut errors = Vec::new();
    errors.extend(validate_name(name));
    errors.extend(validate_age(age));
    errors.extend(validate_email(email));

    if errors.is_empty() {
        Ok(ValidForm {
            name: name.to_string(),
            age,
            email: email.to_string(),
        })
    } else {
        Err(errors)
    }
}

// Approach 2: Functional with iterators
fn validate_field<T>(field: &str, value: &T, checks: &[(fn(&T) -> bool, &str)]) -> Vec<FieldError> {
    checks
        .iter()
        .filter(|(pred, _)| !pred(value))
        .map(|(_, msg)| FieldError {
            field: field.to_string(),
            message: msg.to_string(),
        })
        .collect()
}

fn validate_form_functional(
    name: &str,
    age: i32,
    email: &str,
) -> Result<ValidForm, Vec<FieldError>> {
    let name_checks: Vec<(fn(&&str) -> bool, &str)> = vec![
        (|s: &&str| !s.is_empty(), "required"),
        (|s: &&str| s.len() <= 50, "too long"),
    ];
    let age_checks: Vec<(fn(&i32) -> bool, &str)> = vec![
        (|n: &i32| *n >= 0, "negative"),
        (|n: &i32| *n <= 150, "unreasonable"),
    ];

    let errors: Vec<FieldError> = [
        validate_field("name", &name, &name_checks),
        validate_field("age", &age, &age_checks),
        validate_email(email),
    ]
    .concat();

    if errors.is_empty() {
        Ok(ValidForm {
            name: name.into(),
            age,
            email: email.into(),
        })
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_errors_collected() {
        let result = validate_form("", -5, "bademail");
        let errors = result.unwrap_err();
        assert!(errors.len() >= 3);
        assert!(errors.iter().any(|e| e.field == "name"));
        assert!(errors.iter().any(|e| e.field == "age"));
        assert!(errors.iter().any(|e| e.field == "email"));
    }

    #[test]
    fn test_valid_form() {
        let result = validate_form("Alice", 30, "a@b.com");
        assert!(result.is_ok());
        let form = result.unwrap();
        assert_eq!(form.name, "Alice");
        assert_eq!(form.age, 30);
    }

    #[test]
    fn test_single_error() {
        let result = validate_form("Alice", 30, "no-at");
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].field, "email");
    }

    #[test]
    fn test_functional_approach() {
        let result = validate_form_functional("", -1, "bad");
        assert!(result.is_err());
        assert!(result.unwrap_err().len() >= 3);

        let result = validate_form_functional("Bob", 25, "b@c.com");
        assert!(result.is_ok());
    }

    #[test]
    fn test_no_short_circuit() {
        // Key: unlike ?, ALL fields are checked even after first error
        let errors = validate_form("", -5, "").unwrap_err();
        // name error + age error + email errors — all present
        assert!(errors.len() >= 4); // empty name + negative age + empty email + missing @
    }
}
