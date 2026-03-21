📖 **[View on hightechmind.io →](https://hightechmind.io/rust/366-segment-tree)**

---

# 366: Segment Tree
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Database range aggregation, stock price range queries, and competitive programming problems frequently ask: "What is the sum/min/max over elements from index L to R?" With a plain array, this is O(n) per query. Sorting allows binary search for range bounds but not arbitrary aggregates. The segment tree (developed independently in multiple contexts, popularized in competitive programming circa 1990s) achieves O(log n) for both range queries and point updates by maintaining a binary tree where each node stores the aggregate for a contiguous subarray. This is the data structure behind range aggregate functions in databases and time-series systems.

## Learning Outcomes

- Build a segment tree stored in a flat array with the "1-indexed" convention (left child at `2v`, right at `2v+1`)
- Understand the heap-like layout: root at index 1, leaves at indices n..2n
- Implement recursive `build` from a base array in O(n)
- Implement recursive range query `query(ql, qr)` in O(log n)
- Implement point update `update(pos, delta)` propagating changes up the tree in O(log n)
- Generalize the aggregate operation (sum, min, max, GCD) by changing the merge function

## Rust Application

```rust
pub struct SegmentTree {
    data: Vec<i64>,
    n: usize,
}

impl SegmentTree {
    pub fn new(arr: &[i64]) -> Self {
        let n = arr.len();
        let mut st = Self { data: vec![0; 4 * n], n };
        if n > 0 { st.build(arr, 1, 0, n - 1); }
        st
    }

    fn build(&mut self, arr: &[i64], v: usize, l: usize, r: usize) {
        if l == r { self.data[v] = arr[l]; return; }
        let m = (l + r) / 2;
        self.build(arr, 2 * v, l, m);       // left child
        self.build(arr, 2 * v + 1, m + 1, r); // right child
        self.data[v] = self.data[2 * v] + self.data[2 * v + 1]; // aggregate
    }

    pub fn query(&self, ql: usize, qr: usize) -> i64 {
        self.query_rec(1, 0, self.n - 1, ql, qr)
    }

    fn query_rec(&self, v: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if ql > r || qr < l { return 0; }       // outside range
        if ql <= l && r <= qr { return self.data[v]; } // fully inside
        let m = (l + r) / 2;
        self.query_rec(2 * v, l, m, ql, qr) + self.query_rec(2 * v + 1, m + 1, r, ql, qr)
    }

    pub fn update(&mut self, pos: usize, delta: i64) {
        self.update_rec(1, 0, self.n - 1, pos, delta);
    }

    fn update_rec(&mut self, v: usize, l: usize, r: usize, pos: usize, delta: i64) {
        if l == r { self.data[v] += delta; return; }
        let m = (l + r) / 2;
        if pos <= m { self.update_rec(2 * v, l, m, pos, delta); }
        else { self.update_rec(2 * v + 1, m + 1, r, pos, delta); }
        self.data[v] = self.data[2 * v] + self.data[2 * v + 1];
    }
}
```

`4 * n` storage is standard — the recursion can produce up to 4n nodes even for an n-element array due to the tree structure. The "1-indexed heap" layout means left child of node `v` is `2v`, right child is `2v + 1` — no pointer storage needed.

## OCaml Approach

```ocaml
let build arr =
  let n = Array.length arr in
  let data = Array.make (4 * n) 0 in
  let rec go v l r =
    if l = r then data.(v) <- arr.(l)
    else begin
      let m = (l + r) / 2 in
      go (2*v) l m; go (2*v+1) (m+1) r;
      data.(v) <- data.(2*v) + data.(2*v+1)
    end
  in
  go 1 0 (n-1); data

let query data n ql qr =
  let rec go v l r =
    if ql > r || qr < l then 0
    else if ql <= l && r <= qr then data.(v)
    else let m = (l + r) / 2 in
      go (2*v) l m + go (2*v+1) (m+1) r
  in
  go 1 0 (n-1)
```

The algorithm is identical — OCaml's recursive functions mirror Rust's recursive methods. Both use mutable arrays for the tree storage.

## Key Differences

| Aspect | Rust `SegmentTree` | OCaml segment tree |
|--------|-------------------|--------------------|
| Storage | `Vec<i64>` (heap) | `int array` |
| Mutability | `&mut self` for update | Array mutation |
| Aggregate | Hardcoded `+` (extensible via generic) | Hardcoded `+` |
| Lazy propagation | Requires `lazy: Vec<i64>` array | Same pattern |
| Alternative | Fenwick tree for prefix sums only | Same |

## Exercises

1. **Min segment tree**: Change the aggregate from `+` to `min`; implement `range_min(ql, qr)` and verify correctness on a range that spans multiple subtrees.
2. **Lazy propagation**: Implement range-update (add `delta` to all elements in `[ql, qr]`) using a lazy propagation array; push lazy values down when recursing into children.
3. **Generic aggregate**: Make `SegmentTree<T>` generic over a monoid `(T, identity: T, combine: fn(T, T) -> T)`; test with `(i64, 0, +)`, `(i64, i64::MAX, min)`, and `(i64, 1, *)`.
