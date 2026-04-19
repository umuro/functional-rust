# Example 1216: List Filter — Select Elements by Predicate

**Difficulty:** ⭐
**Category:** Lists & HOF
**OCaml Source:** `List.filter` — OCaml standard library (`Stdlib.List`)

## Problem Statement

Given a list and a predicate `p : 'a -> bool`, return a new list containing
exactly those elements of the original list for which `p` returns `true`,
preserving their relative order.

## Learning Outcomes

- Translating `List.filter` to Rust's `Iterator::filter` combinator.
- How Rust closures and `Fn` trait bounds replace OCaml's first-class function values.
- The borrow-then-copy pattern: predicate takes `&T`, survivors are copied into a new `Vec`.
- Partial application (`let pos = List.filter p`) becomes either a closure capturing `p` or a generic function in Rust.

## OCaml Approach

`List.filter p lst` walks the cons list, keeping each head that satisfies `p`
and recursing on the tail.  Because functions curry by default, writing
`let pos = List.filter (fun n -> n > 0)` yields a reusable
`int list -> int list` with the predicate baked in — a classic partial
application.

## Rust Approach

Rust reaches for the iterator chain: `iter().filter(|x| p(x)).copied().collect()`.
The predicate is a closure implementing `Fn(&T) -> bool`, and `collect()`
materialises the filtered sequence into a fresh `Vec<T>`.  When the OCaml
version partially applies `List.filter`, Rust expresses the same idea by
writing a closure that captures the predicate or a generic `fn` parameterised
over the predicate type.

## Key Differences

1. **Evaluation strategy:** OCaml's `List.filter` is eager and allocates a new list; Rust's `Iterator::filter` is lazy and only materialises when `collect` runs.
2. **Borrowing vs ownership:** OCaml threads immutable values freely; Rust's predicate borrows `&T`, then `copied()` / `cloned()` decides when to produce owned elements.
3. **Partial application:** OCaml curries by default, so `List.filter p` is already a function; Rust expresses this with closures or generic functions parameterised by the predicate.
4. **Recursion style:** OCaml's native cons-cell pattern match (`| x :: xs ->`) maps onto Rust's slice pattern match (`[h, rest @ ..]`).
