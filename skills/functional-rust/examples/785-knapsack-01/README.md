# 785. 0/1 Knapsack: DP Table Approach

**Difficulty:** 3  **Level:** Intermediate

Classic resource-allocation DP: given items with weights and values, maximise total value within a weight budget — with three implementations and full traceback.

## The Problem This Solves

The 0/1 knapsack problem appears wherever you must make binary include/exclude decisions under a resource constraint. Scheduling jobs within a time budget, selecting features within a sprint velocity, packing cargo within weight limits — all are knapsack variants. The "0/1" means each item is taken whole or not at all (unlike the fractional knapsack, which is solvable greedily).

This is also the canonical introduction to 2D DP tables: the recurrence `dp[i][w] = max(dp[i-1][w], dp[i-1][w-weight_i] + value_i)` captures the exact structure of "take or skip" decisions at each step. Understanding this table and its traceback unlocks most other combinatorial DP problems.

## The Intuition

Work backwards: at each item, you either take it (gaining its value, spending its weight) or skip it. The DP table stores the best achievable value for every `(item_count, remaining_capacity)` pair. Fill it left-to-right, row by row. The answer is at `dp[n][capacity]`. Traceback recovers *which* items were selected by walking backwards through the table. In OCaml you'd naturally write the recursive version first and add a `Hashtbl` for memoisation; in Rust, the tabulation approach with a `Vec<Vec<u64>>` is more cache-friendly and lets you traceback directly.

## How It Works in Rust

```rust
// O(n × capacity) time, O(n × capacity) space
pub fn knapsack_tab(items: &[Item], capacity: usize) -> KnapsackResult {
    let n = items.len();
    // dp[i][w] = max value using first i items with capacity w
    let mut dp = vec![vec![0u64; capacity + 1]; n + 1];

    for i in 1..=n {
        let item = &items[i - 1];
        for w in 0..=capacity {
            dp[i][w] = if item.weight > w {
                dp[i-1][w]                                    // can't take it
            } else {
                dp[i-1][w].max(item.value + dp[i-1][w - item.weight]) // take or skip
            };
        }
    }

    // Traceback: walk backwards to find selected items
    let mut selected = Vec::new();
    let mut w = capacity;
    for i in (1..=n).rev() {
        if dp[i][w] != dp[i-1][w] {       // value changed → item i was taken
            selected.push(i - 1);
            w = w.saturating_sub(items[i-1].weight);
        }
    }
    // ...
}

// Space-optimised: 1D rolling array — O(capacity) space
// Must iterate weight right-to-left to avoid using an item twice
pub fn knapsack_1d(items: &[Item], capacity: usize) -> u64 {
    let mut dp = vec![0u64; capacity + 1];
    for item in items {
        for w in (item.weight..=capacity).rev() {   // ← right-to-left is critical
            dp[w] = dp[w].max(item.value + dp[w - item.weight]);
        }
    }
    dp[capacity]
}
```

The 1D optimisation reduces space from O(n × W) to O(W). The right-to-left iteration ensures each item is considered only once (if you iterate left-to-right, you'd allow taking the same item multiple times — that's the unbounded knapsack variant).

## What This Unlocks

- **Portfolio optimisation**: select investments maximising return within a risk/capital budget.
- **Sprint planning**: maximise feature delivery given team velocity, treating each story as an item with story-point weight and business-value score.
- **Subset-sum and coin change**: both are knapsack variants with unit values; the same 1D rolling array technique applies directly.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| DP table | `Array.make_matrix` (mutable) or recursive + `Hashtbl` | `vec![vec![0u64; W+1]; n+1]` |
| Take/skip recurrence | `max` on recursive calls | `dp[i-1][w].max(value + dp[i-1][w-wt])` |
| Traceback | Recursive descent through memo table | Reverse loop over 2D `Vec` |
| Space optimisation | Swap two arrays each row | Single `Vec`, iterate `rev()` |
| Item struct | Record type with fields | `struct Item { weight: usize, value: u64 }` |
