**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐⭐⭐  

[fenwick-tree on hightechmind.io](https://hightechmind.io/posts/functional-rust/fenwick-tree)

---

## Problem Statement

Implement a Fenwick tree (Binary Indexed Tree) for O(log n) prefix sum queries and point updates. The key operation is `lowbit(i) = i & (-i)` — isolating the lowest set bit — which drives both update traversal (add lowbit to climb) and query traversal (subtract lowbit to descend). The tree is 1-indexed internally for clean bit arithmetic.

## Learning Outcomes

- Implement `update(i, delta)`: starting at 1-indexed position `i+1`, add `delta` to `tree[idx]` and advance via `idx += lowbit(idx)`
- Implement `prefix_sum(i)`: starting at `i+1`, accumulate `tree[idx]` and retreat via `idx -= lowbit(idx)`
- Implement `range_sum(l, r)` as `prefix_sum(r) - prefix_sum(l-1)`
- Understand why `i & (-i)` in two's complement isolates the lowest set bit
- Build a Fenwick tree from an array in O(n) using incremental updates

## Rust Application

```rust
pub struct FenwickTree {
    n: usize,
    tree: Vec<i64>,  // 1-indexed: tree[0] unused
}

impl FenwickTree {
    pub fn new(n: usize) -> Self {
        FenwickTree { n, tree: vec![0i64; n + 1] }
    }

    pub fn update(&mut self, i: usize, delta: i64) {
        let mut idx = (i + 1) as i64;
        while idx <= self.n as i64 {
            self.tree[idx as usize] += delta;
            idx += idx & (-idx);  // climb: add lowbit
        }
    }

    pub fn prefix_sum(&self, i: usize) -> i64 {
        let mut idx = (i + 1) as i64;
        let mut sum = 0i64;
        while idx > 0 {
            sum += self.tree[idx as usize];
            idx -= idx & (-idx);  // descend: subtract lowbit
        }
        sum
    }

    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        if l == 0 { self.prefix_sum(r) }
        else { self.prefix_sum(r) - self.prefix_sum(l - 1) }
    }
}
```

The `i & (-i)` trick works in two's complement: `-i` flips all bits and adds 1, so the lowest set bit of `i` remains set while all higher bits cancel. For `i = 12` (binary `1100`): `-12 = 0100` in low bits, so `12 & -12 = 4`.

Update climbs the tree (adding lowbit), touching all ancestors that cover position `i`. Query descends (subtracting lowbit), accumulating partial sums that together cover `[0, i]`.

## OCaml Approach

```ocaml
type t = {
  n: int;
  tree: int array;  (* 1-indexed *)
}

let create n = { n; tree = Array.make (n + 1) 0 }

let lowbit i = i land (-i)

let update fw i delta =
  let idx = ref (i + 1) in
  while !idx <= fw.n do
    fw.tree.(!idx) <- fw.tree.(!idx) + delta;
    idx := !idx + lowbit !idx
  done

let prefix_sum fw i =
  let idx = ref (i + 1) in
  let sum = ref 0 in
  while !idx > 0 do
    sum := !sum + fw.tree.(!idx);
    idx := !idx - lowbit !idx
  done;
  !sum
```

OCaml's `land` and unary `-` correspond to Rust's `&` and unary `-`. Both implementations use the same `while` loop structure with `ref` (OCaml) vs `let mut` (Rust) for mutable indices.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Lowbit | `idx & (-idx)` | `idx land (- idx)` |
| Index conversion | `(i + 1) as i64` then cast back | Same arithmetic |
| Signed arithmetic | `i64` for safe negation | `int` (63-bit on 64-bit systems) |
| `range_sum` | `prefix(r) - prefix(l-1)` | Same |

The Fenwick tree uses O(n) space and O(log n) time for both operations, making it more cache-friendly and simpler to implement than a segment tree — at the cost of supporting only prefix-sum-decomposable queries (no arbitrary range functions like min/max).

## Exercises

1. Implement `set(i, value)`: compute the current value with `range_sum(i, i)`, then call `update(i, value - current)`.
2. Build a Fenwick tree from a slice in O(n) using `from_slice` (n sequential updates) vs O(n log n) naive approach.
3. Implement `find_kth(k) -> usize`: find the leftmost position where prefix sum >= k using binary lifting on the Fenwick tree.
4. Implement a 2D Fenwick tree for range sum queries on a matrix.
5. Benchmark Fenwick tree vs segment tree for 1,000,000 random updates and prefix queries.
