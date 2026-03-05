// From, Into, TryFrom, TryInto in Rust
use std::convert::{TryFrom, TryInto};
use std::fmt;

// Custom type conversions
#[derive(Debug, Clone, PartialEq)]
struct Celsius(f64);

#[derive(Debug, Clone, PartialEq)]
struct Fahrenheit(f64);

// From (infallible)
impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Fahrenheit {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Celsius {
        Celsius((f.0 - 32.0) * 5.0 / 9.0)
    }
}

// TryFrom (fallible)
#[derive(Debug, PartialEq)]
struct PositiveInt(u32);

#[derive(Debug)]
struct NegativeError;

impl fmt::Display for NegativeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "value must be positive") }
}

impl TryFrom<i32> for PositiveInt {
    type Error = NegativeError;
    fn try_from(n: i32) -> Result<Self, NegativeError> {
        if n > 0 { Ok(PositiveInt(n as u32)) }
        else { Err(NegativeError) }
    }
}

// String parsing with TryFrom
#[derive(Debug)]
struct Port(u16);

impl TryFrom<&str> for Port {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, String> {
        let n: u16 = s.parse().map_err(|_| format!("Invalid port: {}", s))?;
        if n >= 1024 { Ok(Port(n)) } else { Err(format!("Port {} below 1024", n)) }
    }
}

fn main() {
    // From / Into (infallible)
    let boiling = Celsius(100.0);
    let f: Fahrenheit = boiling.into(); // uses From<Celsius> for Fahrenheit
    println!("100°C = {:.1}°F", f.0);

    let body_temp = Fahrenheit(98.6);
    let c = Celsius::from(body_temp);
    println!("98.6°F = {:.1}°C", c.0);

    // TryFrom / TryInto (fallible)
    let pos: Result<PositiveInt, _> = 42i32.try_into();
    println!("TryInto 42: {:?}", pos);

    let neg: Result<PositiveInt, _> = (-1i32).try_into();
    println!("TryInto -1: {:?}", neg);

    // Port parsing
    let ports = ["8080", "80", "invalid", "65535"];
    for p in &ports {
        match Port::try_from(*p) {
            Ok(port) => println!("Port OK: {}", port.0),
            Err(e) => println!("Port Error: {}", e),
        }
    }

    // Standard library TryFrom examples
    let big: i32 = 1000;
    let small: Result<u8, _> = u8::try_from(big);
    println!("1000 -> u8: {:?}", small); // Err (overflow)

    let ok: Result<u8, _> = u8::try_from(100i32);
    println!("100 -> u8: {:?}", ok); // Ok(100)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_fahrenheit() {
        let c = Celsius(0.0);
        let f: Fahrenheit = c.into();
        assert!((f.0 - 32.0).abs() < 0.001);
    }

    #[test]
    fn test_try_from_positive() {
        assert!(PositiveInt::try_from(1i32).is_ok());
        assert!(PositiveInt::try_from(0i32).is_err());
        assert!(PositiveInt::try_from(-5i32).is_err());
    }

    #[test]
    fn test_port_try_from() {
        assert!(Port::try_from("8080").is_ok());
        assert!(Port::try_from("80").is_err()); // below 1024
        assert!(Port::try_from("xyz").is_err());
    }
}
