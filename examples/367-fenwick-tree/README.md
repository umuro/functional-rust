📖 **[View on hightechmind.io →](https://hightechmind.io/rust/367-fenwick-tree)**

---

# 367: Fenwick Tree (Binary Indexed Tree)
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Prefix sum queries ("sum of elements 1 through i") and point updates are needed in ranking systems (how many scores are below X?), inversion counting in arrays, and order-statistic operations. A Fenwick tree (Peter Fenwick, 1994) achieves O(log n) for both operations using only O(n) space and remarkably simple index arithmetic. Unlike the segment tree, which needs 4n space and explicit left/right children, the Fenwick tree is a flat array where the "tree structure" is encoded in the binary representation of indices. It's the most space-efficient and cache-friendly structure for prefix sum queries with updates.

## Learning Outcomes

- Build a Fenwick tree using 1-indexed storage where index 0 is unused
- Update position i by adding delta and propagating via `i += i & (-i)` (lowbit trick)
- Query prefix sum `[1..i]` by summing and walking via `i -= i & (-i)`
- Convert to range query `[l..r]` as `prefix_sum(r) - prefix_sum(l-1)`
- Understand the lowbit `i & (-i)` as isolating the lowest set bit in binary
- Compare Fenwick tree to segment tree: simpler code, less memory, prefix-sum only

## Rust Application

```rust
pub struct FenwickTree {
    tree: Vec<i64>,
    n: usize,
}

impl FenwickTree {
    pub fn new(n: usize) -> Self {
        Self { tree: vec![0; n + 1], n } // 1-indexed: tree[0] unused
    }

    pub fn from_slice(arr: &[i64]) -> Self {
        let mut ft = Self::new(arr.len());
        for (i, &v) in arr.iter().enumerate() {
            ft.update(i + 1, v); // convert 0-indexed to 1-indexed
        }
        ft
    }

    pub fn update(&mut self, mut i: usize, delta: i64) {
        while i <= self.n {
            self.tree[i] += delta;
            i += i & i.wrapping_neg(); // i += lowbit(i)
        }
    }

    pub fn prefix_sum(&self, mut i: usize) -> i64 {
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i -= i & i.wrapping_neg(); // i -= lowbit(i)
        }
        sum
    }

    pub fn range_sum(&self, l: usize, r: usize) -> i64 {
        if l == 0 { self.prefix_sum(r) }
        else { self.prefix_sum(r) - self.prefix_sum(l - 1) }
    }
}
```

`i & i.wrapping_neg()` isolates the lowest set bit of `i`. For `i = 6` (binary `110`), `lowbit = 010 = 2`. The update walks up the tree adding `lowbit` each time; the query walks down subtracting `lowbit`. These paths cover exactly the right combinations to compute prefix sums.

## OCaml Approach

```ocaml
let make n = Array.make (n + 1) 0

let lowbit i = i land (-i)

let update tree n i delta =
  let i = ref i in
  while !i <= n do
    tree.(!i) <- tree.(!i) + delta;
    i := !i + lowbit !i
  done

let prefix_sum tree i =
  let i = ref i and sum = ref 0 in
  while !i > 0 do
    sum := !sum + tree.(!i);
    i := !i - lowbit !i
  done;
  !sum

let range_sum tree l r =
  prefix_sum tree r - (if l = 1 then 0 else prefix_sum tree (l - 1))
```

Both implementations are essentially identical — the algorithm is purely index arithmetic on a flat array. OCaml uses `land` for bitwise AND, Rust uses `&`.

## Key Differences

| Aspect | Rust Fenwick tree | OCaml Fenwick tree |
|--------|------------------|-------------------|
| Lowbit | `i & i.wrapping_neg()` | `i land (-i)` |
| Storage | `Vec<i64>` (1-indexed, index 0 unused) | `int array` (same) |
| Range query | `range_sum(l, r)` | Two `prefix_sum` calls |
| vs Segment tree | Less code, less memory, prefix sums only | Same tradeoff |
| Range updates | Requires second Fenwick tree (difference array trick) | Same |

## Exercises

1. **Inversion count**: Count inversions in an array (pairs i < j where arr[i] > arr[j]) using a Fenwick tree: process elements right-to-left, querying "how many elements already processed are smaller than current?"
2. **Order statistics**: Given a stream of integers in range [1, 1000], use a Fenwick tree as a frequency array to answer "what is the k-th smallest element seen so far?" using binary search on prefix sums.
3. **2D Fenwick tree**: Extend to 2D: `update(x, y, delta)` and `prefix_sum(x, y)` give sum of the rectangle [1..x][1..y]; implement by nesting two levels of the lowbit trick.
