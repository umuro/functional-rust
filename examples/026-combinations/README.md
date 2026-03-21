📖 **[View on hightechmind.io →](https://hightechmind.io/rust/026-combinations)**

---

# 026 — Generate the Combinations of K Distinct Objects Chosen from N Elements
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Generating all k-element subsets (combinations) of a set (OCaml 99 Problems #26) is a fundamental combinatorics problem. C(n, k) = n! / (k! · (n-k)!) is the number of ways to choose k elements from n without regard to order. The recursive structure — either include the first element (and choose k-1 from the rest) or skip it (and choose k from the rest) — is the textbook example of backtracking.

Combinations appear in feature selection for machine learning, generating test cases for pairwise testing, computing binomial coefficients, scheduling round-robin tournaments, and drug trial design. The recursive divide-and-conquer structure here is the basis for all backtracking algorithms.

## Learning Outcomes

- Implement combinations using recursive backtracking: include/exclude the head
- Understand the recursion: `combinations(k, [h|t]) = [h|c] for c in combinations(k-1, t) ++ combinations(k, t)`
- Handle base cases: k=0 (yield empty set), k > remaining (yield nothing)
- Recognize combinations as the foundation for permutations, subsets, and power sets
- Use the result to understand how C(n, k) is computed recursively

## Rust Application

The recursive implementation: if `k == 0`, return `vec![vec![]]` (one empty combination). If the list is empty and k > 0, return `vec![]` (no combinations possible). Otherwise split on the head: combinations including the head = prepend head to each k-1 combination from the tail; combinations excluding the head = k combinations from the tail. Concatenate both. An iterative approach uses a bitmask over n elements.

## OCaml Approach

OCaml's canonical version: `let rec combinations k lst = match k, lst with | 0, _ -> [[]] | _, [] -> [] | k, x :: xs -> List.map (fun c -> x :: c) (combinations (k-1) xs) @ combinations k xs`. The `@` concatenates the two cases. This directly encodes the mathematical recurrence: C(k, n) = C(k-1, n-1) + C(k, n-1).

## Key Differences

1. **`@` vs extend**: OCaml's `@` concatenates two lists in O(|left|). Rust uses `result.extend(...)` which is O(|right|). For collecting into a flat `Vec<Vec<T>>`, Rust's approach avoids intermediate allocations.
2. **Clone cost**: Rust must clone each element when prepending it to combination sub-lists. OCaml's GC shares structure — prepending x to a list does not copy x.
3. **Stack depth**: Recursion depth is O(n) for both. OCaml guarantees TCO only for tail calls. The `combinations` recursion is not tail-recursive in either language (it recurses on two branches).
4. **Itertools**: Rust's `itertools` crate provides `(0..n).combinations(k)` — use `Itertools::combinations` in production code. OCaml has no such standard facility.

## Exercises

1. **Combinations with repetition**: Write `combinations_with_rep(list: &[i32], k: usize) -> Vec<Vec<i32>>` where elements can be chosen multiple times. The recursion changes to allow reusing elements.
2. **Count without generating**: Write `count_combinations(n: u64, k: u64) -> u64` using the multiplicative formula C(n,k) = n*(n-1)*...*(n-k+1) / k! without overflow. Use `u128` or arbitrary precision.
3. **Subsets (power set)**: Generalize to `power_set(list: &[i32]) -> Vec<Vec<i32>>` that returns all 2^n subsets. This is `combinations(0) ++ combinations(1) ++ ... ++ combinations(n)`.
