📖 **[View on hightechmind.io →](https://hightechmind.io/rust/836-line-segment-intersection)**

---

# Line Segment Intersection
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Testing whether two line segments intersect is a building block of computational geometry: polygon union/intersection, road network analysis, PCB trace routing, robot obstacle detection, and sweep line algorithms all reduce to repeated segment intersection tests. The cross product orientation test determines which side of a directed line a point lies on; two segments intersect if and only if each segment's endpoints straddle the other segment's line. Degenerate cases (collinear segments, touching at endpoints) require careful handling. This is one of the most error-prone geometric primitives due to floating-point precision issues.

## Learning Outcomes

- Implement the orientation test using cross products: left turn, right turn, or collinear
- Test segment intersection: two segments AB and CD intersect iff orientations disagree properly
- Handle the collinear overlap case: segments lie on the same line and may share a range
- Understand the "straddle" test: A and B on opposite sides of line CD, AND C and D on opposite sides of line AB
- Compute the actual intersection point using parametric line equations when intersection is confirmed

## Rust Application

```rust
pub fn orientation(p: Point, q: Point, r: Point) -> i32 {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    if val == 0.0 { 0 } else if val > 0.0 { 1 } else { 2 }
}
pub fn segments_intersect(p1: Point, q1: Point, p2: Point, q2: Point) -> bool {
    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);
    if o1 != o2 && o3 != o4 { return true; }
    // Collinear cases: check if point lies on segment
    // ...
    false
}
```

Integer coordinates eliminate floating-point precision issues entirely. With `f64` coordinates, near-collinear cases require an epsilon comparison. Rust's `f64` comparison `val == 0.0` is exact only for integer-valued floats; robust code uses `val.abs() < EPSILON`. The four orientation tests cover all non-degenerate cases; the collinear overlap case requires additional `on_segment` checks. The orientation value encodes the sign of the cross product — the key invariant for computational geometry.

## OCaml Approach

OCaml uses the same cross-product orientation test with `float` arithmetic. The `compare` function on floats respects IEEE 754. OCaml's polymorphic comparison works on `(float * float)` point tuples directly. The `on_segment` collinear check uses `min/max` range tests. OCaml's pipe operator `|>` chains the four orientation computations readably. The `Gg` library provides production-quality 2D geometry primitives.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Point representation | `struct Point` or `(f64, f64)` | Record or tuple |
| Orientation return | `i32` (0,1,2 or -1,0,1) | `int` |
| Epsilon handling | `val.abs() < f64::EPSILON` | Same |
| Collinear check | Separate `on_segment` function | Same |
| Integer coords | `i64` for exact arithmetic | `int` for exact |
| Production library | `geo` crate | `Gg` library |

## Exercises

1. Compute the exact intersection point (as `Option<Point>`) when two segments do intersect.
2. Implement the sweep line algorithm using segment intersection tests to find all intersections among n segments in O((n + k) log n).
3. Handle collinear overlapping segments by returning the overlap interval as `Option<(Point, Point)>`.
4. Implement with integer coordinates and compare robustness vs. floating-point on near-collinear cases.
5. Use your segment intersection test to implement a simple polygon clip against a rectangular window (Cohen-Sutherland).
