// Supertraits and trait inheritance in Rust
use std::fmt;

// Base trait
trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

// Supertrait: Shape is required
trait LabeledShape: Shape + fmt::Display {
    fn label(&self) -> &str;
    fn describe(&self) -> String {
        // Default method can use supertrait methods
        format!("[{}] area={:.2}, perimeter={:.2}", self.label(), self.area(), self.perimeter())
    }
}

// Another layer
trait ColoredShape: LabeledShape {
    fn color(&self) -> &str;
    fn full_description(&self) -> String {
        format!("{} (color: {})", self.describe(), self.color())
    }
}

#[derive(Debug)]
struct Circle { radius: f64 }
#[derive(Debug)]
struct Rectangle { width: f64, height: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn perimeter(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "Circle(r={})", self.radius) }
}
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "Rect({}x{})", self.width, self.height) }
}

impl LabeledShape for Circle {
    fn label(&self) -> &str { "Circle" }
}

impl LabeledShape for Rectangle {
    fn label(&self) -> &str { "Rectangle" }
    fn describe(&self) -> String {
        format!("[Rect] {}x{}, area={:.2}", self.width, self.height, self.area())
    }
}

impl ColoredShape for Circle {
    fn color(&self) -> &str { "red" }
}

fn print_shape(s: &dyn LabeledShape) {
    println!("{}", s.describe());
}

fn main() {
    let c = Circle { radius: 5.0 };
    let r = Rectangle { width: 4.0, height: 3.0 };

    print_shape(&c);
    print_shape(&r);
    println!("{}", c.full_description());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supertrait_methods() {
        let c = Circle { radius: 1.0 };
        // Can call Shape methods through LabeledShape bound
        assert!((c.area() - std::f64::consts::PI).abs() < 1e-9);
        assert_eq!(c.label(), "Circle");
    }

    #[test]
    fn test_describe() {
        let r = Rectangle { width: 3.0, height: 4.0 };
        let desc = r.describe();
        assert!(desc.contains("Rect"));
    }
}
