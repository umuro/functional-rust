// Newtype pattern in Rust
use std::fmt;
use std::ops::Add;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Meters(f64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Kilograms(f64);

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct Seconds(f64);

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
    fn add(self, other: Meters) -> Meters { Meters(self.0 + other.0) }
}

// Domain-specific validated type
#[derive(Debug, Clone)]
struct Email(String);

impl Email {
    fn new(s: &str) -> Option<Self> {
        if s.contains('@') && s.contains('.') {
            Some(Email(s.to_string()))
        } else {
            None
        }
    }

    fn as_str(&self) -> &str { &self.0 }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn bmi(mass: Kilograms, height: Meters) -> f64 {
    mass.0 / (height.0 * height.0)
}

// Type safety: bmi(Meters(1.75), Kilograms(70.0)) would NOT compile!

fn main() {
    let height = Meters(1.75);
    let weight = Kilograms(70.0);
    println!("Height: {}", height);
    println!("Weight: {}", weight);
    println!("BMI: {:.1}", bmi(weight, height));

    let distance = Meters(5.0) + Meters(3.5);
    println!("Distance: {}", distance);

    match Email::new("user@example.com") {
        Some(email) => println!("Valid email: {}", email),
        None => println!("Invalid email"),
    }

    println!("Size of Meters: {} (same as f64: {})",
             std::mem::size_of::<Meters>(),
             std::mem::size_of::<f64>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_newtype_safety() {
        let m = Meters(5.0);
        let k = Kilograms(70.0);
        // These are different types — compiler enforces distinction
        assert_ne!(m.0, k.0 as f64); // just to use both
        assert_eq!(m + Meters(3.0), Meters(8.0));
    }

    #[test]
    fn test_email_validation() {
        assert!(Email::new("a@b.com").is_some());
        assert!(Email::new("invalid").is_none());
    }

    #[test]
    fn test_transparent_size() {
        assert_eq!(std::mem::size_of::<Meters>(), std::mem::size_of::<f64>());
    }
}
