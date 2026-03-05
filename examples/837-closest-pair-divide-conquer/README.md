# 837: Closest Pair of Points — Divide and Conquer

**Difficulty:** 5  **Level:** Master

Find the two closest points in a set of n points in O(n log n) — the classical divide-and-conquer geometry algorithm.

## The Problem This Solves

Given n points in the plane, find the pair with the smallest Euclidean distance. The brute-force approach checks all O(n²) pairs. The divide-and-conquer algorithm achieves O(n log n) — the same asymptotic bound as sorting — through a non-trivial strip-merging argument.

Closest pair appears in clustering algorithms (initial step for single-linkage clustering), collision detection (finding nearby objects), computational biology (protein structure analysis), and as the canonical example of a geometric divide-and-conquer algorithm. Understanding it is essential for anyone working in computational geometry or teaching algorithm design.

The algorithm returns the minimum distance and the pair of points achieving it. It handles degenerate cases (coincident points, collinear configurations) correctly.

## The Intuition

1. Sort points by x-coordinate.
2. Split into left and right halves at the median x.
3. Recurse on each half; let δ = min(left_min, right_min).
4. The closest pair might cross the split line — but only if both points lie within δ of the split. This "strip" has width 2δ.
5. In the strip, sort by y and check each point against at most 7 others (the geometric argument: in a δ×2δ rectangle, you can pack at most 8 points with pairwise distance ≥ δ, so each point has at most 7 candidates above it within δ vertical distance).

The strip-of-7 argument is the key insight that makes the algorithm O(n log n) rather than O(n²). Without it, checking all pairs in the strip would be quadratic.

O(n log n) total: O(n log n) for the initial sort + O(n log n) from the recurrence T(n) = 2T(n/2) + O(n).

## How It Works in Rust

```rust
fn dist_sq(a: (f64, f64), b: (f64, f64)) -> f64 {
    (a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)
}

fn closest_pair(pts: &mut [(f64, f64)]) -> f64 {
    pts.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap()); // sort by x
    closest_rec(pts).sqrt()
}

fn closest_rec(pts: &[(f64, f64)]) -> f64 {
    let n = pts.len();
    if n <= 3 {
        // Base case: brute force among ≤3 points
        let mut min_d = f64::INFINITY;
        for i in 0..n {
            for j in i+1..n { min_d = min_d.min(dist_sq(pts[i], pts[j])); }
        }
        return min_d;
    }

    let mid = n / 2;
    let mid_x = pts[mid].0;

    // Recurse on left and right halves
    let d_left  = closest_rec(&pts[..mid]);
    let d_right = closest_rec(&pts[mid..]);
    let mut d = d_left.min(d_right); // δ² — compare squared to avoid sqrt

    // Collect strip: points within δ of the split line
    let delta = d.sqrt();
    let mut strip: Vec<(f64, f64)> = pts.iter()
        .filter(|&&p| (p.0 - mid_x).abs() < delta)
        .copied()
        .collect();

    // Sort strip by y for the 7-point check
    strip.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // Check each point against at most 7 above it within δ vertical distance
    for i in 0..strip.len() {
        let mut j = i + 1;
        while j < strip.len() && (strip[j].1 - strip[i].1).powi(2) < d {
            d = d.min(dist_sq(strip[i], strip[j]));
            j += 1;
        }
    }
    d
}
```

Working with squared distances (`dist_sq`) avoids `sqrt` in the inner loop — `sqrt` is expensive and the ordering is preserved under squaring. Only take the final `sqrt` at the top level.

`partial_cmp` is required for `f64` because `f64::NAN != f64::NAN` — Rust's `Ord` requires total order, which floats don't satisfy. If your points are guaranteed finite, `partial_cmp(...).unwrap()` is safe.

The strip filter `(p.0 - mid_x).abs() < delta` uses the pre-computed `delta = d.sqrt()` from the current best distance — as d improves, fewer points enter the strip.

## What This Unlocks

- **Clustering initialisation**: closest pair gives the first merge step in single-linkage hierarchical clustering.
- **Computational geometry curriculum**: this algorithm appears in every algorithms textbook as the paradigmatic divide-and-conquer geometric problem.
- **Robotics path planning**: finding nearby obstacles or waypoints efficiently requires closest-pair queries, often in a k-d tree variant of this approach.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable sort | `Array.sort` in place | `pts.sort_by(...)` — in-place, `partial_cmp` for f64 |
| Float ordering | `compare` works but is slow for float | `partial_cmp(...).unwrap()` — explicit, panics on NaN |
| Strip filter | `List.filter` creates a new list | `.filter().copied().collect()` — same cost, owns data |
| Squared distance | Same trick | `dist_sq` avoids sqrt in inner loop |
| Recursion on slices | Subarray with index bounds | `&pts[..mid]`, `&pts[mid..]` — zero-copy slice borrows |
