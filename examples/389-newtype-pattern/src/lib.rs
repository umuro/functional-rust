#![allow(clippy::all)]
//! Newtype Pattern

use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Meters(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kilograms(pub f64);

impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}m", self.0)
    }
}
impl fmt::Display for Kilograms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2}kg", self.0)
    }
}

impl Add for Meters {
    type Output = Meters;
    fn add(self, o: Meters) -> Meters {
        Meters(self.0 + o.0)
    }
}

pub struct Email(String);
impl Email {
    pub fn new(s: &str) -> Option<Self> {
        if s.contains('@') && s.contains('.') {
            Some(Email(s.to_string()))
        } else {
            None
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meters_display() {
        assert_eq!(format!("{}", Meters(5.0)), "5.00m");
    }
    #[test]
    fn test_meters_add() {
        assert_eq!(Meters(2.0) + Meters(3.0), Meters(5.0));
    }
    #[test]
    fn test_email_valid() {
        assert!(Email::new("a@b.com").is_some());
    }
    #[test]
    fn test_email_invalid() {
        assert!(Email::new("invalid").is_none());
    }
}
