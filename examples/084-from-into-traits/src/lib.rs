#![allow(clippy::all)]
// 084: From and Into Traits

// Approach 1: Temperature conversion
#[derive(Debug, Clone, Copy)]
struct Celsius(f64);

#[derive(Debug, Clone, Copy)]
struct Fahrenheit(f64);

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

// Approach 2: Enum from string (TryFrom)
#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Green,
    Blue,
}

impl TryFrom<&str> for Color {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "red" => Ok(Color::Red),
            "green" => Ok(Color::Green),
            "blue" => Ok(Color::Blue),
            _ => Err(format!("Unknown color: {}", s)),
        }
    }
}

impl From<Color> for &str {
    fn from(c: Color) -> Self {
        match c {
            Color::Red => "red",
            Color::Green => "green",
            Color::Blue => "blue",
        }
    }
}

// Approach 3: From for complex types
struct RawUser {
    name: String,
    age: String,
    email: String,
}

#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u32,
    email: String,
}

impl TryFrom<RawUser> for User {
    type Error = String;
    fn try_from(raw: RawUser) -> Result<Self, Self::Error> {
        let age = raw.age.parse().map_err(|_| "Invalid age".to_string())?;
        Ok(User {
            name: raw.name,
            age,
            email: raw.email,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        let f: Fahrenheit = Celsius(100.0).into();
        assert!((f.0 - 212.0).abs() < 0.001);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        let c: Celsius = Fahrenheit(32.0).into();
        assert!(c.0.abs() < 0.001);
    }

    #[test]
    fn test_color_try_from() {
        assert_eq!(Color::try_from("red"), Ok(Color::Red));
        assert!(Color::try_from("purple").is_err());
    }

    #[test]
    fn test_user_try_from() {
        let raw = RawUser {
            name: "Alice".into(),
            age: "30".into(),
            email: "a@b.com".into(),
        };
        let user = User::try_from(raw).unwrap();
        assert_eq!(user.age, 30);
    }

    #[test]
    fn test_user_invalid() {
        let raw = RawUser {
            name: "Bob".into(),
            age: "xyz".into(),
            email: "b@c.com".into(),
        };
        assert!(User::try_from(raw).is_err());
    }
}
