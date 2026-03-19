#![allow(clippy::all)]
//! From, Into, TryFrom, TryInto Traits
//!
//! Type conversion traits for infallible and fallible conversions.

use std::fmt;

/// Temperature in Celsius.
#[derive(Debug, Clone, PartialEq)]
pub struct Celsius(pub f64);

/// Temperature in Fahrenheit.
#[derive(Debug, Clone, PartialEq)]
pub struct Fahrenheit(pub f64);

/// Kelvin temperature.
#[derive(Debug, Clone, PartialEq)]
pub struct Kelvin(pub f64);

// Infallible conversions using From

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Self {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

impl From<Celsius> for Kelvin {
    fn from(c: Celsius) -> Self {
        Kelvin(c.0 + 273.15)
    }
}

impl From<Kelvin> for Celsius {
    fn from(k: Kelvin) -> Self {
        Celsius(k.0 - 273.15)
    }
}

/// A validated positive integer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PositiveInt(u32);

impl PositiveInt {
    /// Returns the inner value.
    pub fn value(&self) -> u32 {
        self.0
    }
}

/// Error for non-positive values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotPositiveError;

impl fmt::Display for NotPositiveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "value must be positive (> 0)")
    }
}

impl std::error::Error for NotPositiveError {}

// Fallible conversion using TryFrom

impl TryFrom<i32> for PositiveInt {
    type Error = NotPositiveError;

    fn try_from(n: i32) -> Result<Self, Self::Error> {
        if n > 0 {
            Ok(PositiveInt(n as u32))
        } else {
            Err(NotPositiveError)
        }
    }
}

impl TryFrom<i64> for PositiveInt {
    type Error = NotPositiveError;

    fn try_from(n: i64) -> Result<Self, Self::Error> {
        if n > 0 && n <= u32::MAX as i64 {
            Ok(PositiveInt(n as u32))
        } else {
            Err(NotPositiveError)
        }
    }
}

/// A valid network port (1024-65535 for non-privileged).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port(u16);

impl Port {
    /// Returns the port number.
    pub fn number(&self) -> u16 {
        self.0
    }
}

/// Error for invalid port values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortError {
    ParseError(String),
    TooLow(u16),
}

impl fmt::Display for PortError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PortError::ParseError(s) => write!(f, "invalid port: {}", s),
            PortError::TooLow(n) => write!(f, "port {} is below 1024", n),
        }
    }
}

impl TryFrom<&str> for Port {
    type Error = PortError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let n: u16 = s
            .parse()
            .map_err(|_| PortError::ParseError(s.to_string()))?;
        if n >= 1024 {
            Ok(Port(n))
        } else {
            Err(PortError::TooLow(n))
        }
    }
}

impl TryFrom<u32> for Port {
    type Error = PortError;

    fn try_from(n: u32) -> Result<Self, Self::Error> {
        if (1024..=65535).contains(&n) {
            Ok(Port(n as u16))
        } else if n < 1024 {
            Err(PortError::TooLow(n as u16))
        } else {
            Err(PortError::ParseError(format!("{} exceeds u16", n)))
        }
    }
}

/// Demonstrates the `as` keyword for primitive casts.
pub fn primitive_casts() -> (i32, u8, f64) {
    let a: i64 = 1000;
    let b: i32 = a as i32; // truncating cast

    let c: i32 = 300;
    let d: u8 = c as u8; // wrapping cast (300 % 256 = 44)

    let e: i32 = 42;
    let f: f64 = e as f64; // widening cast

    (b, d, f)
}

/// Generic function accepting anything convertible to String.
pub fn greet<S: Into<String>>(name: S) {
    let name = name.into();
    println!("Hello, {}!", name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        let c = Celsius(0.0);
        let f: Fahrenheit = c.into();
        assert!((f.0 - 32.0).abs() < 0.001);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        let f = Fahrenheit(212.0);
        let c = Celsius::from(f);
        assert!((c.0 - 100.0).abs() < 0.001);
    }

    #[test]
    fn test_celsius_to_kelvin() {
        let c = Celsius(0.0);
        let k: Kelvin = c.into();
        assert!((k.0 - 273.15).abs() < 0.001);
    }

    #[test]
    fn test_kelvin_to_celsius() {
        let k = Kelvin(0.0);
        let c: Celsius = k.into();
        assert!((c.0 - (-273.15)).abs() < 0.001);
    }

    #[test]
    fn test_positive_int_success() {
        let p: Result<PositiveInt, _> = 42i32.try_into();
        assert!(p.is_ok());
        assert_eq!(p.unwrap().value(), 42);
    }

    #[test]
    fn test_positive_int_zero_fails() {
        let p: Result<PositiveInt, _> = 0i32.try_into();
        assert!(p.is_err());
    }

    #[test]
    fn test_positive_int_negative_fails() {
        let p: Result<PositiveInt, _> = (-5i32).try_into();
        assert!(p.is_err());
    }

    #[test]
    fn test_port_valid() {
        let p = Port::try_from("8080");
        assert!(p.is_ok());
        assert_eq!(p.unwrap().number(), 8080);
    }

    #[test]
    fn test_port_below_1024() {
        let p = Port::try_from("80");
        assert!(matches!(p, Err(PortError::TooLow(80))));
    }

    #[test]
    fn test_port_invalid_string() {
        let p = Port::try_from("abc");
        assert!(matches!(p, Err(PortError::ParseError(_))));
    }

    #[test]
    fn test_port_from_u32() {
        let p = Port::try_from(3000u32);
        assert!(p.is_ok());
        assert_eq!(p.unwrap().number(), 3000);
    }

    #[test]
    fn test_primitive_casts() {
        let (b, d, f) = primitive_casts();
        assert_eq!(b, 1000);
        assert_eq!(d, 44); // 300 % 256
        assert_eq!(f, 42.0);
    }

    #[test]
    fn test_stdlib_try_from() {
        // Standard library TryFrom for primitive narrowing
        let ok: Result<u8, _> = u8::try_from(100i32);
        assert_eq!(ok, Ok(100u8));

        let err: Result<u8, _> = u8::try_from(1000i32);
        assert!(err.is_err());
    }
}
