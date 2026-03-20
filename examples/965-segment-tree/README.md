[segment-tree on hightechmind.io](https://hightechmind.io/posts/functional-rust/segment-tree)

---

## Problem Statement

Implement a segment tree for range sum queries with point updates. The tree is stored in a flat array of size `4 * n`. Each internal node stores the sum of its range. `query(l, r)` returns the sum over index range `[l, r]` in O(log n). `update(pos, value)` replaces the value at `pos` and propagates the change upward in O(log n).

## Learning Outcomes

- Represent the segment tree in a flat `Vec<i64>` with 1-indexed nodes: children of node `i` are `2*i` and `2*i+1`
- Implement `build` recursively: leaf nodes store array values; internal nodes store left+right sums
- Implement `query(l, r)` with range splitting: when the query range fully covers a node's range, return directly; otherwise recurse into children
- Implement `update(pos, value)` that replaces the leaf and propagates sums back up the tree
- Understand why the flat array needs `4 * n` entries (not `2 * n`) to safely accommodate all possible recursion patterns

## Rust Application

```rust
pub struct SegmentTree {
    n: usize,
    tree: Vec<i64>,
}

impl SegmentTree {
    pub fn new(n: usize) -> Self {
        SegmentTree { n, tree: vec![0i64; 4 * n] }
    }

    pub fn build(&mut self, arr: &[i64]) {
        self.build_rec(1, 0, self.n - 1, arr);
    }

    fn build_rec(&mut self, node: usize, lo: usize, hi: usize, arr: &[i64]) {
        if lo == hi { self.tree[node] = arr[lo]; return; }
        let mid = (lo + hi) / 2;
        self.build_rec(2 * node, lo, mid, arr);
        self.build_rec(2 * node + 1, mid + 1, hi, arr);
        self.tree[node] = self.tree[2 * node] + self.tree[2 * node + 1];
    }

    pub fn query(&self, l: usize, r: usize) -> i64 {
        self.query_rec(1, 0, self.n - 1, l, r)
    }

    fn query_rec(&self, node: usize, lo: usize, hi: usize, l: usize, r: usize) -> i64 {
        if r < lo || hi < l { return 0; }          // disjoint
        if l <= lo && hi <= r { return self.tree[node]; }  // fully covered
        let mid = (lo + hi) / 2;
        self.query_rec(2 * node, lo, mid, l, r)
            + self.query_rec(2 * node + 1, mid + 1, hi, l, r)
    }
}
```

The node indexing scheme: root is `1`, children of node `i` are `2i` (left) and `2i+1` (right). This 1-indexed scheme avoids root-at-index-0 edge cases. The `4*n` allocation is a conservative upper bound that guarantees no index overflow for any recursion path.

The query base cases: `r < lo || hi < l` means the query range is disjoint from this node's range (return 0, the identity for sum). `l <= lo && hi <= r` means the query range fully covers this node's range (return stored sum without recursing).

## OCaml Approach

```ocaml
type segment_tree = {
  n: int;
  tree: int array;
}

let create n = { n; tree = Array.make (4 * n) 0 }

let rec build_rec st node lo hi arr =
  if lo = hi then st.tree.(node) <- arr.(lo)
  else
    let mid = (lo + hi) / 2 in
    build_rec st (2 * node) lo mid arr;
    build_rec st (2 * node + 1) (mid + 1) hi arr;
    st.tree.(node) <- st.tree.(2 * node) + st.tree.(2 * node + 1)

let rec query_rec st node lo hi l r =
  if r < lo || hi < l then 0
  else if l <= lo && hi <= r then st.tree.(node)
  else
    let mid = (lo + hi) / 2 in
    query_rec st (2 * node) lo mid l r +
    query_rec st (2 * node + 1) (mid + 1) hi l r
```

OCaml's mutable array `st.tree.(node) <- value` corresponds directly to Rust's `self.tree[node] = value`. The algorithm is structurally identical; the main syntactic difference is `.(i)` vs `[i]` for array indexing.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Array indexing | `self.tree[i]` | `st.tree.(i)` |
| Recursive methods | `&mut self` — unique mutable reference | Mutable record fields |
| Node size | `4 * n` | Same |
| Identity value | `0` (hardcoded for sum) | Same |

Segment trees support any associative operation (sum, min, max, GCD) by replacing the `+` in build, query, and update with the desired operation. The structure generalizes to lazy propagation for range updates.

## Exercises

1. Implement `update(&mut self, pos: usize, value: i64)` that replaces the value at `pos` and propagates.
2. Extend to support range minimum queries by replacing `+` with `min` in build and query.
3. Implement lazy propagation: `range_add(l, r, delta)` that adds `delta` to all elements in `[l, r]` in O(log n).
4. Generalize with a `fold_fn: Fn(i64, i64) -> i64` and `identity: i64` parameter to support arbitrary monoids.
5. Implement a persistent segment tree where each update returns a new root (for historical queries).
