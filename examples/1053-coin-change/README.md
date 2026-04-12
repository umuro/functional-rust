📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1053-coin-change)**

---

# 1053-coin-change — Coin Change
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

The coin change problem asks: given coin denominations and a target amount, what is the minimum number of coins to make that amount? This is a classic DP problem with applications in currency systems, memory allocators (minimal number of blocks to satisfy a request), and NP-hard scheduling approximations.

It is the canonical example of unbounded knapsack / complete knapsack: each coin can be used multiple times, and you want to minimize the count rather than maximize value.

## Learning Outcomes

- Implement bottom-up DP for the coin change problem
- Understand the `dp[i] = min(dp[i], dp[i-coin] + 1)` recurrence
- Implement the same algorithm with top-down memoization
- Recognize the unbounded knapsack structure
- Handle the "no solution" case (return -1)

## Rust Application

`src/lib.rs` implements `coin_change_dp` using a `Vec<usize>` of size `amount+1`, initialized to `amount+1` (infinity sentinel). The inner loop iterates over all coins and takes the minimum. `coin_change_memo` implements the same with a `HashMap` cache and recursive calls.

The problem has O(amount × num_coins) time complexity and O(amount) space. The DP approach is preferred over greedy (which fails for non-standard coin sets like {1, 3, 4} where greedy picks 4+1+1 but optimal is 3+3).

## OCaml Approach

```ocaml
let coin_change coins amount =
  let max_val = amount + 1 in
  let dp = Array.make (amount + 1) max_val in
  dp.(0) <- 0;
  for i = 1 to amount do
    List.iter (fun coin ->
      if coin <= i && dp.(i - coin) + 1 < dp.(i) then
        dp.(i) <- dp.(i - coin) + 1
    ) coins
  done;
  if dp.(amount) > amount then -1 else dp.(amount)
```

The OCaml version is structurally identical. Both use the same recurrence; the implementation differences are syntactic.

## Key Differences

1. **Sentinel value**: Both use `amount + 1` as an infinity sentinel; Rust's explicit `usize::MAX` would overflow on addition.
2. **Mutable array**: OCaml's array is mutable by default; Rust's `Vec<usize>` requires no special annotation for mutation.
3. **List vs slice iteration**: OCaml iterates over coins with `List.iter`; Rust uses `for &coin in coins`.
4. **Negative amounts**: Rust's `usize` cannot represent negative values; the check `if c <= i` prevents underflow. OCaml's `int` allows subtraction without overflow.

## Exercises

1. Extend to also reconstruct the actual coins used: return `Vec<i32>` of denominations that sum to the amount.
2. Implement a variant that counts the number of distinct ways to make the amount (combination sum count DP).
3. Solve the variant where each coin can only be used once (0/1 knapsack variant) and compare the recurrence change.
