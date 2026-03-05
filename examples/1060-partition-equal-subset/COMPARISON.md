# Partition Equal Subset Sum — Comparison

## Core Insight
This is a boolean variant of 0/1 knapsack: can we fill a "knapsack" of capacity `total/2`? The 1D boolean DP with reverse iteration is the standard approach. The HashSet variant trades space efficiency for conceptual clarity.

## OCaml Approach
- `IntSet` module (functor-based ordered set) for reachable sums
- `IntSet.fold` to generate new sums — purely functional
- Boolean array with reverse `for` loop for standard DP
- `Array.fold_left (+) 0` for sum

## Rust Approach
- `HashSet` for reachable sums with collect/insert pattern
- Boolean `vec!` with reverse range `(num..=target).rev()`
- Early exit with `contains(&target)` check
- `iter().sum()` for total

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Set type | `Set.Make(Int)` (ordered, tree-based) | `HashSet<i32>` (hash-based) |
| Set iteration | `IntSet.fold` | `.iter().map().filter().collect()` |
| Boolean DP | `Array.make n false` | `vec![false; n]` |
| Sum | `Array.fold_left (+) 0` | `iter().sum()` |
| Early exit | Must check after fold | `if reachable.contains` mid-loop |
