// 1003: Custom Error Types
// Custom error type with Display + Error impl

use std::fmt;

// Approach 1: Simple error enum with Display
#[derive(Debug, PartialEq)]
enum ValidationError {
    NegativeAge(i32),
    UnreasonableAge(i32),
    EmptyName,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::NegativeAge(n) => write!(f, "negative age: {}", n),
            ValidationError::UnreasonableAge(n) => write!(f, "unreasonable age: {}", n),
            ValidationError::EmptyName => write!(f, "name cannot be empty"),
        }
    }
}

impl std::error::Error for ValidationError {}

fn validate_age(age: i32) -> Result<i32, ValidationError> {
    if age < 0 {
        Err(ValidationError::NegativeAge(age))
    } else if age > 150 {
        Err(ValidationError::UnreasonableAge(age))
    } else {
        Ok(age)
    }
}

fn validate_name(name: &str) -> Result<&str, ValidationError> {
    if name.is_empty() {
        Err(ValidationError::EmptyName)
    } else {
        Ok(name)
    }
}

// Approach 2: Error with structured context
#[derive(Debug)]
struct DetailedError {
    field: String,
    message: String,
}

impl fmt::Display for DetailedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "field '{}': {}", self.field, self.message)
    }
}

impl std::error::Error for DetailedError {}

fn validate_field(field: &str, value: &str) -> Result<(), DetailedError> {
    if value.is_empty() {
        Err(DetailedError {
            field: field.to_string(),
            message: "cannot be empty".to_string(),
        })
    } else {
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_age() {
        assert_eq!(validate_age(25), Ok(25));
    }

    #[test]
    fn test_negative_age() {
        assert_eq!(validate_age(-5), Err(ValidationError::NegativeAge(-5)));
    }

    #[test]
    fn test_unreasonable_age() {
        assert_eq!(validate_age(200), Err(ValidationError::UnreasonableAge(200)));
    }

    #[test]
    fn test_display_impl() {
        let err = ValidationError::NegativeAge(-1);
        assert_eq!(err.to_string(), "negative age: -1");

        let err = ValidationError::EmptyName;
        assert_eq!(err.to_string(), "name cannot be empty");
    }

    #[test]
    fn test_error_trait() {
        let err: Box<dyn std::error::Error> = Box::new(ValidationError::EmptyName);
        assert_eq!(err.to_string(), "name cannot be empty");
    }

    #[test]
    fn test_validate_name() {
        assert_eq!(validate_name("Alice"), Ok("Alice"));
        assert_eq!(validate_name(""), Err(ValidationError::EmptyName));
    }

    #[test]
    fn test_detailed_error() {
        let result = validate_field("email", "");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "field 'email': cannot be empty");
    }
}
