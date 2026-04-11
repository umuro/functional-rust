📖 **[View on hightechmind.io →](https://hightechmind.io/rust/056-fold-right)**

---

# Example 056: fold_right — Structural Recursion

**Difficulty:** ⭐⭐
**Category:** Higher-Order Functions
**OCaml Source:** [CS3110 — Fold](https://cs3110.github.io/textbook/chapters/hop/fold.html)

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

1. **Right vs left fold direction:** `fold_right f [a; b; c] init = f a (f b (f c init))` — processes right-to-left. `fold_left f init [a; b; c] = f (f (f init a) b) c` — processes left-to-right. For non-associative operations, the order matters.
2. **Stack usage:** `fold_right` on a linked list is not tail-recursive — it builds a chain of pending `f` calls on the stack. For large lists, it risks stack overflow. `fold_left` with an accumulator is O(1) stack.
3. **Rust's `fold` is `fold_left`:** `iter.fold(init, |acc, x| f(acc, x))` applies `f` left-to-right — it is `fold_left`. Rust has no built-in `fold_right` because right-fold on iterators requires buffering all elements first.
4. **`rfold` for right fold:** `iter.rev().fold(init, |acc, x| ...)` approximates right-fold on finite iterators. Alternatively, use `iter.collect::<Vec<_>>().into_iter().rfold(init, f)` if true right-fold semantics are needed.

## Exercises

1. Use `fold_right` to implement `map` — derive the mapping operation purely from a right fold.
2. Implement `maximum` using `fold_right` that returns the largest element in a non-empty list wrapped in `Option`.
3. Use `fold_right` to implement `flatten` that concatenates a list of lists into a single list, and compare its stack usage to an iterative approach for large inputs.

4. **List from fold_right**: Implement `map_via_fold_right<T: Clone, U>(f: impl Fn(&T) -> U, list: &[T]) -> Vec<U>` using only `fold_right` — demonstrate that fold_right can express map.
5. **Natural number arithmetic**: Use `fold_right` to implement `sum_right` and `product_right` on a list. Compare the results with `fold_left` for subtraction to see where the difference in direction matters.
