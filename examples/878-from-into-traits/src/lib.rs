#![allow(clippy::all)]
// Example 084: From/Into Traits
// OCaml coercion → Rust explicit conversions

use std::fmt;

// === Approach 1: From trait for type conversions ===
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

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.0)
    }
}

impl fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°F", self.0)
    }
}

// === Approach 2: From for string parsing (TryFrom for fallible) ===
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl TryFrom<&str> for Point {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.trim_start_matches('(').trim_end_matches(')');
        let parts: Vec<&str> = s.split(',').map(str::trim).collect();
        if parts.len() != 2 {
            return Err("Expected (x, y)".to_string());
        }
        let x = parts[0]
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        let y = parts[1]
            .parse()
            .map_err(|e: std::num::ParseIntError| e.to_string())?;
        Ok(Point { x, y })
    }
}

// From<Point> for (i32, i32)
impl From<Point> for (i32, i32) {
    fn from(p: Point) -> Self {
        (p.x, p.y)
    }
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point { x, y }
    }
}

// === Approach 3: Into in generic contexts ===
fn print_temperature<T: Into<Celsius>>(temp: T) {
    let c: Celsius = temp.into();
    println!("Temperature: {}", c);
}

// From/Into chain
fn fahrenheit_string_to_celsius(s: &str) -> Result<String, String> {
    let val: f64 = s
        .parse()
        .map_err(|e: std::num::ParseFloatError| e.to_string())?;
    let c: Celsius = Fahrenheit(val).into(); // Into comes free from From
    Ok(format!("{}", c))
}

// Collecting with From
fn strings_to_points(data: &[(i32, i32)]) -> Vec<Point> {
    data.iter().copied().map(Point::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit() {
        let f: Fahrenheit = Celsius(100.0).into();
        assert!((f.0 - 212.0).abs() < 1e-10);
    }

    #[test]
    fn test_fahrenheit_to_celsius() {
        let c: Celsius = Fahrenheit(32.0).into();
        assert!((c.0 - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_roundtrip() {
        let original = Celsius(37.0);
        let back: Celsius = Fahrenheit::from(original).into();
        assert!((back.0 - 37.0).abs() < 1e-10);
    }

    #[test]
    fn test_point_try_from() {
        assert_eq!(Point::try_from("(3, 4)"), Ok(Point { x: 3, y: 4 }));
        assert!(Point::try_from("invalid").is_err());
    }

    #[test]
    fn test_point_from_tuple() {
        let p: Point = (1, 2).into();
        assert_eq!(p, Point { x: 1, y: 2 });
    }

    #[test]
    fn test_tuple_from_point() {
        let t: (i32, i32) = Point { x: 5, y: 6 }.into();
        assert_eq!(t, (5, 6));
    }

    #[test]
    fn test_fahrenheit_string_to_celsius() {
        let result = fahrenheit_string_to_celsius("212");
        assert_eq!(result, Ok("100.0°C".to_string()));
        assert!(fahrenheit_string_to_celsius("abc").is_err());
    }

    #[test]
    fn test_strings_to_points() {
        let pts = strings_to_points(&[(1, 2), (3, 4)]);
        assert_eq!(pts.len(), 2);
        assert_eq!(pts[0], Point { x: 1, y: 2 });
    }
}
