# 835: Point-in-Polygon — Ray Casting

**Difficulty:** 4  **Level:** Advanced

Determine if a point lies inside an arbitrary polygon in O(n) — the fundamental primitive for geographic analysis, game collision, and mesh intersection.

## The Problem This Solves

"Is this GPS coordinate inside this country's border?" "Did this mouse click land inside this irregular shape?" "Is this mesh vertex inside this other mesh?" All three reduce to point-in-polygon. It's one of the most frequently needed geometric primitives, appearing in GIS systems (PostGIS uses ray casting for `ST_Contains`), game engines (click detection on irregular sprites), and computational geometry pipelines.

The ray casting algorithm is O(n) per query where n is the number of polygon edges — optimal without preprocessing. The winding number algorithm is an alternative that handles edge cases differently (particularly for self-intersecting polygons); ray casting is simpler and faster for simple polygons.

Correctness hinges entirely on handling degenerate cases: the ray passing exactly through a vertex, the ray running along an edge, the point exactly on the boundary. Getting these right requires careful inequality choices (using `<` vs `≤` at exactly the right places).

## The Intuition

Shoot a ray from P in the +x direction (i.e., to the right). Count how many times the ray crosses a polygon edge. Odd crossings: P is inside. Even crossings: outside. This is the Jordan curve theorem applied computationally.

For each edge (A, B): the ray crosses the edge if A.y and B.y straddle P.y (one strictly above, one at or below — the careful inequality prevents double-counting vertices), and the crossing x-coordinate is to the right of P. The crossing x: `A.x + (P.y - A.y) / (B.y - A.y) × (B.x - A.x)`.

Boundary detection (point exactly on an edge) is handled separately using the cross product to test collinearity and a bounding box check for the range.

## How It Works in Rust

```rust
const EPS: f64 = 1e-10;

// Test if P lies on segment AB: collinear + within bounding box
fn on_segment(p: &Point, a: &Point, b: &Point) -> bool {
    let cross = (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x);
    if cross.abs() > EPS { return false; }  // Not collinear
    p.x >= f64::min(a.x, b.x) - EPS && p.x <= f64::max(a.x, b.x) + EPS
        && p.y >= f64::min(a.y, b.y) - EPS && p.y <= f64::max(a.y, b.y) + EPS
}

fn point_in_polygon(p: &Point, polygon: &[Point]) -> Location {
    let n = polygon.len();
    if n == 0 { return Location::Outside; }

    let mut crossings = 0usize;
    for i in 0..n {
        let (a, b) = (&polygon[i], &polygon[(i + 1) % n]);

        if on_segment(p, a, b) { return Location::OnBoundary; }

        // Ray crossing condition:
        // 1. Edge straddles P.y: one vertex strictly above, one at-or-below
        //    (the strict/non-strict choice ensures a vertex is counted once)
        // 2. The crossing x-coordinate is to the RIGHT of P
        if (a.y <= p.y && b.y > p.y) || (b.y <= p.y && a.y > p.y) {
            let x_cross = a.x + (p.y - a.y) / (b.y - a.y) * (b.x - a.x);
            if p.x < x_cross {
                crossings += 1;
            }
        }
    }
    if crossings % 2 == 1 { Location::Inside } else { Location::Outside }
}
```

The `(i + 1) % n` wrap connects the last vertex to the first, closing the polygon without special-casing. The inequality choice `a.y <= p.y && b.y > p.y` (one `<=`, one `>`) is the standard way to count each vertex exactly once when the ray passes through it.

## What This Unlocks

- **GIS and geographic queries**: PostGIS `ST_Contains`, OpenLayers polygon selection, and every "is this coordinate in this region?" query uses ray casting or winding number.
- **Game engines and UI**: Click detection on irregular shapes — buttons, sprites, terrain regions. Efficient with spatial indexing (quadtree) to narrow candidates before calling point_in_polygon.
- **Mesh processing**: Voxelization, CSG (Constructive Solid Geometry), and mesh boolean operations all need point-in-polygon as a subroutine for classifying geometry relative to surfaces.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Float comparison | `<`, `<=` (built-in, raises on NaN) | Same; use `EPS` for near-equality |
| Polygon edges | `Array.init n (fun i -> (poly.(i), poly.((i+1) mod n)))` | `for i in 0..n` with `(i+1) % n` |
| Mutable counter | `let count = ref 0 in count := !count + 1` | `let mut crossings = 0; crossings += 1` |
| Result type | Variant or int | `enum Location { Inside, Outside, OnBoundary }` |
| Boundary epsilon | Same EPS pattern | `const EPS: f64 = 1e-10` |
