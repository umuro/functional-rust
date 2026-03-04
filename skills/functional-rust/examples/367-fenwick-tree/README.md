# 367: Fenwick Tree — Efficient Prefix Sums

**Difficulty:** 3  **Level:** Advanced

O(log n) prefix sum queries and point updates in half the memory of a segment tree, powered by a single bit trick.

## The Problem This Solves

You maintain an array of values that updates frequently. You often need the sum of all elements from index 0 to some index i (a "prefix sum"). A plain prefix sum array gives O(1) queries but O(n) updates — every update forces a full rebuild. A segment tree gives O(log n) for both but requires 4n nodes and two recursive functions.

A Fenwick tree (Binary Indexed Tree, BIT) achieves O(log n) for both prefix sum queries and point updates using just n+1 storage and two loops — no recursion, no tree struct, no child pointers. The implementation is strikingly compact: 10-15 lines of code. It has excellent cache performance because everything is a contiguous array.

The limitation: Fenwick trees are specialized for prefix sums (and by extension, range sums via prefix(r) - prefix(l-1)). They don't generalize to range minimum/maximum queries without significant complexity. For those, reach for a segment tree instead.

## The Intuition

The bit trick that makes it work: `i & (-i)` isolates the lowest set bit of i. This determines the "responsibility range" of each array cell — how many elements it stores the sum of.

To update index i: add the delta to tree[i], then jump to `i += i & (-i)` and repeat until you leave the array. You're walking up the chain of nodes that "cover" index i.

To query prefix sum [1..i]: add tree[i], then jump to `i -= i & (-i)` and repeat until i = 0. You're collecting non-overlapping partial sums that together cover [1..i].

Think of it as a compressed partial sum table where each cell covers a power-of-two-sized range determined by its position's trailing zeros.

## How It Works in Rust

```rust
struct Fenwick {
    tree: Vec<i64>, // 1-indexed; tree[0] unused
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick { tree: vec![0; n + 1] }
    }

    // Build from existing data in O(n)
    fn from_slice(data: &[i64]) -> Self {
        let mut fw = Fenwick::new(data.len());
        for (i, &v) in data.iter().enumerate() {
            fw.add(i + 1, v); // Fenwick is 1-indexed
        }
        fw
    }

    // Add delta to index i (1-indexed) — O(log n)
    fn add(&mut self, mut i: usize, delta: i64) {
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += i & i.wrapping_neg(); // i += lowest set bit of i
        }
    }

    // Prefix sum [1..=i] (1-indexed) — O(log n)
    fn prefix_sum(&self, mut i: usize) -> i64 {
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i -= i & i.wrapping_neg(); // i -= lowest set bit of i
        }
        sum
    }

    // Range sum [l..=r] (1-indexed) — O(log n)
    fn range_sum(&self, l: usize, r: usize) -> i64 {
        self.prefix_sum(r) - if l > 1 { self.prefix_sum(l - 1) } else { 0 }
    }

    // Point value at index i — O(log n) via range_sum
    fn get(&self, i: usize) -> i64 {
        self.range_sum(i, i)
    }
}

// Usage
let data = vec![3i64, 2, -1, 6, 5, 4, -3, 3, 7, 2];
let mut fw = Fenwick::from_slice(&data);

println!("{}", fw.prefix_sum(5)); // sum of first 5: 3+2-1+6+5 = 15
println!("{}", fw.range_sum(3, 7)); // sum of indices 3..=7

fw.add(4, 1); // add 1 to index 4 (now 6+1=7)
println!("{}", fw.prefix_sum(5)); // 16 after update
```

## What This Unlocks

- **Online range sum queries**: leaderboard score aggregation, time-bucket histograms, cumulative distribution functions — with updates interleaved between queries.
- **Order statistics / rank queries**: map values to frequency counts in a Fenwick tree; prefix sum up to value X tells you how many elements are ≤ X (counts sort / inversion count).
- **2D extensions**: a 2D Fenwick tree (Fenwick of Fenwick arrays) supports rectangle sum queries in O(log² n) — used in 2D competitive programming problems.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Prefix sum (static) | O(n) precompute + O(1) query | same, use prefix sum array |
| Prefix sum (dynamic) | O(n) scan per query | O(log n) Fenwick |
| Point update | O(1) | O(log n) propagation |
| Space | O(n) | O(n) — half of segment tree |
| Code complexity | moderate | very compact (2 loops) |
| Generalizes to range min/max? | with effort | no — use segment tree instead |
