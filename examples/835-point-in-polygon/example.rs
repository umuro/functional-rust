/// Point-in-Polygon: Ray Casting Algorithm.
///
/// Shoot a ray from P in the +x direction; count edge crossings.
/// Odd crossings = inside, even = outside. Boundary points detected separately.

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self { Point { x, y } }
}

#[derive(Debug, PartialEq)]
enum Location {
    Inside,
    Outside,
    OnBoundary,
}

const EPS: f64 = 1e-10;

/// Test if point P lies on segment AB.
fn on_segment(p: &Point, a: &Point, b: &Point) -> bool {
    // Cross product near zero AND P within bounding box of AB
    let cross = (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x);
    if cross.abs() > EPS { return false; }
    p.x >= f64::min(a.x, b.x) - EPS
        && p.x <= f64::max(a.x, b.x) + EPS
        && p.y >= f64::min(a.y, b.y) - EPS
        && p.y <= f64::max(a.y, b.y) + EPS
}

/// Ray casting: find location of P relative to polygon.
fn point_in_polygon(p: &Point, polygon: &[Point]) -> Location {
    let n = polygon.len();
    if n == 0 { return Location::Outside; }

    let mut crossings = 0usize;

    for i in 0..n {
        let a = &polygon[i];
        let b = &polygon[(i + 1) % n];

        // Boundary check
        if on_segment(p, a, b) {
            return Location::OnBoundary;
        }

        // Ray crossing: edge straddles P.y and crossing is to the right
        if (a.y <= p.y && b.y > p.y) || (b.y <= p.y && a.y > p.y) {
            let x_cross = a.x + (p.y - a.y) / (b.y - a.y) * (b.x - a.x);
            if p.x < x_cross {
                crossings += 1;
            }
        }
    }

    if crossings % 2 == 1 {
        Location::Inside
    } else {
        Location::Outside
    }
}

fn main() {
    // Unit square [0,2]×[0,2]
    let square = vec![
        Point::new(0.0, 0.0), Point::new(2.0, 0.0),
        Point::new(2.0, 2.0), Point::new(0.0, 2.0),
    ];

    let tests = [
        (Point::new(1.0, 1.0), "centre"),
        (Point::new(3.0, 1.0), "right outside"),
        (Point::new(0.0, 0.0), "corner"),
        (Point::new(1.0, 0.0), "bottom edge"),
        (Point::new(1.0, 2.5), "above"),
    ];
    println!("Square [0,2]×[0,2]:");
    for (p, label) in &tests {
        println!("  ({},{}) [{}]: {:?}", p.x, p.y, label, point_in_polygon(p, &square));
    }

    // L-shaped polygon
    let l_shape = vec![
        Point::new(0.0, 0.0), Point::new(2.0, 0.0), Point::new(2.0, 1.0),
        Point::new(1.0, 1.0), Point::new(1.0, 2.0), Point::new(0.0, 2.0),
    ];
    println!("\nL-shape:");
    let l_tests = [
        (Point::new(0.5, 0.5), "in bottom-left"),
        (Point::new(1.5, 1.5), "in top-right notch (outside)"),
        (Point::new(0.5, 1.5), "in top-left"),
    ];
    for (p, label) in &l_tests {
        println!("  ({},{}) [{}]: {:?}", p.x, p.y, label, point_in_polygon(p, &l_shape));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square() -> Vec<Point> {
        vec![
            Point::new(0.0, 0.0), Point::new(2.0, 0.0),
            Point::new(2.0, 2.0), Point::new(0.0, 2.0),
        ]
    }

    #[test]
    fn test_inside() {
        assert_eq!(point_in_polygon(&Point::new(1.0, 1.0), &square()), Location::Inside);
    }

    #[test]
    fn test_outside() {
        assert_eq!(point_in_polygon(&Point::new(3.0, 1.0), &square()), Location::Outside);
        assert_eq!(point_in_polygon(&Point::new(-1.0, 1.0), &square()), Location::Outside);
    }

    #[test]
    fn test_boundary_edge() {
        assert_eq!(point_in_polygon(&Point::new(1.0, 0.0), &square()), Location::OnBoundary);
    }

    #[test]
    fn test_boundary_corner() {
        assert_eq!(point_in_polygon(&Point::new(0.0, 0.0), &square()), Location::OnBoundary);
    }

    #[test]
    fn test_l_shape() {
        let l = vec![
            Point::new(0.0, 0.0), Point::new(2.0, 0.0), Point::new(2.0, 1.0),
            Point::new(1.0, 1.0), Point::new(1.0, 2.0), Point::new(0.0, 2.0),
        ];
        assert_eq!(point_in_polygon(&Point::new(0.5, 0.5), &l), Location::Inside);
        assert_eq!(point_in_polygon(&Point::new(1.5, 1.5), &l), Location::Outside);
        assert_eq!(point_in_polygon(&Point::new(0.5, 1.5), &l), Location::Inside);
    }

    #[test]
    fn test_triangle() {
        let tri = vec![
            Point::new(0.0, 0.0), Point::new(4.0, 0.0), Point::new(2.0, 4.0),
        ];
        assert_eq!(point_in_polygon(&Point::new(2.0, 1.0), &tri), Location::Inside);
        assert_eq!(point_in_polygon(&Point::new(3.5, 3.0), &tri), Location::Outside);
    }
}
