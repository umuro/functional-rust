//! # Custom Serialize Logic
//!
//! When you need special serialization behavior.

use std::fmt::Write;

/// Date with custom serialization format
#[derive(Debug, PartialEq, Clone)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Date { year, month, day }
    }

    /// Serialize as ISO 8601 string
    pub fn to_iso_string(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// Serialize as compact integer (YYYYMMDD)
    pub fn to_compact(&self) -> u32 {
        self.year as u32 * 10000 + self.month as u32 * 100 + self.day as u32
    }

    /// Parse from ISO string
    pub fn from_iso_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return None;
        }
        Some(Date {
            year: parts[0].parse().ok()?,
            month: parts[1].parse().ok()?,
            day: parts[2].parse().ok()?,
        })
    }

    /// Parse from compact integer
    pub fn from_compact(n: u32) -> Self {
        Date {
            year: (n / 10000) as u16,
            month: ((n / 100) % 100) as u8,
            day: (n % 100) as u8,
        }
    }
}

/// Money with custom decimal serialization
#[derive(Debug, PartialEq, Clone)]
pub struct Money {
    /// Amount in cents
    cents: i64,
    currency: String,
}

impl Money {
    pub fn new(cents: i64, currency: &str) -> Self {
        Money {
            cents,
            currency: currency.to_string(),
        }
    }

    pub fn from_dollars(dollars: f64, currency: &str) -> Self {
        Money {
            cents: (dollars * 100.0).round() as i64,
            currency: currency.to_string(),
        }
    }

    pub fn to_display(&self) -> String {
        let dollars = self.cents / 100;
        let cents = (self.cents % 100).abs();
        if self.cents < 0 {
            format!("-{}.{:02} {}", dollars.abs(), cents, self.currency)
        } else {
            format!("{}.{:02} {}", dollars, cents, self.currency)
        }
    }

    /// Serialize as JSON object
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"cents": {}, "currency": "{}"}}"#,
            self.cents, self.currency
        )
    }
}

/// Secret value that redacts in serialization
#[derive(Clone)]
pub struct Secret<T> {
    value: T,
}

impl<T> Secret<T> {
    pub fn new(value: T) -> Self {
        Secret { value }
    }

    pub fn expose(&self) -> &T {
        &self.value
    }
}

impl<T> std::fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Secret([REDACTED])")
    }
}

impl<T> std::fmt::Display for Secret<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[REDACTED]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_iso() {
        let date = Date::new(2024, 3, 15);
        assert_eq!(date.to_iso_string(), "2024-03-15");
    }

    #[test]
    fn test_date_compact() {
        let date = Date::new(2024, 3, 15);
        assert_eq!(date.to_compact(), 20240315);
    }

    #[test]
    fn test_date_roundtrip_iso() {
        let original = Date::new(2024, 12, 25);
        let s = original.to_iso_string();
        let parsed = Date::from_iso_string(&s).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_date_roundtrip_compact() {
        let original = Date::new(2024, 1, 1);
        let n = original.to_compact();
        let parsed = Date::from_compact(n);
        assert_eq!(original, parsed);
    }

    #[test]
    fn test_money_display() {
        let m = Money::new(1234, "USD");
        assert_eq!(m.to_display(), "12.34 USD");
    }

    #[test]
    fn test_money_negative() {
        let m = Money::new(-1234, "EUR");
        assert_eq!(m.to_display(), "-12.34 EUR");
    }

    #[test]
    fn test_money_from_dollars() {
        let m = Money::from_dollars(19.99, "USD");
        assert_eq!(m.cents, 1999);
    }

    #[test]
    fn test_money_json() {
        let m = Money::new(500, "GBP");
        assert_eq!(m.to_json(), r#"{"cents": 500, "currency": "GBP"}"#);
    }

    #[test]
    fn test_secret_redacted() {
        let secret = Secret::new("password123");
        let debug_output = format!("{:?}", secret);
        assert!(!debug_output.contains("password123"));
        assert!(debug_output.contains("REDACTED"));
    }

    #[test]
    fn test_secret_expose() {
        let secret = Secret::new(42);
        assert_eq!(*secret.expose(), 42);
    }
}
