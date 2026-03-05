//! # Convex Hull (Graham Scan)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point { pub x: f64, pub y: f64 }

fn cross(o: Point, a: Point, b: Point) -> f64 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}

pub fn convex_hull(mut points: Vec<Point>) -> Vec<Point> {
    if points.len() < 3 { return points; }
    points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap().then(a.y.partial_cmp(&b.y).unwrap()));
    let mut lower = vec![];
    for &p in &points {
        while lower.len() >= 2 && cross(lower[lower.len()-2], lower[lower.len()-1], p) <= 0.0 { lower.pop(); }
        lower.push(p);
    }
    let mut upper = vec![];
    for &p in points.iter().rev() {
        while upper.len() >= 2 && cross(upper[upper.len()-2], upper[upper.len()-1], p) <= 0.0 { upper.pop(); }
        upper.push(p);
    }
    lower.pop(); upper.pop();
    lower.extend(upper); lower
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_hull() {
        let pts = vec![Point{x:0.0,y:0.0}, Point{x:1.0,y:0.0}, Point{x:0.5,y:0.5}, Point{x:0.0,y:1.0}, Point{x:1.0,y:1.0}];
        assert_eq!(convex_hull(pts).len(), 4);
    }
}
