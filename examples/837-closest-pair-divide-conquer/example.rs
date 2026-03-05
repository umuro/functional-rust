/// Closest Pair of Points — Divide and Conquer O(n log n).
///
/// Split by median x, recurse, then check the strip of width δ around the split.
/// At most 8 points in any δ×2δ box → strip check is O(n).

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self { Point { x, y } }
    fn dist(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Brute-force for small n.
fn brute_force(pts: &[Point]) -> f64 {
    let n = pts.len();
    let mut best = f64::INFINITY;
    for i in 0..n {
        for j in i + 1..n {
            best = best.min(pts[i].dist(&pts[j]));
        }
    }
    best
}

/// Check strip: points within delta of mid_x, sorted by y.
fn strip_closest(strip: &mut Vec<Point>, delta: f64) -> f64 {
    strip.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
    let mut best = delta;
    let n = strip.len();
    for i in 0..n {
        let mut j = i + 1;
        // At most 7 comparisons per point (geometric packing)
        while j < n && strip[j].y - strip[i].y < best {
            best = best.min(strip[i].dist(&strip[j]));
            j += 1;
        }
    }
    best
}

/// Recursive divide-and-conquer. pts_x must be sorted by x.
fn closest_rec(pts_x: &[Point]) -> f64 {
    let n = pts_x.len();
    if n <= 3 {
        return brute_force(pts_x);
    }

    let mid = n / 2;
    let mid_x = pts_x[mid].x;

    let dl = closest_rec(&pts_x[..mid]);
    let dr = closest_rec(&pts_x[mid..]);
    let mut delta = dl.min(dr);

    // Collect strip: points within delta of the dividing line
    let mut strip: Vec<Point> = pts_x
        .iter()
        .filter(|p| (p.x - mid_x).abs() < delta)
        .copied()
        .collect();

    delta = delta.min(strip_closest(&mut strip, delta));
    delta
}

/// Public API: find minimum distance among all pairs of points.
pub fn closest_pair(points: &[Point]) -> f64 {
    if points.len() < 2 { return f64::INFINITY; }
    let mut sorted = points.to_vec();
    sorted.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    closest_rec(&sorted)
}

fn main() {
    let points = vec![
        Point::new(2.0, 3.0),  Point::new(12.0, 30.0),
        Point::new(40.0, 50.0), Point::new(5.0, 1.0),
        Point::new(12.0, 10.0), Point::new(3.0, 4.0),
    ];

    let d = closest_pair(&points);
    let bf = brute_force(&points);
    println!("Closest pair (D&C):    {:.6}", d);
    println!("Closest pair (brute):  {:.6}", bf);
    println!("Match: {}", (d - bf).abs() < 1e-9);

    // Edge case: two points
    let two = vec![Point::new(0.0, 0.0), Point::new(3.0, 4.0)];
    println!("Two points distance: {} (expected 5.0)", closest_pair(&two));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matches_brute(pts: Vec<Point>) {
        let d = closest_pair(&pts);
        let bf = brute_force(&pts);
        assert!((d - bf).abs() < 1e-9,
            "d&c={d} brute={bf} for {pts:?}");
    }

    #[test]
    fn test_basic() {
        matches_brute(vec![
            Point::new(2.0, 3.0), Point::new(12.0, 30.0),
            Point::new(40.0, 50.0), Point::new(5.0, 1.0),
            Point::new(12.0, 10.0), Point::new(3.0, 4.0),
        ]);
    }

    #[test]
    fn test_two_points() {
        let d = closest_pair(&[Point::new(0.0, 0.0), Point::new(3.0, 4.0)]);
        assert!((d - 5.0).abs() < 1e-9);
    }

    #[test]
    fn test_collinear() {
        let pts: Vec<Point> = (0..10).map(|i| Point::new(i as f64, 0.0)).collect();
        matches_brute(pts);
    }

    #[test]
    fn test_grid() {
        let pts: Vec<Point> = (0..5)
            .flat_map(|i| (0..5).map(move |j| Point::new(i as f64, j as f64)))
            .collect();
        matches_brute(pts);
    }

    #[test]
    fn test_clustered() {
        let mut pts = vec![
            Point::new(0.0, 0.0), Point::new(100.0, 100.0), Point::new(200.0, 200.0),
        ];
        pts.push(Point::new(0.1, 0.0)); // Very close to (0,0)
        let d = closest_pair(&pts);
        assert!((d - 0.1).abs() < 1e-9);
    }

    #[test]
    fn test_strip_case() {
        // Points near the dividing line
        let pts = vec![
            Point::new(0.0, 0.0), Point::new(1.0, 0.0),
            Point::new(0.5, 0.3), Point::new(0.5, -0.3),
        ];
        matches_brute(pts);
    }
}
