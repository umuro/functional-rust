# Collatz Conjecture: OCaml vs Rust

## The Core Insight
The Collatz sequence is simple enough that both languages express it almost identically. The interesting comparison is in error handling: OCaml's `Result` type and Rust's `Result<T, E>` serve the same purpose but with different ergonomics around the `?` operator and pattern matching.

## OCaml Approach
OCaml uses `if/else` chains for the three cases (n=1, even, odd). The `Result` type (`Ok`/`Error`) wraps the safe API. Pattern matching on the result uses `match ... with Ok s -> ... | Error e -> ...`. The recursive version relies on OCaml's guaranteed tail-call optimization for the `else` branches (though this particular recursion isn't strictly tail-recursive due to `1 + ...`).

## Rust Approach
Rust uses `match` with guards (`n if n % 2 == 0 => ...`) for cleaner pattern matching. The `Result<u64, String>` return type forces callers to handle errors. The iterative version with `while current != 1` is more idiomatic Rust and avoids any stack concerns. The `?` operator (not shown here) could propagate errors even more concisely in larger pipelines.

## Side-by-Side
| Concept | OCaml | Rust |
|---------|-------|------|
| Pattern match | `if n = 1 then ... else if ...` | `match n { 1 => ..., n if ... => ... }` |
| Error type | `(int, string) result` | `Result<u64, String>` |
| Safe API | `Ok (collatz_steps n)` | `Ok(collatz_steps(n as u64))` |
| Integer types | `int` (63-bit) | `u64` (explicit unsigned) |
| Iteration | Recursion (idiomatic) | `while` loop (idiomatic) |

## What Rust Learners Should Notice
- Match guards (`n if n % 2 == 0`) are Rust's way to add conditions to match arms — cleaner than nested `if/else`
- `Result<u64, String>` is the Rust equivalent of OCaml's `(int, string) result` — both force explicit error handling
- Rust's explicit integer types (`u64` vs `i64`) make the domain constraint (positive integers) partially expressible in the type system
- The iterative version is preferred in Rust — it's clear, stack-safe, and often faster
- `as u64` is an explicit cast — Rust never silently converts between integer types

## Further Reading
- [The Rust Book — Error Handling](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Exercism Collatz Conjecture](https://exercism.org/tracks/ocaml/exercises/collatz-conjecture)
