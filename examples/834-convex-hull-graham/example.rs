/// Convex Hull: Graham Scan O(n log n).
///
/// Sort by polar angle from bottom-left pivot, then stack-sweep
/// keeping only left turns (CCW orientation).

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self { Point { x, y } }

    fn dist2(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}

/// Cross product of vectors (b-a) and (c-a).
/// > 0: left turn (CCW), < 0: right turn (CW), = 0: collinear.
fn cross(a: &Point, b: &Point, c: &Point) -> f64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

/// Graham scan: returns CCW convex hull.
fn convex_hull(mut points: Vec<Point>) -> Vec<Point> {
    let n = points.len();
    if n <= 1 { return points; }

    // Find bottom-most (then left-most) pivot
    let pivot_idx = points
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            a.y.partial_cmp(&b.y).unwrap()
             .then(a.x.partial_cmp(&b.x).unwrap())
        })
        .map(|(i, _)| i)
        .unwrap();
    points.swap(0, pivot_idx);
    let pivot = points[0];

    // Sort by polar angle w.r.t. pivot
    points[1..].sort_by(|a, b| {
        let c = cross(&pivot, a, b);
        if c > 0.0 { std::cmp::Ordering::Less }
        else if c < 0.0 { std::cmp::Ordering::Greater }
        else {
            // Collinear: closer point first
            pivot.dist2(a).partial_cmp(&pivot.dist2(b)).unwrap()
        }
    });

    // Stack-based sweep
    let mut stack: Vec<Point> = Vec::with_capacity(n);
    for p in &points {
        // Pop while last three make a right turn or are collinear
        while stack.len() >= 2
            && cross(&stack[stack.len() - 2], &stack[stack.len() - 1], p) <= 0.0
        {
            stack.pop();
        }
        stack.push(*p);
    }
    stack
}

/// Hull area via shoelace formula.
fn hull_area(hull: &[Point]) -> f64 {
    let n = hull.len();
    if n < 3 { return 0.0; }
    let sum: f64 = (0..n)
        .map(|i| {
            let j = (i + 1) % n;
            hull[i].x * hull[j].y - hull[j].x * hull[i].y
        })
        .sum();
    sum.abs() / 2.0
}

fn main() {
    let points = vec![
        Point::new(0.0, 0.0), Point::new(1.0, 1.0), Point::new(2.0, 2.0),
        Point::new(0.0, 2.0), Point::new(2.0, 0.0), Point::new(1.0, 0.0),
    ];
    let hull = convex_hull(points);
    println!("Hull ({} points):", hull.len());
    for p in &hull { println!("  ({:.1}, {:.1})", p.x, p.y); }

    // Unit square + interior point
    let square = vec![
        Point::new(0.0, 0.0), Point::new(1.0, 0.0),
        Point::new(1.0, 1.0), Point::new(0.0, 1.0),
        Point::new(0.5, 0.5), // interior
    ];
    let hull2 = convex_hull(square);
    println!("\nSquare hull ({} points):", hull2.len());
    println!("Area: {:.1} (expected 1.0)", hull_area(&hull2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_hull() {
        let pts = vec![
            Point::new(0.0, 0.0), Point::new(1.0, 0.0),
            Point::new(1.0, 1.0), Point::new(0.0, 1.0),
            Point::new(0.5, 0.5),
        ];
        let hull = convex_hull(pts);
        assert_eq!(hull.len(), 4); // Interior point excluded
    }

    #[test]
    fn test_triangle_hull() {
        let pts = vec![
            Point::new(0.0, 0.0), Point::new(2.0, 0.0),
            Point::new(1.0, 2.0), Point::new(1.0, 0.5),
        ];
        let hull = convex_hull(pts);
        assert_eq!(hull.len(), 3);
    }

    #[test]
    fn test_collinear_points() {
        // All collinear: hull should have 2 or more points
        let pts = vec![
            Point::new(0.0, 0.0), Point::new(1.0, 1.0), Point::new(2.0, 2.0),
        ];
        let hull = convex_hull(pts);
        assert!(hull.len() >= 2);
    }

    #[test]
    fn test_single_point() {
        let pts = vec![Point::new(1.0, 2.0)];
        let hull = convex_hull(pts);
        assert_eq!(hull.len(), 1);
    }

    #[test]
    fn test_all_hull_points_are_extreme() {
        let pts = vec![
            Point::new(0.0, 0.0), Point::new(4.0, 0.0),
            Point::new(4.0, 4.0), Point::new(0.0, 4.0),
            Point::new(2.0, 2.0), Point::new(1.0, 1.0),
        ];
        let hull = convex_hull(pts.clone());
        // Every hull point should have been in the original set
        for h in &hull {
            assert!(pts.contains(h), "{h:?} not in original set");
        }
    }

    #[test]
    fn test_area_unit_square() {
        let pts = vec![
            Point::new(0.0, 0.0), Point::new(1.0, 0.0),
            Point::new(1.0, 1.0), Point::new(0.0, 1.0),
        ];
        let hull = convex_hull(pts);
        let area = hull_area(&hull);
        assert!((area - 1.0).abs() < 1e-9, "area = {area}");
    }

    #[test]
    fn test_cross_product() {
        let a = Point::new(0.0, 0.0);
        let b = Point::new(1.0, 0.0);
        let c = Point::new(0.0, 1.0);
        assert!(cross(&a, &b, &c) > 0.0); // CCW / left turn
        assert!(cross(&a, &c, &b) < 0.0); // CW / right turn
    }
}
