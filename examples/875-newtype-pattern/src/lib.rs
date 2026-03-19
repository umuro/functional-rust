// Example 081: Newtype Pattern
// Rust tuple structs for type safety

use std::fmt;

// === Approach 1: Simple newtype wrappers ===
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct UserId(u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct OrderId(u64);

impl UserId {
    fn new(id: u64) -> Option<Self> {
        if id > 0 {
            Some(UserId(id))
        } else {
            None
        }
    }
    fn value(self) -> u64 {
        self.0
    }
}

impl OrderId {
    fn new(id: u64) -> Option<Self> {
        if id > 0 {
            Some(OrderId(id))
        } else {
            None
        }
    }
    fn value(self) -> u64 {
        self.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "User#{}", self.0)
    }
}

impl fmt::Display for OrderId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Order#{}", self.0)
    }
}

// Can't accidentally pass UserId where OrderId expected!
fn process_order(order: OrderId, user: UserId) -> String {
    format!("{} placed {}", user, order)
}

// === Approach 2: Newtype with validation ===
#[derive(Debug, Clone, PartialEq)]
struct Email(String);

impl Email {
    fn new(s: &str) -> Option<Self> {
        if s.contains('@') {
            Some(Email(s.to_string()))
        } else {
            None
        }
    }
    fn as_str(&self) -> &str {
        &self.0
    }
}

// === Approach 3: Newtype for unit safety ===
#[derive(Debug, Clone, Copy, PartialEq)]
struct Celsius(f64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Fahrenheit(f64);

impl Celsius {
    fn new(v: f64) -> Self {
        Celsius(v)
    }
    fn to_fahrenheit(self) -> Fahrenheit {
        Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
    }
    fn value(self) -> f64 {
        self.0
    }
}

impl Fahrenheit {
    fn new(v: f64) -> Self {
        Fahrenheit(v)
    }
    fn to_celsius(self) -> Celsius {
        Celsius((self.0 - 32.0) * 5.0 / 9.0)
    }
    fn value(self) -> f64 {
        self.0
    }
}

// Implement Deref for transparent access when appropriate
use std::ops::Deref;

#[derive(Debug, Clone, PartialEq)]
struct NonEmptyString(String);

impl NonEmptyString {
    fn new(s: &str) -> Option<Self> {
        if s.is_empty() {
            None
        } else {
            Some(NonEmptyString(s.to_string()))
        }
    }
}

impl Deref for NonEmptyString {
    type Target = str;
    fn deref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_order_type_safety() {
        let u = UserId::new(1).unwrap();
        let o = OrderId::new(2).unwrap();
        assert_eq!(u.value(), 1);
        assert_eq!(o.value(), 2);
        // UserId and OrderId are different types
    }

    #[test]
    fn test_invalid_ids() {
        assert!(UserId::new(0).is_none());
        assert!(OrderId::new(0).is_none());
    }

    #[test]
    fn test_email_validation() {
        assert!(Email::new("a@b.com").is_some());
        assert!(Email::new("invalid").is_none());
    }

    #[test]
    fn test_temperature_conversion() {
        let c = Celsius::new(0.0);
        assert!((c.to_fahrenheit().value() - 32.0).abs() < 1e-10);

        let f = Fahrenheit::new(212.0);
        assert!((f.to_celsius().value() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_roundtrip() {
        let c = Celsius::new(37.0);
        let back = c.to_fahrenheit().to_celsius();
        assert!((back.value() - 37.0).abs() < 1e-10);
    }

    #[test]
    fn test_non_empty_string() {
        assert!(NonEmptyString::new("hello").is_some());
        assert!(NonEmptyString::new("").is_none());
        let s = NonEmptyString::new("test").unwrap();
        assert_eq!(s.len(), 4); // Deref to &str
    }
}
