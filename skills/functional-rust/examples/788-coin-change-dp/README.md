# 788. Coin Change: Minimum Coins DP

**Difficulty:** 3  **Level:** Intermediate

Given coin denominations, find the fewest coins that sum to a target amount — plus count the number of distinct ways to make change.

## The Problem This Solves

Coin change is the archetypal 1D DP problem. It appears directly in payment systems (minimise transaction count), and in abstracted form everywhere capacity is being optimally allocated in discrete units: minimise the number of network packets to fill a buffer, minimise the number of tiles to cover a floor, minimise the number of API calls to reach a quota.

The "count ways" variant is equally important: how many distinct decompositions exist? This appears in combinatorics, probability, and cryptography (e.g., counting distinct factorizations or key derivation paths).

## The Intuition

Build a 1D array `dp` where `dp[i]` = minimum coins to make amount `i`. Start with `dp[0] = 0` (base case: zero coins for zero amount) and fill left to right. For each amount `i`, try every coin and take the minimum. If no combination works, the cell stays at `∞`. This is the unbounded knapsack pattern — you can use the same coin multiple times — so you iterate *forward* through the table (unlike 0/1 knapsack which goes backward). OCaml's recursive memoised version naturally expresses the same recurrence; Rust's tabulation version is usually faster due to better cache behaviour.

## How It Works in Rust

```rust
// Bottom-up tabulation — O(amount × coins) time, O(amount) space
pub fn coin_change_tab(coins: &[u64], amount: usize) -> Option<u64> {
    let mut dp = vec![u64::MAX; amount + 1];
    dp[0] = 0;

    for i in 1..=amount {
        for &coin in coins {
            let coin = coin as usize;
            if coin <= i && dp[i - coin] != u64::MAX {
                dp[i] = dp[i].min(dp[i - coin] + 1);
            }
        }
    }

    if dp[amount] == u64::MAX { None } else { Some(dp[amount]) }
}

// Count distinct ways (order-independent combinations)
// Loop over coins in outer loop: each coin used 0 or more times
pub fn count_ways(coins: &[u64], amount: usize) -> u64 {
    let mut dp = vec![0u64; amount + 1];
    dp[0] = 1;
    for &coin in coins {
        for i in coin as usize..=amount {
            dp[i] = dp[i].saturating_add(dp[i - coin as usize]);
        }
    }
    dp[amount]
}

// Reconstruct which coins were chosen
pub fn coin_change_with_coins(coins: &[u64], amount: usize) -> Option<Vec<u64>> {
    let mut last_coin = vec![0u64; amount + 1];
    // ... same DP, track last_coin[i] = which coin improved dp[i]
    // Backtrack: while remaining > 0 { push last_coin[remaining]; remaining -= coin; }
}
```

Note the loop structure difference: `count_ways` has coins in the *outer* loop (each coin extended across all amounts) to count unordered combinations. Swapping to amounts-outer would count ordered permutations instead.

## What This Unlocks

- **Payment systems**: ATM software computing minimum-denomination change; point-of-sale systems verifying that exact change is possible.
- **Partition counting**: the "count ways" pattern counts integer partitions, used in combinatorics, cryptographic analysis, and musical rhythm generation.
- **Unbounded resource allocation**: any problem where you can reuse a resource unit (tiles, packets, words in a sentence) and want minimum or count of allocations.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Infinity sentinel | `max_int` or `None` | `u64::MAX` (use `!= MAX` guards to prevent overflow) |
| Memoisation | `Hashtbl` keyed on amount | `HashMap<u64, Option<u64>>` |
| Tabulation | Imperative loop over `Array` | `Vec<u64>` with forward iteration |
| Reconstruction | Recursive traceback | `while remaining > 0` with `last_coin` array |
| Count vs min | Same table, `+` vs `min` | Same pattern: change operator in inner loop |
