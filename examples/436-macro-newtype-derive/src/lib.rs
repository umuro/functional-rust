//! Newtype Derive Patterns
//!
//! Generating trait impls for newtypes.

/// Newtype wrapper for validated email.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    pub fn new(s: &str) -> Result<Self, &'static str> {
        if s.contains('@') {
            Ok(Email(s.to_string()))
        } else {
            Err("Invalid email")
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Newtype for positive integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PositiveInt(u32);

impl PositiveInt {
    pub fn new(n: u32) -> Option<Self> {
        if n > 0 {
            Some(PositiveInt(n))
        } else {
            None
        }
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_valid() {
        let e = Email::new("test@example.com").unwrap();
        assert_eq!(e.as_str(), "test@example.com");
    }

    #[test]
    fn test_email_invalid() {
        assert!(Email::new("invalid").is_err());
    }

    #[test]
    fn test_positive_int_valid() {
        let p = PositiveInt::new(42).unwrap();
        assert_eq!(p.get(), 42);
    }

    #[test]
    fn test_positive_int_zero() {
        assert!(PositiveInt::new(0).is_none());
    }

    #[test]
    fn test_positive_int_ord() {
        let a = PositiveInt::new(1).unwrap();
        let b = PositiveInt::new(2).unwrap();
        assert!(a < b);
    }
}
