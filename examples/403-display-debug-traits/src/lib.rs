#![allow(clippy::all)]
//! Display and Debug Traits
//!
//! Two formatting traits: Debug for developers, Display for users.

use std::fmt;

/// A color type demonstrating Display, Debug, and LowerHex formatting.
#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
}

impl Color {
    /// Creates a color from RGB values.
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color::Rgb(r, g, b)
    }

    /// Returns the RGB components.
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Red => (255, 0, 0),
            Color::Green => (0, 255, 0),
            Color::Blue => (0, 0, 255),
            Color::Rgb(r, g, b) => (*r, *g, *b),
        }
    }
}

/// Display: user-facing output (used by `{}` and `println!`)
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::Red => write!(f, "red"),
            Color::Green => write!(f, "green"),
            Color::Blue => write!(f, "blue"),
            Color::Rgb(r, g, b) => write!(f, "rgb({},{},{})", r, g, b),
        }
    }
}

/// LowerHex: hexadecimal format (used by `{:x}`)
impl fmt::LowerHex for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (r, g, b) = self.to_rgb();
        write!(f, "#{:02x}{:02x}{:02x}", r, g, b)
    }
}

/// A 2D point demonstrating precision-aware Display.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Creates a new point.
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    /// Calculates distance from origin.
    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Respect precision from format string if specified
        match f.precision() {
            Some(p) => write!(f, "({:.prec$}, {:.prec$})", self.x, self.y, prec = p),
            None => write!(f, "({:.2}, {:.2})", self.x, self.y),
        }
    }
}

/// A wrapper that formats its contents in binary.
pub struct Binary<T>(pub T);

impl fmt::Display for Binary<u8> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08b}", self.0)
    }
}

impl fmt::Display for Binary<u16> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:016b}", self.0)
    }
}

/// A person struct showing the difference between Debug and Display.
#[derive(Clone)]
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn new(name: &str, age: u32) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({} years old)", self.name, self.age)
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Person")
            .field("name", &self.name)
            .field("age", &self.age)
            .finish()
    }
}

/// Demonstrates alignment and padding.
pub fn format_aligned(s: &str, width: usize) -> (String, String, String) {
    (
        format!("{:>width$}", s, width = width), // right-aligned
        format!("{:<width$}", s, width = width), // left-aligned
        format!("{:^width$}", s, width = width), // centered
    )
}

/// Demonstrates numeric formatting.
pub fn format_number(n: u32) -> String {
    format!("dec:{} hex:{:#x} oct:{:#o} bin:{:#b}", n, n, n, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_display() {
        assert_eq!(format!("{}", Color::Red), "red");
        assert_eq!(format!("{}", Color::Green), "green");
        assert_eq!(format!("{}", Color::Blue), "blue");
        assert_eq!(format!("{}", Color::Rgb(10, 20, 30)), "rgb(10,20,30)");
    }

    #[test]
    fn test_color_debug() {
        assert_eq!(format!("{:?}", Color::Red), "Red");
        assert_eq!(format!("{:?}", Color::Rgb(1, 2, 3)), "Rgb(1, 2, 3)");
    }

    #[test]
    fn test_color_hex() {
        assert_eq!(format!("{:x}", Color::Red), "#ff0000");
        assert_eq!(format!("{:x}", Color::Green), "#00ff00");
        assert_eq!(format!("{:x}", Color::Blue), "#0000ff");
        assert_eq!(format!("{:x}", Color::Rgb(128, 64, 32)), "#804020");
    }

    #[test]
    fn test_color_to_rgb() {
        assert_eq!(Color::Red.to_rgb(), (255, 0, 0));
        assert_eq!(Color::rgb(100, 150, 200).to_rgb(), (100, 150, 200));
    }

    #[test]
    fn test_point_display_default() {
        let p = Point::new(3.14159, 2.71828);
        assert_eq!(format!("{}", p), "(3.14, 2.72)");
    }

    #[test]
    fn test_point_display_precision() {
        let p = Point::new(1.23456, 7.89012);
        assert_eq!(format!("{:.1}", p), "(1.2, 7.9)");
        assert_eq!(format!("{:.4}", p), "(1.2346, 7.8901)");
    }

    #[test]
    fn test_point_debug() {
        let p = Point::new(1.0, 2.0);
        assert_eq!(format!("{:?}", p), "Point { x: 1.0, y: 2.0 }");
    }

    #[test]
    fn test_binary_display() {
        assert_eq!(format!("{}", Binary(5u8)), "00000101");
        assert_eq!(format!("{}", Binary(255u8)), "11111111");
    }

    #[test]
    fn test_person_display_vs_debug() {
        let p = Person::new("Alice", 30);
        assert_eq!(format!("{}", p), "Alice (30 years old)");
        assert_eq!(format!("{:?}", p), "Person { name: \"Alice\", age: 30 }");
    }

    #[test]
    fn test_format_aligned() {
        let (right, left, center) = format_aligned("hi", 6);
        assert_eq!(right, "    hi");
        assert_eq!(left, "hi    ");
        assert_eq!(center, "  hi  ");
    }

    #[test]
    fn test_format_number() {
        let s = format_number(255);
        assert!(s.contains("dec:255"));
        assert!(s.contains("hex:0xff"));
        assert!(s.contains("oct:0o377"));
        assert!(s.contains("bin:0b11111111"));
    }

    #[test]
    fn test_pretty_debug() {
        let p = Point::new(1.0, 2.0);
        let pretty = format!("{:#?}", p);
        assert!(pretty.contains("Point"));
        assert!(pretty.contains("x:"));
        assert!(pretty.contains("y:"));
    }
}
