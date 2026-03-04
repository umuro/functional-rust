# 836: Line Segment Intersection Detection

**Difficulty:** 4  **Level:** Advanced

Determine whether two line segments intersect using cross-product orientation tests — robust, branchless, and degenerate-case aware.

## The Problem This Solves

Given two line segments AB and CD in the plane, do they intersect? This elementary computational geometry primitive appears in collision detection (game physics, robotics), map overlay algorithms (GIS), circuit board routing (do two wire segments cross?), and as the inner test in sweep-line algorithms for the general line segment intersection problem.

The naïve approach — compute the intersection point by solving the linear system — fails on parallel segments and requires careful handling of the collinear case. The cross-product orientation approach is cleaner: it works purely with sign tests, handles all degenerate cases naturally, and avoids division entirely (important for exact arithmetic with integer coordinates).

This example implements the complete intersection test including the collinear overlap case, and also computes the intersection point when segments properly cross.

## The Intuition

The orientation of three points P, Q, R is determined by the sign of the cross product (Q-P) × (R-P):
- Positive: P→Q→R is counter-clockwise
- Negative: clockwise
- Zero: collinear

Two segments AB and CD intersect if and only if:
1. A and B have opposite orientations with respect to CD (C and D are on opposite sides of line AB), **and**
2. C and D have opposite orientations with respect to AB

Special case: if any orientation is zero, points are collinear — check if the segment endpoint lies within the other segment's bounding box.

No division, no floating-point edge cases for the orientation test itself. When you *do* need the intersection point, a single parameterised formula works after verifying intersection.

## How It Works in Rust

```rust
#[derive(Clone, Copy, Debug)]
struct Point { x: f64, y: f64 }

// Cross product of vectors (b-a) and (c-a)
// Positive: counter-clockwise; negative: clockwise; zero: collinear
fn cross(a: Point, b: Point, c: Point) -> f64 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

fn on_segment(p: Point, a: Point, b: Point) -> bool {
    // Is p on segment AB, given p is collinear with A and B?
    p.x >= a.x.min(b.x) && p.x <= a.x.max(b.x) &&
    p.y >= a.y.min(b.y) && p.y <= a.y.max(b.y)
}

fn segments_intersect(a: Point, b: Point, c: Point, d: Point) -> bool {
    let d1 = cross(c, d, a); // orientation of A w.r.t. CD
    let d2 = cross(c, d, b); // orientation of B w.r.t. CD
    let d3 = cross(a, b, c); // orientation of C w.r.t. AB
    let d4 = cross(a, b, d); // orientation of D w.r.t. AB

    // Proper intersection: A and B on opposite sides of CD, and vice versa
    if d1 * d2 < 0.0 && d3 * d4 < 0.0 { return true; }

    // Degenerate: collinear cases — check if endpoint lies on other segment
    if d1 == 0.0 && on_segment(a, c, d) { return true; }
    if d2 == 0.0 && on_segment(b, c, d) { return true; }
    if d3 == 0.0 && on_segment(c, a, b) { return true; }
    if d4 == 0.0 && on_segment(d, a, b) { return true; }

    false
}

// Compute intersection point (assumes segments properly intersect)
fn intersection_point(a: Point, b: Point, c: Point, d: Point) -> Option<Point> {
    let denom = (b.x - a.x) * (d.y - c.y) - (b.y - a.y) * (d.x - c.x);
    if denom.abs() < 1e-12 { return None; } // parallel

    let t = ((c.x - a.x) * (d.y - c.y) - (c.y - a.y) * (d.x - c.x)) / denom;
    Some(Point {
        x: a.x + t * (b.x - a.x),
        y: a.y + t * (b.y - a.y),
    })
}
```

`d1 * d2 < 0.0` elegantly tests "opposite signs" without branching on which is positive. For integer coordinates, use `i64` cross products and `d1 * d2 < 0` to avoid any floating-point error entirely.

The four collinear checks handle all degenerate configurations: T-intersections, endpoint touching, and collinear overlap. Skipping them causes subtle bugs in sweep-line algorithms.

## What This Unlocks

- **Sweep-line intersection**: Shamos-Hoey and Bentley-Ottmann algorithms use this test as their O(1) inner operation.
- **Polygon inside test**: the ray-casting algorithm fires a ray from the query point and counts segment intersections.
- **Collision detection**: convex polygon intersection (GJK, SAT) reduces to segment intersection tests on polygon edges.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Point type | Record `{ x: float; y: float }` | `#[derive(Clone, Copy)] struct Point { x: f64, y: f64 }` |
| Cross product | Same formula | Identical — geometry is language-agnostic |
| `Copy` semantics | Records copy by default | Requires explicit `#[derive(Copy)]` — opt-in, not default |
| Float comparison | `<> 0.0` is fine for exact cases | `== 0.0` for collinear check (exact orientation test output) |
| Opposite-sign test | `d1 *. d2 < 0.0` | `d1 * d2 < 0.0` — same, no dot notation for `f64` |
