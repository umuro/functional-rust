📖 **[View on hightechmind.io →](https://hightechmind.io/rust/792-rod-cutting-dp)**

---

# 792: Rod Cutting Problem

**Difficulty:** 3  **Level:** Advanced

Maximize profit by cutting a rod into pieces — an unbounded knapsack variant solved with bottom-up DP.

## The Problem This Solves

You have a rod of length `n` and a price list where `prices[i]` is the value of a piece of length `i+1`. You can cut the rod any way you like (or not at all). What's the maximum revenue you can achieve? This is the canonical **unbounded knapsack** problem: each piece length can be used multiple times, and you want to maximize value given a capacity constraint (the rod length).

Real-world analogues: stock cutting in manufacturing (minimize waste), task scheduling (split a time budget across repeatable tasks for maximum value), and bandwidth allocation. Any time you have a divisible resource and want to optimally assign it to repeatable options, rod cutting is the template.

The key insight is that each cut produces a *prefix* of the remaining rod — so `dp[i]` only needs to look at all smaller sub-problems, making bottom-up filling straightforward. Unlike 0/1 knapsack (each item used once), here you can always try cutting off another piece of the same length.

## The Intuition

`dp[i]` = maximum revenue for a rod of length `i`. Base case: `dp[0] = 0`. Recurrence: for each length `i`, try cutting off a piece of length `j` (1 ≤ j ≤ i) and add its price to `dp[i-j]`. Take the max over all `j`. The `cuts[i]` array tracks which cut length was chosen at each step, enabling O(n) reconstruction. Total: O(n²) time, O(n) space.

## How It Works in Rust

```rust
fn rod_cut(prices: &[u64]) -> (u64, Vec<usize>) {
    let n = prices.len();
    let mut dp   = vec![0u64; n + 1];    // max revenue for each rod length
    let mut cuts = vec![0usize; n + 1];  // which cut was optimal

    for i in 1..=n {
        for j in 1..=i {
            // prices[j-1] = value of a piece of length j
            let v = prices[j - 1] + dp[i - j];
            if v > dp[i] {
                dp[i]   = v;
                cuts[i] = j;  // cut off j from the front
            }
        }
    }

    // Reconstruct: follow the cuts[] array until rod length = 0
    let mut pieces = Vec::new();
    let mut len = n;
    while len > 0 {
        pieces.push(cuts[len]);
        len -= cuts[len];
    }
    (dp[n], pieces)
}
```

The reconstruction loop is clean: `cuts[len]` always tells you the size of the first piece, and you subtract it until nothing remains. `u64` prices prevent overflow on large inputs. The `prices` slice is 0-indexed (`prices[j-1]` = price for length `j`), matching the natural 1-indexed problem statement.

## What This Unlocks

- **Unbounded knapsack pattern** — rod cutting is the clearest example of "items can be reused"; the same inner loop structure applies to coin change, word break, and interval scheduling.
- **Parallel decision tables** — maintaining `cuts[]` alongside `dp[]` is the standard way to reconstruct which choices led to the optimal value, without re-running the DP.
- **1D DP reduction** — grid/interval DPs often reduce to 1D when the problem is linear (e.g., rod cutting reduces from 2D "start,end" to 1D "remaining length").

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| 1D mutable DP table | `Array.make (n+1) 0` | `vec![0u64; n + 1]` |
| 0-indexed price list | `prices.(j-1)` | `prices[j - 1]` — same |
| Reconstruction loop | Tail-recursive or `while` | Idiomatic `while len > 0` |
| Overflow protection | OCaml ints wrap silently | `u64` gives 64-bit range; use `saturating_add` for extra safety |
