📖 **[View on hightechmind.io →](https://hightechmind.io/rust/788-coin-change-dp)**

---

# 788-coin-change-dp — Coin Change
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

The coin change problem asks: given denominations of coins and an amount, what is the minimum number of coins needed to make that amount? Greedy fails for non-standard denominations (e.g., coins of 1, 3, 4 — greedy gives 3 coins for 6, but optimal is 2). DP finds the true minimum. The counting variant (number of ways to make change) is used in combinatorics and financial applications. Canonical in algorithm courses since the 1970s.

## Learning Outcomes

- Implement `coin_change(coins, amount) -> Option<usize>` returning `None` when impossible
- Use the bottom-up recurrence: `dp[i] = min over coins c: dp[i-c] + 1`
- Distinguish the minimum-count problem from the counting-ways problem (`coin_change_ways`)
- Understand why the top-down memoized approach is equivalent but sometimes cleaner
- Reconstruct the actual coins chosen, not just the count

## Rust Application

`coin_change` initializes `dp = [usize::MAX; amount+1]` with `dp[0] = 0`. For each amount `i` and each coin `c`, if `c <= i` and `dp[i-c] != MAX`, updates `dp[i]`. Returns `dp[amount]` or `None` for `MAX`. `coin_change_ways` counts combinations using the classic unbounded knapsack recurrence. Tests cover the classic `coins=[1,2,5], amount=11` case (3 coins: 5+5+1) and impossible amounts.

## OCaml Approach

OCaml implements the same bottom-up DP with `Array.make (amount+1) max_int` and a nested `for` loop. Functional style uses `List.fold_left` over coins. The `Int.max_int` sentinel instead of `usize::MAX`. Reconstruction uses a `prev: int array` tracking which coin was used at each step. OCaml's `min_coins` function in competitive programming is often a one-liner using `Array.fold_left`.

## Key Differences

1. **Sentinel value**: Rust uses `usize::MAX` as "impossible"; OCaml uses `max_int` — same concept, different constant name.
2. **Overflow guard**: Rust's `usize::MAX + 1` would panic; the `!= usize::MAX` guard prevents this; OCaml has the same issue with `max_int + 1`.
3. **Return type**: Rust's `Option<usize>` clearly communicates impossibility; OCaml might return `-1` or `Int.max_int` without a dedicated option type.
4. **Counting variant**: `coin_change_ways` uses a different loop structure (iterate coins in outer loop for combinations); both languages implement it identically.

## Exercises

1. Implement `coin_change_reconstruct(coins, amount) -> Option<Vec<usize>>` that returns the actual coins used, not just the count.
2. Add `coin_change_limited(coins, counts, amount)` where `counts[i]` limits how many times coin `i` can be used (bounded knapsack variant).
3. Implement the top-down memoized version and benchmark it against the bottom-up version for large amounts (100,000+). Which is faster in practice?
