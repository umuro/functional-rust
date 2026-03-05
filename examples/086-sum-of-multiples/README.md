# Example 086: Sum of Multiples

**Difficulty:** ⭐⭐
**Category:** Math / Set Operations
**OCaml Source:** Custom — set-union approach using `Set.Make(Int)`

## Problem Statement

Given a list of factors and an upper limit, compute the sum of all unique positive integers below the limit that are divisible by at least one of the factors. Zero factors are ignored.

## Learning Outcomes

- How `HashSet` deduplicates values exactly like OCaml's `Set.Make(Int)`
- Using `flat_map` + `step_by` to generate arithmetic sequences idiomatically
- Translating `List.fold_left` over a mutable accumulator into Rust's `fold`
- How inclusion-exclusion with LCM provides an O(2^k) mathematical alternative

## OCaml Approach

OCaml uses a polymorphic set module (`Set.Make(Int)`) to accumulate multiples without double-counting. `List.fold_left` iterates over factors, generating each factor's multiples with `List.init`, and inserting them into the set. `IS.fold (+)` then reduces the set to a sum.

## Rust Approach

The idiomatic Rust solution uses `flat_map` to lazily generate all multiples via `step_by`, collects them into a `HashSet<u64>` for deduplication, and sums the result. The mathematical variant avoids allocations entirely by applying the inclusion-exclusion principle over all non-empty subsets of unique factors.

## Key Differences

1. **Set construction:** OCaml uses a functor `Set.Make(Int)` producing a balanced BST; Rust uses `HashSet` (hash table) — same deduplication semantics, different performance profile.
2. **Sequence generation:** OCaml's `List.init n f` creates an eager list; Rust's `(f..limit).step_by(f)` is a lazy iterator — no intermediate allocation.
3. **Accumulator pattern:** OCaml's `List.fold_left` with a functional set returns a new set each step; Rust's `fold` with `|mut acc, ...| { acc.insert(...); acc }` mutates in place.
4. **Zero handling:** Both treat factor `0` as a no-op to avoid division by zero; Rust's `filter` and OCaml's `if factor = 0 then s` serve the same guard role.
