# GCD and LCM — Comparison

## Core Insight
The Euclidean algorithm is a pure recursive function on integers. Since integers are `Copy` in Rust, the translation is nearly identical — no ownership complications.

## OCaml Approach
- `let rec gcd a b = if b = 0 then a else gcd b (a mod b)` — concise one-liner
- `List.fold_left gcd h t` — fold over list with GCD as combining function
- Polymorphic integers (but typically `int`)

## Rust Approach
- Same recursive structure with explicit `u64` type
- `iter().copied().reduce(gcd)` mirrors `fold_left`
- Can also accept `impl IntoIterator` for generic input

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Recursion | `let rec` | plain `fn` (no keyword needed) |
| Modulo | `mod` (operator) | `%` (operator) |
| Fold | `List.fold_left` | `.iter().reduce()` |
| Integer type | `int` (63-bit) | `u64` (explicit) |
| Overflow | Silent wraparound | Panic in debug, wrap in release |

## Learner Notes
- Rust requires explicit numeric types — no polymorphic `int`
- `reduce` vs `fold`: reduce uses first element as initial value
- For integers, ownership is trivial — everything is Copy
- The `a / gcd(a,b) * b` form avoids overflow vs `a * b / gcd(a,b)`
