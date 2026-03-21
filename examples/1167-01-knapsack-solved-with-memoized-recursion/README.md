# 0/1 Knapsack Solved with Memoized Recursion
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


**Source:** https://rosettacode.org/wiki/Knapsack_problem/0-1#OCaml


## Problem Statement

The 0/1 knapsack problem asks: given a set of items each with a weight and value, and a knapsack of fixed capacity, which items maximize total value without exceeding the weight limit? Each item is either taken (1) or left (0) — no fractional amounts. The naive recursive solution is exponential (2ⁿ subsets), but memoization reduces it to O(n × W) by caching overlapping subproblems — a canonical example of dynamic programming via top-down recursion.

## Learning Outcomes

- Understand why the knapsack problem has optimal substructure and overlapping subproblems
- Learn how memoization transforms exponential recursion into polynomial time
- See how Rust's `HashMap` serves as a memo table for recursive DP
- Understand the difference between top-down (memoized recursion) and bottom-up (tabulation) DP

## Rust Application

The memoized recursive solution uses a `HashMap<(usize, usize), u64>` keyed on `(item_index, remaining_capacity)`. Rust requires the memo table to be passed as a mutable reference through recursive calls, making the data flow explicit. Each call checks the cache before recursing, and stores its result before returning. The borrow checker ensures the cache is never aliased unsafely during recursion.

## OCaml Approach

OCaml uses a `Hashtbl` for memoization, typically wrapped in a `let memo = Hashtbl.create 16` at the top of the function. The recursive function is defined with `let rec solve i w = match Hashtbl.find_opt memo (i, w) with Some v -> v | None -> ...`. OCaml's closures naturally capture the memo table without threading it as a parameter, making the code more concise.

## Key Differences

1. **Mutable state threading**: Rust requires `&mut HashMap` as an explicit parameter; OCaml closures capture mutable `Hashtbl` by reference implicitly.
2. **Recursion style**: Both use `let rec` / recursive functions, but Rust lacks TCO, so deep recursion on very large inputs risks stack overflow.
3. **Cache key**: Both use `(item_index, capacity)` tuples; Rust's `HashMap` requires `Hash + Eq` on keys — tuples derive both automatically.
4. **Overflow safety**: Rust uses `u64` with explicit bounds; OCaml uses arbitrary-precision integers via `Zarith` for large instances.

## Exercises

1. Implement the base memoized solution and verify it produces the same answer as the brute-force exponential version on small inputs.
2. Add item reconstruction: trace back through the memo table to identify which items were selected.
3. Compare performance against bottom-up tabulation (fill a 2D array row by row) on a 100-item, capacity-1000 instance.
