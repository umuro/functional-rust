📖 **[View on hightechmind.io →](https://hightechmind.io/rust/838-interval-tree-stabbing)**

---

# Interval Tree — Stabbing Queries

## Problem Statement

Given a set of intervals [l, r] and a query point q, find all intervals that contain q (stabbing query). Naive linear scan is O(n) per query. An interval tree answers stabbing queries in O(log n + k) where k is the number of reported intervals, after O(n log n) preprocessing. This enables efficient scheduling conflict detection (does event at time q overlap any existing booking?), genomic range queries (which genes cover position q?), and network packet classification (which firewall rules match this port range?). Interval trees are the data structure behind many calendar and scheduling applications.

## Learning Outcomes

- Build an interval tree using a centered decomposition: for each node, store intervals crossing the center
- Organize crossing intervals in two sorted lists: by left endpoint and by right endpoint
- Implement stabbing query: follow left or right child based on q vs. center, query crossing intervals
- Understand the O(log n + k) query complexity and O(n log n) build complexity
- Compare with segment tree and augmented BST approaches to range queries

## Rust Application

```rust
pub struct IntervalTree {
    center: i64,
    left_sorted: Vec<(i64, i64)>,   // intervals containing center, sorted by left
    right_sorted: Vec<(i64, i64)>,  // same intervals, sorted by right (reversed)
    left_child: Option<Box<IntervalTree>>,
    right_child: Option<Box<IntervalTree>>,
}
impl IntervalTree {
    pub fn stab(&self, q: i64) -> Vec<(i64, i64)> {
        // Report from crossing intervals, then recurse to appropriate child
    }
}
```

Rust's `Box<IntervalTree>` enables the recursive tree structure. The `Option<Box<IntervalTree>>` for children handles leaf nodes without null pointers. The `Vec<(i64, i64)>` for sorted intervals keeps tuples compact and cache-friendly. The stabbing query walks down the tree following whether q < center or q > center, accumulating intervals from each node that contains q. Using integers avoids floating-point precision issues common in interval queries.

## OCaml Approach

OCaml represents the interval tree with an algebraic type: `type t = Leaf | Node of { center: int; left_sorted: interval list; right_sorted: interval list; left_child: t; right_child: t }`. The `stab q tree` function pattern-matches on the tree structure. `List.filter` or binary search on sorted lists reports crossing intervals. OCaml's immutable default makes building the tree via `let rec build` natural. The `List.sort` and `List.rev` operations create the two sorted interval lists.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Tree structure | `Option<Box<IntervalTree>>` | Algebraic type with `Leaf` |
| Sorted intervals | `Vec<(i64, i64)>` | `(int * int) list` |
| Null handling | `Option` | Pattern match on `Leaf` |
| Build complexity | O(n log n) | Same |
| Query complexity | O(log n + k) | Same |
| Memory | Compact Vec tuples | GC-managed list nodes |

## Exercises

1. Implement `all_overlaps(l, r)`: find all intervals that overlap the query interval [l, r].
2. Add deletion support: mark intervals as deleted with a tombstone and rebuild when tombstone ratio exceeds 50%.
3. Compare interval tree vs. sorted array + binary search for stabbing queries at varying n.
4. Implement a segment tree-based approach for stabbing queries and compare with interval tree.
5. Handle open/closed interval endpoints: `(l, r)`, `[l, r)`, `(l, r]`, `[l, r]` all handled correctly.
