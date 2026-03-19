// 054: Applicative Validation
// Collect all validation errors instead of stopping at the first

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

#[derive(Debug, PartialEq, Clone)]
enum ValidationError {
    NameEmpty,
    NameTooLong,
    AgeNegative,
    AgeUnrealistic,
    EmailInvalid,
}

// Approach 1: Individual validators
fn validate_name(name: &str) -> Result<String, Vec<ValidationError>> {
    if name.is_empty() {
        Err(vec![ValidationError::NameEmpty])
    } else if name.len() > 50 {
        Err(vec![ValidationError::NameTooLong])
    } else {
        Ok(name.to_string())
    }
}

fn validate_age(age: i32) -> Result<u32, Vec<ValidationError>> {
    if age < 0 {
        Err(vec![ValidationError::AgeNegative])
    } else if age > 150 {
        Err(vec![ValidationError::AgeUnrealistic])
    } else {
        Ok(age as u32)
    }
}

fn validate_email(email: &str) -> Result<String, Vec<ValidationError>> {
    if !email.contains('@') {
        Err(vec![ValidationError::EmailInvalid])
    } else {
        Ok(email.to_string())
    }
}

// Approach 2: Collect all errors
fn validate_person(name: &str, age: i32, email: &str) -> Result<Person, Vec<ValidationError>> {
    let mut errors = Vec::new();
    let name_result = validate_name(name);
    let age_result = validate_age(age);
    let email_result = validate_email(email);

    if let Err(ref e) = name_result {
        errors.extend(e.iter().cloned());
    }
    if let Err(ref e) = age_result {
        errors.extend(e.iter().cloned());
    }
    if let Err(ref e) = email_result {
        errors.extend(e.iter().cloned());
    }

    if errors.is_empty() {
        Ok(Person {
            name: name_result.unwrap(),
            age: age_result.unwrap(),
            email: email_result.unwrap(),
        })
    } else {
        Err(errors)
    }
}

// Approach 3: Using a Validated type
enum Validated<T> {
    Valid(T),
    Invalid(Vec<ValidationError>),
}

impl<T> Validated<T> {
    fn and_then<U>(self, f: impl FnOnce(T) -> Validated<U>) -> Validated<U> {
        match self {
            Validated::Valid(x) => f(x),
            Validated::Invalid(e) => Validated::Invalid(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_person() {
        let result = validate_person("Alice", 30, "alice@example.com");
        assert!(result.is_ok());
        let p = result.unwrap();
        assert_eq!(p.name, "Alice");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_all_errors_collected() {
        let result = validate_person("", -5, "bad");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().len(), 3);
    }

    #[test]
    fn test_partial_errors() {
        let result = validate_person("Bob", 25, "bad");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().len(), 1);
    }
}
