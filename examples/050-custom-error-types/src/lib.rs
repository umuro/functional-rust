#![allow(clippy::all)]
// A custom error enum with Display + std::error::Error for self-documenting failures.
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ValidationError {
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

pub fn validate_age(age: i32) -> Result<i32, ValidationError> {
    if age < 0 {
        Err(ValidationError::NegativeAge(age))
    } else if age > 150 {
        Err(ValidationError::UnreasonableAge(age))
    } else {
        Ok(age)
    }
}

pub fn validate_name(name: &str) -> Result<&str, ValidationError> {
    if name.is_empty() {
        Err(ValidationError::EmptyName)
    } else {
        Ok(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_age_ok() {
        assert_eq!(validate_age(30), Ok(30));
    }

    #[test]
    fn test_validate_age_negative() {
        assert_eq!(validate_age(-1), Err(ValidationError::NegativeAge(-1)));
    }

    #[test]
    fn test_validate_age_unreasonable() {
        assert_eq!(validate_age(200), Err(ValidationError::UnreasonableAge(200)));
    }

    #[test]
    fn test_validate_name() {
        assert_eq!(validate_name("Ada"), Ok("Ada"));
        assert_eq!(validate_name(""), Err(ValidationError::EmptyName));
    }

    #[test]
    fn test_display_messages() {
        assert_eq!(ValidationError::NegativeAge(-5).to_string(), "negative age: -5");
        assert_eq!(ValidationError::UnreasonableAge(200).to_string(), "unreasonable age: 200");
        assert_eq!(ValidationError::EmptyName.to_string(), "name cannot be empty");
    }

    #[test]
    fn test_is_a_std_error() {
        let e: Box<dyn std::error::Error> = Box::new(ValidationError::EmptyName);
        assert_eq!(e.to_string(), "name cannot be empty");
    }
}
