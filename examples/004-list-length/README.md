📖 **[View on hightechmind.io →](https://hightechmind.io/rust/004-list-length)**

---

# Example 004: List Length

**Difficulty:** ⭐  
**Category:** Lists & Recursion  
**OCaml Source:** OCaml.org 99 Problems #4

## Problem Statement

Compute the number of elements in a list. OCaml demonstrates naive recursion vs tail-recursive accumulator.

Computing the length of a sequence is so fundamental that most languages cache it. Rust's `Vec<T>` and slice `&[T]` store a `usize` length alongside the data pointer, making `.len()` O(1) and infallible. This contrasts sharply with OCaml's linked lists, where length requires O(n) traversal. Understanding this trade-off — list flexibility vs array length caching — is essential for choosing the right data structure.

## Learning Outcomes

- Understanding O(1) `.len()` on Rust slices vs O(n) in OCaml
- Tail recursion and why Rust doesn't guarantee TCO
- `fold` as the functional accumulator pattern
- Stack overflow risks with naive recursion
- How data structure choice (array vs linked list) fundamentally changes complexity

## OCaml Approach

Two implementations: naive `1 + length_naive t` (stack risk) and tail-recursive `aux n` with accumulator. The tail-recursive version is production-ready in OCaml.

OCaml's `List.length : 'a list -> int` is O(n) and non-tail-recursive in older versions. The standard library now uses the tail-recursive form internally. For large lists, always prefer the tail-recursive version: `let rec aux acc = function [] -> acc | _ :: t -> aux (acc + 1) t`. Calling `List.length` in a tight loop on long lists has been a real performance footgun in OCaml codebases.

## Rust Approach

`.len()` is trivially O(1) since slices store their length. For educational purposes, we also implement `fold`-based and recursive versions matching the OCaml patterns.

## Key Differences

1. **Complexity:** Rust `.len()` is O(1) — the length is stored with the slice; OCaml lists require O(n) traversal
2. **TCO:** OCaml guarantees tail-call optimization; Rust does not — tail-recursive Rust code can still overflow
3. **Fold:** Rust's `fold` is iterative (no stack growth); OCaml's `List.fold_left` is tail-recursive — same safety guarantees
4. **Practical value:** In Rust, manual length computation is purely educational; in OCaml, the tail-recursive version matters
5. **Large lists:** OCaml's naive version risks stack overflow; Rust's naive version does too, but `.len()` eliminates the need

1. **Complexity:** `slice.len()` is O(1) — the length is stored alongside the data. `List.length` in OCaml is O(n) — the list must be traversed. This alone makes arrays preferable for length-sensitive operations.
2. **Stack safety:** OCaml guarantees tail-call optimization, so the accumulator-based `length_aux` is safe for arbitrarily long lists. Rust does not guarantee TCO; the recursive version will stack-overflow on lists longer than ~10,000 elements.
3. **Type:** Rust `.len()` returns `usize` (unsigned). OCaml `List.length` returns `int` (signed). Rust's choice avoids negative-length bugs at the type level.

## Exercises

1. Implement `list_length` without using `.len()` or `.count()` — use only a fold or manual recursion.
2. Write `length_bounded` that counts elements up to a maximum `limit`, stopping early once the limit is reached (without scanning the rest of the list).
3. Implement `lengths` that takes a `Vec<Vec<T>>` and returns a `Vec<usize>` containing the length of each inner list, using only iterator combinators.
