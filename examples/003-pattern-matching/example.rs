/// Pattern Matching: the heart of both OCaml and Rust.
///
/// Both languages use pattern matching as a primary control flow mechanism.
/// OCaml has algebraic data types; Rust has enums. The mapping is remarkably direct.

// ── Define a Shape type (algebraic data type / enum) ────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Circle(f64),              // radius
    Rectangle(f64, f64),      // width, height
    Triangle(f64, f64, f64),  // three sides
}

// ── Idiomatic Rust: match expressions ───────────────────────────────────────

/// Calculate area using match — direct analog of OCaml's pattern matching
pub fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(a, b, c) => {
            // Heron's formula
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

/// Describe a shape — demonstrates string formatting in match arms
pub fn describe(shape: &Shape) -> String {
    match shape {
        Shape::Circle(r) => format!("Circle with radius {r}"),
        Shape::Rectangle(w, h) if (w - h).abs() < f64::EPSILON => {
            format!("Square with side {w}")
        }
        Shape::Rectangle(w, h) => format!("Rectangle {w}×{h}"),
        Shape::Triangle(a, b, c) if (a - b).abs() < f64::EPSILON
            && (b - c).abs() < f64::EPSILON => {
            format!("Equilateral triangle with side {a}")
        }
        Shape::Triangle(a, b, c) => format!("Triangle with sides {a}, {b}, {c}"),
    }
}

// ── Nested pattern matching with Option ─────────────────────────────────────

/// Find the largest shape by area from an optional list
pub fn largest_area(shapes: &[Shape]) -> Option<f64> {
    // Uses iterator + fold, but the interesting bit is Option handling
    shapes.iter()
        .map(|s| area(s))
        .fold(None, |max, a| match max {
            None => Some(a),
            Some(m) if a > m => Some(a),
            _ => max,
        })
}

// ── Recursive style with exhaustive matching ────────────────────────────────

/// Count shapes of each type — recursive traversal with pattern matching
pub fn count_by_type(shapes: &[Shape]) -> (usize, usize, usize) {
    fn aux(shapes: &[Shape], c: usize, r: usize, t: usize) -> (usize, usize, usize) {
        match shapes.split_first() {
            None => (c, r, t),
            Some((Shape::Circle(_), rest)) => aux(rest, c + 1, r, t),
            Some((Shape::Rectangle(_, _), rest)) => aux(rest, c, r + 1, t),
            Some((Shape::Triangle(_, _, _), rest)) => aux(rest, c, r, t + 1),
        }
    }
    aux(shapes, 0, 0, 0)
}

// ── Functional style with iterators ─────────────────────────────────────────

/// Scale all shapes by a factor — map with pattern matching inside
pub fn scale_all(shapes: &[Shape], factor: f64) -> Vec<Shape> {
    shapes.iter().map(|s| match s {
        Shape::Circle(r) => Shape::Circle(r * factor),
        Shape::Rectangle(w, h) => Shape::Rectangle(w * factor, h * factor),
        Shape::Triangle(a, b, c) => Shape::Triangle(a * factor, b * factor, c * factor),
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_area_circle() {
        let c = Shape::Circle(5.0);
        assert!((area(&c) - PI * 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_area_rectangle() {
        assert!((area(&Shape::Rectangle(3.0, 4.0)) - 12.0).abs() < 1e-10);
    }

    #[test]
    fn test_area_triangle() {
        // 3-4-5 right triangle, area = 6
        assert!((area(&Shape::Triangle(3.0, 4.0, 5.0)) - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_describe_with_guards() {
        assert_eq!(describe(&Shape::Rectangle(5.0, 5.0)), "Square with side 5");
        assert_eq!(describe(&Shape::Triangle(3.0, 3.0, 3.0)), "Equilateral triangle with side 3");
        assert!(describe(&Shape::Rectangle(3.0, 4.0)).contains("×"));
    }

    #[test]
    fn test_largest_area_empty() {
        assert_eq!(largest_area(&[]), None);
    }

    #[test]
    fn test_largest_area_nonempty() {
        let shapes = vec![Shape::Circle(1.0), Shape::Rectangle(10.0, 10.0)];
        assert!((largest_area(&shapes).unwrap() - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_count_by_type() {
        let shapes = vec![
            Shape::Circle(1.0), Shape::Circle(2.0),
            Shape::Rectangle(1.0, 2.0),
            Shape::Triangle(3.0, 4.0, 5.0),
        ];
        assert_eq!(count_by_type(&shapes), (2, 1, 1));
        assert_eq!(count_by_type(&[]), (0, 0, 0));
    }

    #[test]
    fn test_scale() {
        let shapes = vec![Shape::Circle(2.0), Shape::Rectangle(3.0, 4.0)];
        let scaled = scale_all(&shapes, 2.0);
        assert_eq!(scaled[0], Shape::Circle(4.0));
        assert_eq!(scaled[1], Shape::Rectangle(6.0, 8.0));
    }
}

fn main() {
    println!("{:?}", (area(&c) - PI * 25.0).abs() < 1e-10);
    println!("{:?}", (area(&Shape::Rectangle(3.0, 4.0)) - 12.0).abs() < 1e-10);
    println!("{:?}", (area(&Shape::Triangle(3.0, 4.0, 5.0)) - 6.0).abs() < 1e-10);
}
