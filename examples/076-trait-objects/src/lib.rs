// 076: Trait Objects — dynamic dispatch with dyn Trait

use std::f64::consts::PI;

// Approach 1: Define trait and implementations
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { PI * self.radius * self.radius }
    fn name(&self) -> &str { "circle" }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
    fn name(&self) -> &str { "rectangle" }
}

// Approach 2: Using dyn Trait for polymorphism
fn describe(shape: &dyn Shape) -> String {
    format!("{} with area {:.2}", shape.name(), shape.area())
}

fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// Approach 3: Returning trait objects
fn make_shape(kind: &str) -> Box<dyn Shape> {
    match kind {
        "circle" => Box::new(Circle { radius: 5.0 }),
        "rectangle" => Box::new(Rectangle { width: 3.0, height: 4.0 }),
        _ => Box::new(Circle { radius: 1.0 }),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let c = Circle { radius: 5.0 };
        assert!((c.area() - 78.54).abs() < 0.01);
        assert_eq!(c.name(), "circle");
    }

    #[test]
    fn test_rectangle() {
        let r = Rectangle { width: 3.0, height: 4.0 };
        assert_eq!(r.area(), 12.0);
    }

    #[test]
    fn test_dyn_dispatch() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Circle { radius: 5.0 }),
            Box::new(Rectangle { width: 3.0, height: 4.0 }),
        ];
        assert!((total_area(&shapes) - 90.54).abs() < 0.01);
    }

    #[test]
    fn test_make_shape() {
        let s = make_shape("circle");
        assert_eq!(s.name(), "circle");
    }
}
