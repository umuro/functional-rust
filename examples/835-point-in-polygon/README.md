📖 **[View on hightechmind.io →](https://hightechmind.io/rust/835-point-in-polygon)**

---

# Point-in-Polygon Test

## Problem Statement

Determining whether a point lies inside a polygon is fundamental to GIS applications, game collision detection, map interfaces (is this GPS coordinate inside this region?), and computational geometry. The ray casting algorithm casts a horizontal ray from the query point rightward and counts crossings with polygon edges — an odd count means inside, even means outside. The winding number algorithm counts how many times the polygon winds around the point, handling non-convex and self-intersecting polygons correctly. These tests appear in every GIS library, game physics engine, and graphics renderer.

## Learning Outcomes

- Implement the ray casting algorithm: count horizontal ray crossings with polygon edges
- Handle edge cases: point exactly on edge, ray passing through vertex, horizontal edges
- Implement the winding number algorithm for non-convex and self-intersecting polygons
- Understand when each algorithm is appropriate: ray casting for simple polygons, winding number for complex
- Apply to batch queries: preprocess polygon for O(log n) per query vs O(n) per query

## Rust Application

```rust
pub fn point_in_polygon(point: &Point, polygon: &[Point]) -> bool {
    let n = polygon.len();
    let mut inside = false;
    let (px, py) = (point.x, point.y);
    let (mut j) = n - 1;
    for i in 0..n {
        let (xi, yi) = (polygon[i].x, polygon[i].y);
        let (xj, yj) = (polygon[j].x, polygon[j].y);
        if ((yi > py) != (yj > py)) &&
           (px < (xj - xi) * (py - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }
    inside
}
```

The Jordan curve theorem guarantees the ray casting result for simple (non-self-intersecting) polygons. The condition `(yi > py) != (yj > py)` checks whether the edge crosses the horizontal ray height. The division avoids edge cases where the edge is horizontal (denominator non-zero due to the first condition). Rust's tuple destructuring `(xi, yi)` keeps the coordinate extraction clean. The algorithm handles the previous vertex via `j = i` update — no circular indexing with modulo.

## OCaml Approach

OCaml implements point-in-polygon with a `List.fold_left` over edge pairs or a `for` loop with `Array`. The condition is identical floating-point arithmetic. OCaml's `ref` for the mutable `inside` flag: `let inside = ref false in ... inside := not !inside`. The functional version threads the crossing count and previous vertex through `fold`. OCaml's `List.filteri` can pre-filter edges that could possibly cross the query point's y-coordinate for batch optimization.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Mutation | `inside = !inside` (bool flip) | `inside := not !inside` |
| Edge iteration | `for i in 0..n` with `j = i` | `for` loop or `fold_left` |
| Previous vertex | Separate `j` variable | Same or pair in fold state |
| Float comparison | `!=` for `f64` (works here) | `<>` for float |
| Winding number | Separate function | Same approach |
| Batch queries | `fn contains_batch` | `List.map (point_in_polygon poly)` |

## Exercises

1. Implement the winding number algorithm and verify it handles self-intersecting polygons correctly where ray casting fails.
2. Add a `point_on_edge` test and handle the "on boundary" case by returning an enum `Inside`/`Outside`/`OnBoundary`.
3. Preprocess a convex polygon for O(log n) point-in-polygon using binary search on edge angles.
4. Batch test 10,000 random points against a complex polygon and measure the performance of ray casting vs. winding number.
5. Implement polygon union/intersection using multiple point-in-polygon tests (Sutherland-Hodgman algorithm).
