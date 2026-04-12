📖 **[View on hightechmind.io →](https://hightechmind.io/rust/057-fold-left)**

---

# Example 057: fold_left — Tail-Recursive Accumulator
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



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

1. **Accumulator pattern:** `fold_left` is the direct encoding of the accumulator pattern — instead of returning from the base case and building the result on the way up, it carries the result forward as an argument.
2. **Tail recursive:** Because the recursive call is in tail position (`fold_left f (f acc head) tail`), OCaml can optimize this to a loop. Rust's `iter.fold()` is already implemented as a loop — no recursion.
3. **Universal:** `fold_left` can implement `map`, `filter`, `reverse`, `length`, and `sum` — it is computationally complete for list operations. Understanding this demonstrates the power of the abstraction.
4. **Non-commutative order:** `fold_left (+) 0 [1;2;3] = ((0+1)+2)+3 = 6`. For addition it doesn't matter, but for subtraction: `fold_left (-) 0 [1;2;3] = ((0-1)-2)-3 = -6` vs `fold_right (-) [1;2;3] 0 = 1-(2-(3-0)) = 2`.

## Exercises

1. Implement `running_sum` using `fold_left` that returns a `Vec` of prefix sums (e.g., `[1,2,3]` → `[1,3,6]`).
2. Use `fold_left` to implement `group_by_first_char` that builds a `HashMap<char, Vec<String>>` grouping strings by their first character.
3. Implement a left fold over a binary tree (not a list) and use it to compute the sum of all node values; compare with a right fold version and explain the traversal order difference.

4. **Running maximum**: Use `fold_left` to compute the running maximum of a list — at each step, the accumulator is the maximum seen so far. Return both the final maximum and the position where it first occurred.
5. **Grouping via fold**: Implement `group_by<T: Clone, K: Eq + Hash>(list: &[T], key: impl Fn(&T) -> K) -> HashMap<K, Vec<T>>` using a single `fold` call — no loops, no explicit iteration.
