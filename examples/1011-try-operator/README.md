📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1011-try-operator)**

---

# 1011-try-operator — The ? (Try) Operator

## Problem Statement

Explicit error propagation is verbose. Without language support, every fallible call requires a `match` block to check for errors and forward them upward. In a deep call stack, this repetition obscures the happy-path logic. Haskell's `do` notation and OCaml's `let*` binding both address this. Rust's `?` operator is the compile-time desugaring of the same idea: it extracts the `Ok` value or returns the `Err` early, optionally converting the error type via `From`.

The `?` operator is foundational in Rust: virtually all production code that handles I/O, parsing, or network operations uses it.

## Learning Outcomes

- Understand what `?` desugars to: early return + optional `From` conversion
- Implement `From<ParseIntError>` for a custom error type to enable `?` across error type boundaries
- Chain multiple fallible calls in a single function body using `?`
- Distinguish the `?` operator from `unwrap` and understand the safety trade-off
- Know when `?` can and cannot be used (function return type must implement `Try`)

## Rust Application

`src/lib.rs` defines `AppError` with variants for configuration, parsing, validation, and service errors. It implements `From<ParseIntError>` so that `s.parse::<i64>()?` automatically converts the standard library error into `AppError::ParseFailed`. The pipeline function chains `read_data`, `parse_data`, and `validate` using `?` — each step can fail with a different underlying error, but the `From` impl makes conversion seamless.

This pattern is the foundation of crates like `anyhow` and `thiserror`, which automate `From` implementations for entire error hierarchies.

## OCaml Approach

OCaml's `let*` syntax (available via `Result.bind` in a `let* = Result.bind` binding) provides equivalent ergonomics:

```ocaml
let ( let* ) = Result.bind

let pipeline key =
  let* data = read_data key in
  let* n = parse_data data in
  validate n
```

Without `let*`, each step requires explicit `match` or `|>` with `Result.bind`. Unlike Rust's `From`, OCaml requires explicit type conversion between error variants.

## Key Differences

1. **Implicit type conversion**: Rust's `?` calls `From::from` on the error type automatically; OCaml's `let*` requires the error types to already match or uses explicit conversion.
2. **Desugaring**: `?` desugars to `match result { Ok(v) => v, Err(e) => return Err(e.into()) }`; OCaml's `let*` desugars to `bind`.
3. **Return type requirement**: Rust's `?` only works in functions returning `Result<_, _>` or `Option<_>`; OCaml's `let*` is polymorphic over any monadic type.
4. **Stack context**: Rust's `?` with `anyhow::Context` can attach human-readable context to each propagation point; OCaml requires manual wrapping.

## Exercises

1. Add a fifth pipeline stage `rate_limit(n: i64) -> Result<i64, AppError>` and chain it with `?` into the existing pipeline.
2. Remove the `From<ParseIntError>` impl and observe the compiler error. Then add a `map_err` call to fix it manually.
3. Write a function that calls the pipeline in a loop for a list of keys and collects all `AppError` values, continuing on error rather than returning early.
