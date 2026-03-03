# Example 057: fold_left — Tail-Recursive Accumulator

**Difficulty:** ⭐⭐
**Category:** Higher-Order Functions
**OCaml Source:** [CS3110 — Fold](https://cs3110.github.io/textbook/chapters/hop/fold.html)

## Problem Statement

Implement `fold_left`, the tail-recursive fold that processes a list left to right with an accumulator: `fold_left f init [a; b; c] = f (f (f init a) b) c`. Use it to implement sum, product, maximum, and reverse.

## Learning Outcomes

- Understand fold_left as a tail-recursive accumulator pattern
- See why fold_left is stack-safe for large lists (tail recursion)
- Map OCaml's `fold_left` to Rust's `Iterator::fold`
- Implement reverse using fold_left (classic trick)
- Compare fold_left vs fold_right evaluation order

## OCaml Approach

OCaml's `fold_left` is tail-recursive — the recursive call is in tail position, so the compiler can optimize it to a loop. This makes it safe for arbitrarily large lists, unlike `fold_right`.

## Rust Approach

1. **Idiomatic:** `iter().sum()`, `iter().product()`, `iter().max()`, `.reverse()`
2. **Functional:** Custom `fold_left` function using a for loop (Rust doesn't guarantee TCO)
3. **Iterator::fold:** Rust's built-in left fold — `iter().fold(init, |acc, x| ...)`

## Key Differences

1. **TCO:** OCaml guarantees tail-call optimization; Rust does not, so our "recursive" fold_left uses an iterative loop
2. **Maximum safety:** OCaml's `maximum` panics on empty list (`List.hd`); Rust returns `Option<T>`
3. **Mutability:** Rust's fold accumulator is moved on each step (ownership transfer); OCaml's is copied/shared
4. **Specialization:** Rust's `.sum()` and `.product()` use traits (`Sum`, `Product`) for type-safe folding
5. **Reverse:** OCaml reverses by consing (`x :: acc`); Rust can reverse in-place with `.reverse()` (O(1) extra space)
