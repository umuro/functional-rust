# 366: Segment Tree — O(log n) Range Queries

**Difficulty:** 4  **Level:** Expert

Answer "what is the sum/min/max over index range [l, r]?" in O(log n) and update any single element in O(log n).

## The Problem This Solves

You have an array of values and need to answer many range queries: "what is the sum of elements 3 through 17?", "what is the minimum value between index 100 and 500?". A naive scan answers each query in O(n). A prefix sum array answers range sums in O(1) — but breaks if any element is updated, requiring O(n) to rebuild.

A segment tree gives you O(log n) for both queries and updates. It stores aggregate values (sum, min, max, or any associative operation) for every contiguous subrange in a complete binary tree. The leaf nodes are the array elements; each internal node holds the aggregate of its subtree. When you update one element, only the O(log n) ancestors need updating. When you query a range, at most O(log n) nodes cover the range exactly.

The key advantage over a Fenwick tree: the segment tree supports any associative operation — not just prefix sums. Range minimum query (RMQ), range GCD, range bitwise AND are all trivial to implement in a segment tree but awkward or impossible in a Fenwick tree.

## The Intuition

Think of it as a divide-and-conquer precomputation. The root node holds the aggregate for the entire array. Its left child holds the aggregate for the left half. Its right child for the right half. Recurse until leaves.

To query range [l, r]: at each node, either the node's range is fully inside [l, r] (return the stored aggregate), fully outside (return identity element), or partially overlapping (recurse to both children and combine). At most 4 log n nodes are touched.

There's no direct Python equivalent in the standard library. The segment tree is primarily a competitive programming / systems programming construct.

## How It Works in Rust

```rust
struct SegTree {
    n: usize,
    tree: Vec<i64>, // tree[1] is root; tree[2i] and tree[2i+1] are children
}

impl SegTree {
    fn new(data: &[i64]) -> Self {
        let n = data.len();
        let mut tree = vec![0i64; 4 * n]; // 4n is safe upper bound
        Self::build(&mut tree, data, 1, 0, n - 1);
        SegTree { n, tree }
    }

    fn build(tree: &mut Vec<i64>, data: &[i64], node: usize, l: usize, r: usize) {
        if l == r {
            tree[node] = data[l];
            return;
        }
        let mid = (l + r) / 2;
        Self::build(tree, data, 2 * node,     l,       mid);
        Self::build(tree, data, 2 * node + 1, mid + 1, r);
        tree[node] = tree[2 * node] + tree[2 * node + 1]; // sum; swap for min/max
    }

    // Point update: set index i to value v — O(log n)
    fn update(&mut self, i: usize, v: i64) {
        self.update_inner(1, 0, self.n - 1, i, v);
    }

    fn update_inner(&mut self, node: usize, l: usize, r: usize, i: usize, v: i64) {
        if l == r {
            self.tree[node] = v;
            return;
        }
        let mid = (l + r) / 2;
        if i <= mid { self.update_inner(2 * node,     l,       mid, i, v); }
        else        { self.update_inner(2 * node + 1, mid + 1, r,   i, v); }
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    // Range sum query [ql, qr] — O(log n)
    fn query(&self, ql: usize, qr: usize) -> i64 {
        self.query_inner(1, 0, self.n - 1, ql, qr)
    }

    fn query_inner(&self, node: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if qr < l || r < ql { return 0; }    // outside range: return identity (0 for sum)
        if ql <= l && r <= qr { return self.tree[node]; } // fully inside: return stored value
        let mid = (l + r) / 2;
        self.query_inner(2 * node,     l,       mid, ql, qr)
        + self.query_inner(2 * node + 1, mid + 1, r,   ql, qr)
    }
}

// Usage
let data = vec![1i64, 3, 5, 7, 9, 11];
let mut seg = SegTree::new(&data);

println!("{}", seg.query(1, 3)); // sum of [3,5,7] = 15
seg.update(2, 10);               // change index 2 from 5 to 10
println!("{}", seg.query(1, 3)); // sum of [3,10,7] = 20
```

## What This Unlocks

- **Range aggregate queries with updates**: leaderboard range sums, time-series range min/max, statistical window aggregates — all O(log n) per operation.
- **Competitive programming foundation**: range sum, range min/max, range GCD — all use the same tree structure with a different combine function.
- **Lazy propagation extension**: add "lazy tags" to internal nodes to support range updates (add X to all elements in [l, r]) in O(log n) instead of O(n).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Range query | O(n) scan | O(log n) segment tree |
| Point update | O(1) array set | O(log n) tree propagation |
| Build time | O(n) | O(n) bottom-up |
| Memory | O(n) | O(4n) — internal nodes |
| Operations supported | any via recursion | any associative op |
| vs. Fenwick tree | — | more general; Fenwick is simpler for prefix sums |
