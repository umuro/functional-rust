📖 **[View on hightechmind.io →](https://hightechmind.io/rust/191-effect-exceptions)**

---

# Effects as Exceptions
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Exceptions are a special case of algebraic effects — an effect that does not resume the computation. When an exception is thrown, the stack is unwound to the nearest handler, which may handle the exception and continue or re-throw it. Understanding exceptions as non-resumable effects unifies the conceptual model: both exceptions and effects interrupt the normal control flow and transfer control to a handler.

## Learning Outcomes

- Understand exceptions as non-resumable algebraic effects
- Learn how `Result<T, E>` and the `?` operator implement exception-like error handling in Rust
- See the connection between effect handler `Abort` results and exception propagation
- Compare Rust's typed exceptions (`Result`) with OCaml's dynamic exceptions (`exn`)

## Rust Application

Rust uses `Result<T, E>` and `?` for typed exception handling. `?` is syntactic sugar for "if this is `Err(e)`, return early from the function with `Err(e)`" — it is a non-resumable effect. The `From<E> for F` trait enables automatic error conversion at `?` sites. Custom error types using `thiserror` or `anyhow` encode the error domain. Unlike free monad-based effects, Rust's `?` is zero-cost — the compiler generates the same code as explicit `match` with early return.

## OCaml Approach

OCaml has two exception mechanisms:
1. Traditional exceptions: `exception MyError of string; raise (MyError "msg")` — caught with `try ... with MyError msg -> ...`
2. OCaml 5 effects: `effect Fail : string` — caught with a handler that does not call `continue`

The traditional OCaml approach uses dynamic exceptions (like Java's unchecked exceptions). OCaml's `result` type provides checked, typed exceptions similar to Rust's `Result`.

## Key Differences

1. **Typing**: Rust's `Result<T, E>` is statically typed — the error type is part of the function signature; OCaml's `exn` is a single open extensible type (all exceptions have the same type).
2. **Zero-cost**: Rust's `?` compiles to conditional branches — no runtime overhead on the happy path; OCaml's exceptions use stack unwinding — no overhead until thrown.
3. **Propagation**: Rust's `?` requires the error types to match (or implement `From`); OCaml's exceptions propagate to any matching handler regardless of type.
4. **Checked vs. unchecked**: Rust forces callers to handle `Result` explicitly; OCaml's exceptions can propagate silently — closer to Java's unchecked exceptions.

## Exercises

1. Implement a multi-error accumulator using `Vec<E>` — collect all errors rather than stopping at the first.
2. Write a `try_all<T, E>(ops: Vec<impl FnOnce() -> Result<T, E>>) -> Result<Vec<T>, Vec<E>>` that runs all operations and returns all successes or all errors.
3. Implement a retry combinator: `retry<T, E>(n: usize, f: impl FnMut() -> Result<T, E>) -> Result<T, E>`.
