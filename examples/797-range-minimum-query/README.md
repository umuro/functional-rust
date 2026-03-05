# 797: Range Minimum Query (Sparse Table)

**Difficulty:** 4  **Level:** Advanced

After O(n log n) preprocessing, answer "what's the minimum in `arr[l..=r]`?" in O(1) — using a sparse table.

## The Problem This Solves

You have a static array and need to answer thousands of queries of the form "what's the minimum value between index `l` and index `r`?" Naively scanning is O(n) per query — too slow when you have millions of queries. Segment trees answer in O(log n) but add implementation complexity. The **sparse table** answers range minimum in O(1) after O(n log n) preprocessing, with no updates needed. It's the fastest structure for static RMQ.

Real-world uses: suffix array construction (LCP queries during string matching), computational geometry (lowest common ancestor in trees reduces to RMQ), sliding window minimum in stream processing, and interval scheduling queries. Any algorithm that repeatedly asks "what's the minimum in this range of a fixed array?" benefits from sparse table preprocessing.

The structure exploits one beautiful property of `min`: it's **idempotent** — `min(x, x) = x`. This means overlapping intervals can be combined freely without double-counting. You can cover any range `[l, r]` with just two overlapping power-of-two intervals and take the min of both, because repeated elements don't change the minimum.

## The Intuition

Build a table where `table[k][i]` = minimum of the `2^k` elements starting at index `i`. Level 0 is the original array. Level k is built from level k-1: `table[k][i] = min(table[k-1][i], table[k-1][i + 2^(k-1)])`. For a query `[l, r]`: find `k = floor(log₂(r-l+1))`. The two intervals `[l, l+2^k-1]` and `[r-2^k+1, r]` both have length `2^k`, both fit inside `[l,r]`, and together cover all of `[l,r]`. The answer is `min(table[k][l], table[k][r - 2^k + 1])`. O(1) query, O(n log n) build, O(n log n) space.

## How It Works in Rust

```rust
struct SparseTable {
    table: Vec<Vec<i64>>,  // table[k] = array of mins for windows of size 2^k
    log2:  Vec<usize>,     // precomputed floor(log2(i)) for fast lookup
}

impl SparseTable {
    fn build(arr: &[i64]) -> Self {
        let n = arr.len();
        // Number of levels = ceil(log2(n)) + 1
        let levels = usize::BITS as usize - n.leading_zeros() as usize;
        let mut table = vec![arr.to_vec()];  // level 0 = original array

        for k in 1..levels {
            let prev = &table[k - 1];
            let half = 1 << (k - 1);   // 2^(k-1)
            // Each row k is shorter: windows of size 2^k need at most n - 2^k + 1 starts
            let row: Vec<i64> = (0..n.saturating_sub((1 << k) - 1))
                .map(|i| prev[i].min(prev[i + half]))
                .collect();
            table.push(row);
        }

        // Precompute log2 table: log2[i] = floor(log2(i))
        let mut log2 = vec![0usize; n + 1];
        for i in 2..=n { log2[i] = log2[i / 2] + 1; }

        SparseTable { table, log2 }
    }

    fn query(&self, l: usize, r: usize) -> i64 {
        let k = self.log2[r - l + 1];           // largest power-of-two ≤ window size
        // Two overlapping windows of size 2^k — idempotence makes overlap safe
        self.table[k][l].min(self.table[k][r + 1 - (1 << k)])
    }
}
```

The `leading_zeros()` trick computes `ceil(log2(n))` without floating point. Each table row shrinks as `k` grows — the inner `Vec` lengths decrease by half each level, keeping total space at O(n log n). The log2 precomputation turns the `query`'s level selection into a single array lookup.

## What This Unlocks

- **Idempotent range queries** — the two-overlapping-windows trick works for any idempotent operator (`min`, `max`, `gcd`, `and`, `or`), but NOT for sum/count (use prefix sums or segment trees for those).
- **Precomputed logarithms** — the `log2[]` table trick replaces floating-point or bit-scan operations in hot query paths; the same pattern applies to level ancestor queries in trees.
- **Static vs. dynamic structures** — sparse table is optimal when the array never changes; if updates are needed, switch to a segment tree (O(log n) query and update).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| 2D ragged table | `Array.init levels (fun k -> ...)` | `Vec<Vec<i64>>` — push each level |
| Bit manipulation | `1 lsl k` | `1 << k` — same idiom |
| Leading zeros | `Int.clz` (OCaml 5.x) | `n.leading_zeros()` — built-in on all integer types |
| Log precomputation | Recursive `let rec` | Iterative `for i in 2..=n` — both O(n) |
