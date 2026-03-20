📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1060-partition-equal-subset)**

---

# 1060-partition-equal-subset — Partition Equal Subset Sum
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Given a set of positive integers, can it be partitioned into two subsets with equal sums? This is an NP-complete problem in general, but the DP solution runs in O(n × sum/2) — pseudo-polynomial and practical for reasonable input sizes.

The problem is equivalent to: can a subset sum to exactly half of the total? Applications include load balancing (split tasks equally between two workers), scheduling (divide work evenly), and cryptographic range proofs.

## Learning Outcomes

- Transform "can we partition into two equal subsets" into "can subset sum to target = sum/2"
- Implement boolean DP for subset sum
- Use a `HashSet<i32>` approach as an alternative
- Understand early termination when total is odd
- Connect to the NP-hardness of general partition problems

## Rust Application

`src/lib.rs` implements `can_partition` with a boolean `dp` array of size `target+1`. `dp[j]` is true if sum `j` is reachable using some subset. The inner loop iterates capacity in reverse (like 0/1 knapsack) to ensure each number is used at most once. `can_partition_set` uses a `HashSet<i32>` of reachable sums — cleaner but uses more memory for large sums.

The reverse iteration and the connection to 0/1 knapsack are the two key insights: this IS knapsack where weights == values == nums[i] and the target == sum/2.

## OCaml Approach

```ocaml
let can_partition nums =
  let total = List.fold_left (+) 0 nums in
  if total mod 2 <> 0 then false
  else
    let target = total / 2 in
    let dp = Array.make (target + 1) false in
    dp.(0) <- true;
    List.iter (fun num ->
      for j = target downto num do
        if dp.(j - num) then dp.(j) <- true
      done
    ) nums;
    dp.(target)
```

Structurally identical to Rust. The `downto` for reverse iteration is the OCaml idiom.

## Key Differences

1. **Boolean array**: Both use `bool` arrays; Rust could use `Vec<bool>` (1 byte/element) or bit manipulation for memory efficiency.
2. **HashSet approach**: Rust's `can_partition_set` using `HashSet` is more readable but less space-efficient than the boolean array approach.
3. **Early termination**: Both check `total % 2 != 0` first; Rust uses `!=` and OCaml uses `<>` for not-equal.
4. **NP-completeness**: Both solve the NP-complete problem efficiently only because the target value is bounded; for arbitrary-precision integers, the problem becomes intractable.

## Exercises

1. Extend to `k_partition(nums: &[i32], k: usize) -> bool` that checks if the array can be split into k equal-sum subsets.
2. Implement `partition_count(nums: &[i32]) -> usize` that counts the number of ways to partition into two equal-sum subsets.
3. Write `min_subset_difference(nums: &[i32]) -> i32` that finds the partition minimizing the difference between the two subset sums (not necessarily equal).
