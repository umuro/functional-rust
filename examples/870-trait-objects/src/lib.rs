// Example 076: Trait Objects — Dynamic Dispatch
// OCaml polymorphism → Rust dyn Trait vs generics

use std::f64::consts::PI;

// === Approach 1: Trait objects (dyn Trait) — dynamic dispatch ===
trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
    fn name(&self) -> &str {
        "Circle"
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn name(&self) -> &str {
        "Rectangle"
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
    fn name(&self) -> &str {
        "Triangle"
    }
}

// Dynamic dispatch: accepts any Shape via trait object
fn total_area_dyn(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn describe_dyn(shape: &dyn Shape) -> String {
    format!("{}: area={:.2}", shape.name(), shape.area())
}

// === Approach 2: Generics (static dispatch / monomorphization) ===
fn describe_generic<S: Shape>(shape: &S) -> String {
    format!("{}: area={:.2}", shape.name(), shape.area())
}

fn total_area_generic(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// === Approach 3: Enum dispatch (like OCaml ADT) ===
enum ShapeEnum {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

impl ShapeEnum {
    fn area(&self) -> f64 {
        match self {
            ShapeEnum::Circle(r) => PI * r * r,
            ShapeEnum::Rectangle(w, h) => w * h,
            ShapeEnum::Triangle(b, h) => 0.5 * b * h,
        }
    }

    fn name(&self) -> &str {
        match self {
            ShapeEnum::Circle(_) => "Circle",
            ShapeEnum::Rectangle(_, _) => "Rectangle",
            ShapeEnum::Triangle(_, _) => "Triangle",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_area() {
        let c = Circle { radius: 5.0 };
        assert!((c.area() - PI * 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_rectangle_area() {
        let r = Rectangle {
            width: 3.0,
            height: 4.0,
        };
        assert!((r.area() - 12.0).abs() < 1e-10);
    }

    #[test]
    fn test_triangle_area() {
        let t = Triangle {
            base: 6.0,
            height: 3.0,
        };
        assert!((t.area() - 9.0).abs() < 1e-10);
    }

    #[test]
    fn test_dynamic_dispatch_total() {
        let c = Circle { radius: 5.0 };
        let r = Rectangle {
            width: 3.0,
            height: 4.0,
        };
        let t = Triangle {
            base: 6.0,
            height: 3.0,
        };
        let shapes: Vec<&dyn Shape> = vec![&c, &r, &t];
        let total = total_area_dyn(&shapes);
        let expected = PI * 25.0 + 12.0 + 9.0;
        assert!((total - expected).abs() < 1e-10);
    }

    #[test]
    fn test_enum_dispatch() {
        let c = ShapeEnum::Circle(5.0);
        assert!((c.area() - PI * 25.0).abs() < 1e-10);
        assert_eq!(c.name(), "Circle");
    }

    #[test]
    fn test_describe() {
        let c = Circle { radius: 1.0 };
        let desc = describe_generic(&c);
        assert!(desc.starts_with("Circle"));
    }

    #[test]
    fn test_boxed_shapes() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Circle { radius: 1.0 }),
            Box::new(Rectangle {
                width: 2.0,
                height: 3.0,
            }),
        ];
        let total = total_area_generic(&shapes);
        assert!((total - (PI + 6.0)).abs() < 1e-10);
    }
}
