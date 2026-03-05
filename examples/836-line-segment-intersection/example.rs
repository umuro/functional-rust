/// Line Segment Intersection Detection.
///
/// Uses cross-product sign tests to determine if two segments AB and CD
/// intersect. No division until we need the actual point.

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self { Point { x, y } }
}

/// Cross product of vectors (b - a) and (c - a).
fn cross(a: Point, b: Point, c: Point) -> f64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn sign(x: f64) -> i32 {
    if x > 0.0 { 1 } else if x < 0.0 { -1 } else { 0 }
}

/// Is point p on segment [a, b]? (assumes p is collinear with a, b)
fn on_segment(a: Point, b: Point, p: Point) -> bool {
    p.x >= f64::min(a.x, b.x) && p.x <= f64::max(a.x, b.x)
        && p.y >= f64::min(a.y, b.y) && p.y <= f64::max(a.y, b.y)
}

/// Do segments AB and CD intersect (including endpoint touches)?
pub fn segments_intersect(a: Point, b: Point, c: Point, d: Point) -> bool {
    let d1 = cross(c, d, a);
    let d2 = cross(c, d, b);
    let d3 = cross(a, b, c);
    let d4 = cross(a, b, d);

    // Proper intersection: each segment straddles the other's line
    if sign(d1) * sign(d2) < 0 && sign(d3) * sign(d4) < 0 {
        return true;
    }

    // Collinear/endpoint cases
    if d1 == 0.0 && on_segment(c, d, a) { return true; }
    if d2 == 0.0 && on_segment(c, d, b) { return true; }
    if d3 == 0.0 && on_segment(a, b, c) { return true; }
    if d4 == 0.0 && on_segment(a, b, d) { return true; }

    false
}

/// Compute actual intersection point (None if parallel or disjoint).
pub fn intersection_point(a: Point, b: Point, c: Point, d: Point) -> Option<Point> {
    let denom = (b.x - a.x) * (d.y - c.y) - (b.y - a.y) * (d.x - c.x);
    if denom.abs() < 1e-12 { return None; } // Parallel

    let t = ((c.x - a.x) * (d.y - c.y) - (c.y - a.y) * (d.x - c.x)) / denom;
    let s = ((c.x - a.x) * (b.y - a.y) - (c.y - a.y) * (b.x - a.x)) / denom;

    if t < 0.0 || t > 1.0 || s < 0.0 || s > 1.0 {
        return None;
    }

    Some(Point::new(
        a.x + t * (b.x - a.x),
        a.y + t * (b.y - a.y),
    ))
}

fn main() {
    let cases: &[(Point, Point, Point, Point, bool)] = &[
        // X-crossing
        (Point::new(0.0, 0.0), Point::new(2.0, 2.0),
         Point::new(0.0, 2.0), Point::new(2.0, 0.0), true),
        // Parallel, same y
        (Point::new(0.0, 0.0), Point::new(1.0, 0.0),
         Point::new(2.0, 0.0), Point::new(3.0, 0.0), false),
        // T-intersection
        (Point::new(0.0, 0.0), Point::new(2.0, 0.0),
         Point::new(1.0, -1.0), Point::new(1.0, 1.0), true),
        // Near-miss
        (Point::new(0.0, 0.0), Point::new(1.0, 1.0),
         Point::new(1.0, 0.0), Point::new(2.0, 1.0), false),
    ];

    for &(a, b, c, d, expected) in cases {
        let result = segments_intersect(a, b, c, d);
        let pt = intersection_point(a, b, c, d);
        print!("({},{})-({},{}) vs ({},{})-({},{}): {} (expected {})",
            a.x, a.y, b.x, b.y, c.x, c.y, d.x, d.y, result, expected);
        if let Some(p) = pt {
            print!("  → intersection at ({:.2},{:.2})", p.x, p.y);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_crossing() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(2.0, 2.0);
        let c = Point::new(0.0, 2.0);
        let d = Point::new(2.0, 0.0);
        assert!(segments_intersect(a, b, c, d));
        let p = intersection_point(a, b, c, d).unwrap();
        assert!((p.x - 1.0).abs() < 1e-9);
        assert!((p.y - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_no_intersection() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(1.0, 0.0);
        let c = Point::new(0.0, 1.0);
        let d = Point::new(1.0, 1.0);
        assert!(!segments_intersect(a, b, c, d));
    }

    #[test]
    fn test_parallel_no_overlap() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(1.0, 0.0);
        let c = Point::new(2.0, 0.0);
        let d = Point::new(3.0, 0.0);
        assert!(!segments_intersect(a, b, c, d));
    }

    #[test]
    fn test_collinear_overlap() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(2.0, 0.0);
        let c = Point::new(1.0, 0.0);
        let d = Point::new(3.0, 0.0);
        assert!(segments_intersect(a, b, c, d));
    }

    #[test]
    fn test_endpoint_touch() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(1.0, 1.0);
        let c = Point::new(1.0, 1.0);
        let d = Point::new(2.0, 0.0);
        assert!(segments_intersect(a, b, c, d));
    }

    #[test]
    fn test_t_intersection() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(2.0, 0.0);
        let c = Point::new(1.0, -1.0);
        let d = Point::new(1.0, 1.0);
        assert!(segments_intersect(a, b, c, d));
    }

    #[test]
    fn test_cross_product() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(2.0, 0.0);
        let above = Point::new(1.0, 1.0);
        let below = Point::new(1.0, -1.0);
        assert!(cross(a, b, above) > 0.0);
        assert!(cross(a, b, below) < 0.0);
    }
}
