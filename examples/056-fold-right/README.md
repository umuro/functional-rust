📖 **[View on hightechmind.io →](https://hightechmind.io/rust/056-fold-right)**

---

# Example 056: fold_right — Structural Recursion
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement `fold_right`, the fundamental higher-order function that processes a list from right to left by replacing each cons (`::`) with an operator and `[]` with an initial value: `fold_right f [a; b; c] init = f a (f b (f c init))`.

## Learning Outcomes

- Understand fold_right as "replacing constructors" in a list
- See why fold_right is not tail-recursive (builds up stack frames)
- Compare OCaml's natural recursion with Rust's iterator-based `rfold`
- Use partial application to create specialized functions from fold
- Recognize when right-fold ordering matters (e.g., string concatenation, list copy)

## OCaml Approach

OCaml's `fold_right` recurses to the end of the list first, then applies `f` as it unwinds. This matches the list's recursive structure perfectly — it's the most natural recursion over a cons-list.

## Rust Approach

Rust offers three approaches:
1. **Idiomatic:** `iter().sum()`, `iter().product()`, `to_vec()` — specialized methods
2. **Functional:** Explicit recursive `fold_right` over slices using `[head, tail @ ..]` patterns
3. **rfold:** `iter().rfold()` on `DoubleEndedIterator` — Rust's built-in right fold

## Key Differences

1. **Stack safety:** OCaml's fold_right can stack overflow on large lists; Rust's `rfold` uses an internal loop (no stack growth)
2. **Borrowing:** Rust's fold takes `&T` references from slices; OCaml's takes owned values from the cons-list
3. **Partial application:** OCaml creates specialized functions via currying (`let sum = fold_right (+) lst 0`); Rust uses closures
4. **No cons-list:** Rust slices are contiguous memory — `rfold` iterates backwards by index, not by recursive structure
5. **Copy semantics:** `copy` in OCaml is free (structural sharing); in Rust it allocates a new Vec

## Exercises

1. Use `fold_right` to implement `map` — derive the mapping operation purely from a right fold.
2. Implement `maximum` using `fold_right` that returns the largest element in a non-empty list wrapped in `Option`.
3. Use `fold_right` to implement `flatten` that concatenates a list of lists into a single list, and compare its stack usage to an iterative approach for large inputs.
