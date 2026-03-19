/// Records — Immutable Update and Pattern Matching
///
/// OCaml's `{ r with field = value }` functional update syntax maps directly
/// to Rust's struct update syntax `Struct { field: value, ..old }`.
/// Both create a new value without mutating the original.

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub origin: Point,
    pub width: f64,
    pub height: f64,
}

/// Area via destructuring — mirrors OCaml's `let area { width; height; _ }`.
pub fn area(r: &Rect) -> f64 {
    r.width * r.height
}

pub fn perimeter(r: &Rect) -> f64 {
    2.0 * (r.width + r.height)
}

/// Functional update: creates a new Rect with a shifted origin.
/// Uses Rust's `..r` struct update syntax, analogous to OCaml's `{ r with ... }`.
pub fn translate(dx: f64, dy: f64, r: &Rect) -> Rect {
    Rect {
        origin: Point {
            x: r.origin.x + dx,
            y: r.origin.y + dy,
        },
        ..*r
    }
}

pub fn contains_point(r: &Rect, p: &Point) -> bool {
    p.x >= r.origin.x
        && p.x <= r.origin.x + r.width
        && p.y >= r.origin.y
        && p.y <= r.origin.y + r.height
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_rect() -> Rect {
        Rect {
            origin: Point { x: 0.0, y: 0.0 },
            width: 10.0,
            height: 5.0,
        }
    }

    #[test]
    fn test_area() {
        assert!((area(&sample_rect()) - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_perimeter() {
        assert!((perimeter(&sample_rect()) - 30.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_translate() {
        let r2 = translate(3.0, 4.0, &sample_rect());
        assert!((r2.origin.x - 3.0).abs() < f64::EPSILON);
        assert!((r2.origin.y - 4.0).abs() < f64::EPSILON);
        assert!((r2.width - 10.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_contains_point() {
        let r = sample_rect();
        assert!(contains_point(&r, &Point { x: 1.0, y: 1.0 }));
        assert!(!contains_point(&r, &Point { x: 11.0, y: 1.0 }));
        assert!(contains_point(&r, &Point { x: 0.0, y: 0.0 })); // edge
        assert!(contains_point(&r, &Point { x: 10.0, y: 5.0 })); // corner
    }

    #[test]
    fn test_immutability() {
        let r = sample_rect();
        let r2 = translate(1.0, 1.0, &r);
        // Original unchanged — Rust's Copy trait means no move
        assert!((r.origin.x - 0.0).abs() < f64::EPSILON);
        assert!((r2.origin.x - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_zero_size_rect() {
        let r = Rect {
            origin: Point { x: 5.0, y: 5.0 },
            width: 0.0,
            height: 0.0,
        };
        assert!((area(&r)).abs() < f64::EPSILON);
        assert!(contains_point(&r, &Point { x: 5.0, y: 5.0 }));
    }
}
