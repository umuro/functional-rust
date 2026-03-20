#![allow(clippy::all)]
//! Derive Macro Patterns
//!
//! Common patterns for derive macros.

/// Simulating what derive macros generate.

/// A simple newtype for demonstration.
pub struct Meters(pub f64);

/// What #[derive(Add)] might generate:
impl std::ops::Add for Meters {
    type Output = Meters;
    fn add(self, other: Meters) -> Meters {
        Meters(self.0 + other.0)
    }
}

/// What #[derive(From)] might generate for newtype:
impl From<f64> for Meters {
    fn from(v: f64) -> Meters {
        Meters(v)
    }
}

/// What #[derive(Into)] provides automatically with From:
impl From<Meters> for f64 {
    fn from(m: Meters) -> f64 {
        m.0
    }
}

/// Example enum for dispatch.
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

impl Shape {
    pub fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meters_add() {
        let a = Meters(1.0);
        let b = Meters(2.0);
        let c = a + b;
        assert_eq!(c.0, 3.0);
    }

    #[test]
    fn test_meters_from() {
        let m: Meters = 5.0.into();
        assert_eq!(m.0, 5.0);
    }

    #[test]
    fn test_meters_into() {
        let m = Meters(10.0);
        let f: f64 = m.into();
        assert_eq!(f, 10.0);
    }

    #[test]
    fn test_circle_area() {
        let s = Shape::Circle { radius: 1.0 };
        assert!((s.area() - std::f64::consts::PI).abs() < 0.001);
    }

    #[test]
    fn test_rectangle_area() {
        let s = Shape::Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert_eq!(s.area(), 12.0);
    }
}
