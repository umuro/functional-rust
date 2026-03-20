📖 **[View on hightechmind.io →](https://hightechmind.io/rust/795-subset-sum-dp)**

---

# 795-subset-sum-dp — Subset Sum
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Subset sum asks: can a subset of given numbers sum to a target value? It is NP-complete in general but solvable in pseudo-polynomial O(n × target) time via DP. It underlies the 0/1 knapsack problem, is used in cryptography (merkle-hellman knapsack), and appears in scheduling (can a set of tasks exactly fill a time slot?) and partitioning problems (can employees be split into equal-salary teams?).

## Learning Outcomes

- Model subset sum as `dp[w]` = whether sum w is achievable
- Apply the 0/1 knapsack recurrence with boolean values: `dp[i] = dp[i] || dp[i - num]`
- Iterate right-to-left to prevent using the same number twice
- Implement `count_subsets(nums, target)` for counting the number of valid subsets
- Understand the connection to the partition problem (target = sum/2)

## Rust Application

`subset_sum(nums, target)` handles non-negative inputs (skips negative). `dp = [false; target+1]` with `dp[0] = true`. For each number, iterates from target down to num, updating `dp[i] = dp[i] || dp[i-num]`. `count_subsets` uses `usize` counts instead of booleans, adding to `dp[i]` instead of OR-ing. Tests cover the classic `[3,34,4,12,5,2]` with target 9 (yes: 4+5 or 3+4+2).

## OCaml Approach

OCaml implements with `Array.make (t+1) false`. The right-to-left iteration: `for i = t downto n do dp.(i) <- dp.(i) || dp.(i-n) done`. The counting variant uses `int array`. OCaml's `Array.blit` can copy and shift for the "extend with reversed order" optimization. The partition problem variant: `subset_sum nums (sum/2)` where `sum = List.fold_left (+) 0 nums`.

## Key Differences

1. **Right-to-left**: Both languages use the same right-to-left trick to avoid counting elements twice; the direction is critical and easy to get wrong.
2. **Boolean vs count**: The same code structure works for both variants by changing the array type and operation (OR vs add).
3. **NP-completeness**: Both languages can only solve small instances efficiently; for large targets, the pseudo-polynomial O(n × target) becomes infeasible.
4. **Cryptographic connection**: The merkle-hellman knapsack cryptosystem (broken by Shamir) is based on subset sum; Rust is used in modern post-quantum cryptography research.

## Exercises

1. Implement `partition_equal(nums: &[i32]) -> bool` using subset sum: check if the total sum is even and if `sum/2` is achievable.
2. Implement `multi_subset_sum(nums, targets: &[i32]) -> Vec<bool>` that answers multiple target queries in one pass using the same DP table.
3. Solve the exact partition into k subsets of equal sum using DFS with the subset sum DP as a feasibility oracle.
