📖 **[View on hightechmind.io →](https://hightechmind.io/rust/837-closest-pair-divide-conquer)**

---

# Closest Pair of Points — Divide and Conquer

## Problem Statement

Finding the two closest points in a set of n points naively requires O(n^2) distance comparisons. Divide-and-conquer solves this in O(n log n): split points by x-coordinate median, recursively find closest pairs in left and right halves, then check the strip of width 2*delta around the dividing line for cross-half pairs. The key insight: within the strip, each point needs to be compared against at most 7 others — a fixed constant, making the strip check O(n) total. This algorithm appears in geographic information systems (nearest neighbor queries), robotics (obstacle proximity), and computational chemistry (molecular interaction detection).

## Learning Outcomes

- Split points at median x, recurse on halves, combine with strip check
- Understand the strip argument: at most 7 points fit in a delta×2*delta rectangle
- Implement the strip check: sort by y-coordinate, slide a window of 8 points
- Recognize the O(n log n) recurrence: T(n) = 2T(n/2) + O(n log n) for strip sort
- Optimize to true O(n log n) by maintaining y-sorted arrays through recursion

## Rust Application

```rust
pub fn closest_pair(points: &mut [Point]) -> f64 {
    if points.len() <= 3 {
        return brute_force_closest(points);
    }
    points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    let mid = points.len() / 2;
    let mid_x = points[mid].x;
    let d = closest_pair(&mut points[..mid])
        .min(closest_pair(&mut points[mid..]));
    let strip: Vec<&Point> = points.iter()
        .filter(|p| (p.x - mid_x).abs() < d)
        .collect();
    closest_in_strip(&strip, d)
}
```

The in-place sort with `sort_by` modifies the slice. Rust's split at index `points[..mid]` and `points[mid..]` gives two disjoint mutable slices. The strip filter collects references into a `Vec<&Point>` for the strip check. Rust's borrowing rules enforce that the strip references don't outlive the points slice. The `f64::min` combines the two recursive results cleanly. Base case `<= 3` ensures brute force handles trivially small inputs without infinite recursion.

## OCaml Approach

OCaml sorts with `Array.sort` and splits using `Array.sub`. The recursive calls work on sub-arrays. The strip check uses `Array.to_list |> List.filter` then `List.sort` by y-coordinate, followed by a sliding window. OCaml's `List.fold_left` computes the minimum distance in the strip. The functional style returns a new sorted-by-y array at each level (merge-sort style) for the true O(n log n) variant. `Float.min` combines the two recursive distances.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Array mutation | In-place sort on `&mut [Point]` | `Array.sort` (in-place) |
| Splitting | Slice indexing `&mut points[..mid]` | `Array.sub` (copies) |
| Strip | `Vec<&Point>` — references | `Point list` or `Point array` |
| Base case size | `<= 3` | Same |
| True O(n log n) | Sort by x once, maintain y-order | `List.merge` approach |
| Parallel variant | `rayon::join` for halves | No built-in parallel |

## Exercises

1. Implement the true O(n log n) variant that avoids re-sorting the strip by maintaining y-sorted arrays.
2. Extend to 3D: find the closest pair of points in 3D space using the same divide-and-conquer approach.
3. Use Rayon to parallelize the two recursive halves and measure speedup on 10^6 points.
4. Implement a KD-tree and compare its closest-pair query time against the divide-and-conquer algorithm.
5. Verify correctness by comparing with brute-force O(n^2) on random point sets of size up to 1000.
