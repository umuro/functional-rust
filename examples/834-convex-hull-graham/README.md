📖 **[View on hightechmind.io →](https://hightechmind.io/rust/834-convex-hull-graham)**

---

# 834: Convex Hull — Graham Scan

**Difficulty:** 4  **Level:** Advanced

Find the smallest convex polygon enclosing a set of 2D points in O(n log n) — the gateway algorithm to computational geometry.

## The Problem This Solves

The convex hull is the "rubber band stretched around all points" — the minimal convex polygon containing the entire point set. It's needed in collision detection (the hull of a mesh), geographic analysis (the boundary region of a GPS track), robotics (workspace reachability), and computer vision (object shape descriptors). It's also the preprocessing step for many harder geometry problems: rotating calipers for diameter, Minkowski sum, half-plane intersection.

Graham scan is O(n log n) — dominated by the sorting step. Once sorted, the sweep is O(n): each point is pushed and popped at most once. This is optimal: any algorithm must sort the points (or implicitly sort them), which requires Ω(n log n) comparisons. Jarvis march is simpler to code but O(n × h) where h is hull size — worse when the hull is large.

The cross product is the fundamental geometric primitive underlying every 2D algorithm: three points, one determinant, three cases: left turn (keep), right turn (pop), collinear (depends on goal). Learning to think in cross products unlocks the rest of 2D computational geometry.

## The Intuition

Sort all points by polar angle from the bottom-left pivot. Then sweep: maintain a stack where every consecutive triple makes a left turn (counterclockwise). When a new point makes a right turn with the top two stack elements, pop — that middle point is interior to the hull. The CCW invariant is maintained throughout.

Cross product of (B-A) × (C-A): positive = C is left of line AB (left turn), negative = right turn, zero = collinear. This single determinant is all you need for any turn test in 2D.

## How It Works in Rust

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
struct Point { x: f64, y: f64 }

// Cross product of vectors (b-a) and (c-a)
// > 0: left turn (CCW), < 0: right turn (CW), = 0: collinear
fn cross(a: &Point, b: &Point, c: &Point) -> f64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn convex_hull(mut points: Vec<Point>) -> Vec<Point> {
    let n = points.len();
    if n <= 1 { return points; }

    // Step 1: Find bottom-left pivot (lowest y, then leftmost x)
    let pivot_idx = points.iter().enumerate()
        .min_by(|(_, a), (_, b)| a.y.partial_cmp(&b.y).unwrap()
            .then(a.x.partial_cmp(&b.x).unwrap()))
        .map(|(i, _)| i).unwrap();
    points.swap(0, pivot_idx);
    let pivot = points[0];

    // Step 2: Sort by polar angle from pivot — O(n log n)
    // Collinear points: keep closer one first
    points[1..].sort_by(|a, b| {
        let c = cross(&pivot, a, b);
        if c > 0.0 { std::cmp::Ordering::Less }     // a comes before b: left turn
        else if c < 0.0 { std::cmp::Ordering::Greater }
        else { pivot.dist2(a).partial_cmp(&pivot.dist2(b)).unwrap() }
    });

    // Step 3: Stack sweep — O(n)
    let mut stack: Vec<Point> = Vec::with_capacity(n);
    for &p in &points {
        // Pop points that make a right turn (non-left turn)
        while stack.len() >= 2
            && cross(&stack[stack.len()-2], &stack[stack.len()-1], &p) <= 0.0
        {
            stack.pop();
        }
        stack.push(p);
    }
    stack
}
```

`partial_cmp` on floats returns `Option<Ordering>` — use `.unwrap()` when you know inputs are non-NaN, or `.unwrap_or(Ordering::Equal)` for defensive code. For integer points, use integer cross products to avoid floating-point precision issues entirely.

## What This Unlocks

- **Collision detection**: Hull of a mesh provides the cheapest broad-phase test; two objects can't collide if their hulls don't overlap (test with GJK or SAT).
- **Rotating calipers**: After computing the hull, rotating calipers finds the diameter (farthest pair), minimum bounding rectangle, and Minkowski sum in O(n) — building directly on Graham scan output.
- **Competitive geometry**: Most computational geometry contest problems start with "given n points…" and the first step is usually the convex hull.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Point type | `type point = { x: float; y: float }` | `struct Point { x: f64, y: f64 }` |
| Sort by angle | `List.sort` with comparison function | `sort_by` with closure — same semantics |
| Stack | Recursive list as stack | `Vec<Point>` with `push`/`pop` |
| Cross product | Pure function `cross a b c` | Function or method on `Point` |
| Float comparison | Raises exception on NaN | `partial_cmp` returns `Option<Ordering>` |
