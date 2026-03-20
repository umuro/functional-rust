📖 **[View on hightechmind.io →](https://hightechmind.io/rust/834-convex-hull-graham)**

---

# Convex Hull — Graham Scan
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

The convex hull of a point set is the smallest convex polygon containing all points — the shape you'd get by stretching a rubber band around the points. It's the foundation of computational geometry: collision detection in game engines, GIS boundary computation, robot motion planning (collision-free paths), image processing (object outline detection), and clustering algorithms all use convex hull. Graham scan computes the convex hull in O(n log n) dominated by sorting — the actual hull construction is O(n) after the sort. Understanding convex hull also introduces the cross product test for point orientation, a building block for many geometric algorithms.

## Learning Outcomes

- Implement the cross product (orientation test) for three points: positive = left turn, negative = right turn, zero = collinear
- Sort points by polar angle from the lowest-leftmost point as pivot
- Implement the Graham scan loop: maintain a stack, pop while right turns occur, push each new point
- Handle degenerate cases: collinear points, duplicate points, all points collinear
- Recognize O(n log n) dominated by sorting; hull construction itself is linear

## Rust Application

```rust
pub fn cross(o: &Point, a: &Point, b: &Point) -> f64 {
    (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x)
}
pub fn convex_hull(mut points: Vec<Point>) -> Vec<Point> {
    points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap()
        .then(a.y.partial_cmp(&b.y).unwrap()));
    let mut hull: Vec<Point> = Vec::new();
    // Lower hull then upper hull
    for p in points.iter().chain(points.iter().rev()) {
        while hull.len() >= 2 && cross(&hull[hull.len()-2], &hull[hull.len()-1], p) <= 0.0 {
            hull.pop();
        }
        hull.push(*p);
    }
    hull.dedup();
    hull
}
```

Using `f64` for coordinates enables direct comparison but introduces floating-point precision issues. For robust geometry, integer coordinates with exact arithmetic are preferred. The `partial_cmp` and `unwrap` for `f64` sorting is a common Rust pattern — `total_cmp` (stable since 1.62) avoids `unwrap` for NaN-free floats. The lower + upper hull approach (Andrew's monotone chain, a Graham scan variant) is simpler to implement correctly than angle-sorted Graham scan. The `<=` threshold (not `<`) handles collinear points by removing them from the hull.

## OCaml Approach

OCaml represents points as `{ x: float; y: float }` or `(float * float)`. The `cross` function is a pure floating-point computation. Sorting uses `List.sort` with a comparison function. OCaml's `Stack` module or a `list` used as a stack implements the hull construction. `List.rev` reverses for the upper hull pass. OCaml's `compare` function handles float comparison with NaN semantics differing from IEEE. For exact arithmetic, OCaml uses `Zarith` with rational coordinates.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Point type | `struct Point { x: f64, y: f64 }` | Record `{ x: float; y: float }` |
| Float sort | `partial_cmp().unwrap()` or `total_cmp` | `compare` (handles NaN differently) |
| Stack | `Vec<Point>` with `pop`/`push` | `list` used as stack or `Stack.t` |
| Cross product | Returns `f64` | Returns `float` |
| Collinear handling | `<= 0.0` in condition | Same |
| Exact arithmetic | `i64` coordinates or `Ratio` crate | `Zarith.q` rationals |

## Exercises

1. Implement integer coordinate convex hull to avoid all floating-point precision issues.
2. Add a `point_in_convex_hull` function using binary search on the hull angles in O(log n).
3. Compute the perimeter and area of the convex hull using the shoelace formula.
4. Find the diameter of the point set (farthest pair) using rotating calipers on the convex hull in O(n).
5. Handle the degenerate case where all input points are collinear and the hull degenerates to a line segment.
