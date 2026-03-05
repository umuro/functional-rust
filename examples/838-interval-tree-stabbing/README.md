📖 **[View on hightechmind.io →](https://hightechmind.io/rust/838-interval-tree-stabbing)**

---

# 838: Interval Tree for Stabbing Queries

**Difficulty:** 4  **Level:** Advanced

Answer "which intervals contain point x?" in O(log n + k) per query after O(n log n) preprocessing — the centroid-decomposition approach.

## The Problem This Solves

Given a set of n intervals [lo, hi] and a query point x, find all intervals that contain x (i.e., lo ≤ x ≤ hi). This is the "stabbing query" — which intervals are "stabbed" by the vertical line x = q?

Stabbing queries appear in genomics (find all genes overlapping position p on a chromosome), database range queries (find all events active at time t), rendering pipelines (find all bounding boxes containing a pixel), and scheduling systems (find all tasks running at a given moment). A sorted list approach costs O(n) per query; an interval tree answers in O(log n + k) where k is the number of reported intervals.

This example uses the median-based interval tree: each node stores its median point, and intervals that span the median in two sorted orders (by lo and by hi). Intervals that don't span the node's median are recursed into the left or right subtree.

## The Intuition

At each tree node, pick the median point m of all interval endpoints. Split intervals into three groups:
- **Spans median**: lo ≤ m ≤ hi — store at this node
- **Entirely left of m**: recurse into left subtree
- **Entirely right of m**: recurse into right subtree

At each node, store spanning intervals twice:
- Sorted by lo (for queries where x ≥ m: report all with lo ≤ x)
- Sorted by hi (for queries where x ≤ m: report all with hi ≥ x)

Query: at each node, if x < m check the hi-sorted list (stop when hi < x), if x > m check the lo-sorted list (stop when lo > x), if x = m report all. Then recurse into the appropriate subtree.

O(n log n) construction. O(log n + k) query — the tree depth is O(log n) and each node visit reports some intervals or terminates early.

## How It Works in Rust

```rust
#[derive(Debug, Clone)]
struct Interval { lo: i64, hi: i64, id: usize }

enum IntervalTree {
    Leaf,
    Node {
        median: i64,
        by_lo: Vec<Interval>, // spanning intervals sorted by lo (ascending)
        by_hi: Vec<Interval>, // spanning intervals sorted by hi (descending)
        left:  Box<IntervalTree>,
        right: Box<IntervalTree>,
    }
}

impl IntervalTree {
    fn build(mut intervals: Vec<Interval>) -> Self {
        if intervals.is_empty() { return IntervalTree::Leaf; }

        // Collect all endpoints, find median
        let mut endpoints: Vec<i64> = intervals.iter()
            .flat_map(|iv| [iv.lo, iv.hi])
            .collect();
        endpoints.sort_unstable();
        let median = endpoints[endpoints.len() / 2];

        // Partition intervals
        let mut left_ivs  = vec![];
        let mut right_ivs = vec![];
        let mut spanning  = vec![];

        for iv in intervals.drain(..) {
            if iv.hi < median      { left_ivs.push(iv); }
            else if iv.lo > median { right_ivs.push(iv); }
            else                   { spanning.push(iv); }
        }

        let mut by_lo = spanning.clone();
        let mut by_hi = spanning;
        by_lo.sort_unstable_by_key(|iv| iv.lo);
        by_hi.sort_unstable_by_key(|iv| std::cmp::Reverse(iv.hi));

        IntervalTree::Node {
            median,
            by_lo,
            by_hi,
            left:  Box::new(IntervalTree::build(left_ivs)),
            right: Box::new(IntervalTree::build(right_ivs)),
        }
    }

    fn stab(&self, x: i64, result: &mut Vec<usize>) {
        match self {
            IntervalTree::Leaf => {}
            IntervalTree::Node { median, by_lo, by_hi, left, right } => {
                if x < *median {
                    // Check intervals sorted descending by hi: stop when hi < x
                    for iv in by_hi {
                        if iv.hi < x { break; }
                        if iv.lo <= x { result.push(iv.id); }
                    }
                    left.stab(x, result);
                } else if x > *median {
                    // Check intervals sorted ascending by lo: stop when lo > x
                    for iv in by_lo {
                        if iv.lo > x { break; }
                        if iv.hi >= x { result.push(iv.id); }
                    }
                    right.stab(x, result);
                } else {
                    // x == median: all spanning intervals match
                    for iv in by_lo { result.push(iv.id); }
                    left.stab(x, result);
                    right.stab(x, result);
                }
            }
        }
    }
}
```

`std::cmp::Reverse` wraps a value to reverse its sort order — `sort_unstable_by_key(|iv| Reverse(iv.hi))` sorts descending by hi without writing a custom comparator.

The `drain(..)` idiom moves all elements out of `intervals`, consuming the vector efficiently. This avoids cloning during the partition step.

## What This Unlocks

- **Genomic interval queries**: find all annotated features (genes, exons, repeats) overlapping a given chromosomal position — standard in bioinformatics pipelines.
- **Event scheduling**: query all active tasks/reservations at a given time — used in calendar systems, resource schedulers, and simulation engines.
- **Rendering**: bounding-volume hierarchy (BVH) trees extend interval trees to 2D/3D for fast ray-object intersection in ray tracing.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive ADT | `type tree = Leaf \| Node of ...` | `enum IntervalTree { Leaf, Node { ... } }` |
| Boxed subtree | `tree` (GC-managed heap) | `Box<IntervalTree>` — explicit heap allocation |
| Reverse sort | `List.sort (fun a b -> compare b.hi a.hi)` | `sort_unstable_by_key(\|iv\| Reverse(iv.hi))` |
| Drain / consume | `List.iter` (no ownership) | `Vec::drain(..)` — moves elements, leaves vec empty |
| Result accumulation | Returns list via recursion | Mutable `&mut Vec<usize>` — avoids allocation on each call |
