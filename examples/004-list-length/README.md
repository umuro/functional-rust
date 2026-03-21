📖 **[View on hightechmind.io →](https://hightechmind.io/rust/004-list-length)**

---

# Example 004: List Length
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Compute the number of elements in a list. OCaml demonstrates naive recursion vs tail-recursive accumulator.

## Learning Outcomes

- Understanding O(1) `.len()` on Rust slices vs O(n) in OCaml
- Tail recursion and why Rust doesn't guarantee TCO
- `fold` as the functional accumulator pattern
- Stack overflow risks with naive recursion
- How data structure choice (array vs linked list) fundamentally changes complexity

## OCaml Approach

Two implementations: naive `1 + length_naive t` (stack risk) and tail-recursive `aux n` with accumulator. The tail-recursive version is production-ready in OCaml.

## Rust Approach

`.len()` is trivially O(1) since slices store their length. For educational purposes, we also implement `fold`-based and recursive versions matching the OCaml patterns.

## Key Differences

1. **Complexity:** Rust `.len()` is O(1) — the length is stored with the slice; OCaml lists require O(n) traversal
2. **TCO:** OCaml guarantees tail-call optimization; Rust does not — tail-recursive Rust code can still overflow
3. **Fold:** Rust's `fold` is iterative (no stack growth); OCaml's `List.fold_left` is tail-recursive — same safety guarantees
4. **Practical value:** In Rust, manual length computation is purely educational; in OCaml, the tail-recursive version matters
5. **Large lists:** OCaml's naive version risks stack overflow; Rust's naive version does too, but `.len()` eliminates the need

## Exercises

1. Implement `list_length` without using `.len()` or `.count()` — use only a fold or manual recursion.
2. Write `length_bounded` that counts elements up to a maximum `limit`, stopping early once the limit is reached (without scanning the rest of the list).
3. Implement `lengths` that takes a `Vec<Vec<T>>` and returns a `Vec<usize>` containing the length of each inner list, using only iterator combinators.
