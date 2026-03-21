📖 **[View on hightechmind.io →](https://hightechmind.io/rust/259-result-monad-error-chaining)**

---

# Example 259: Result Monad — Error Chaining
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Chain multiple validation steps on a string input — parsing, positivity, and parity — so that the first failure short-circuits the remaining checks and returns a descriptive error.

## Learning Outcomes

- `Result::and_then` is Rust's direct equivalent of OCaml's monadic bind (`>>=`)
- The `?` operator desugars to early-return on `Err`, giving monadic sequencing with imperative syntax
- Validation pipelines map naturally to the railway-oriented programming metaphor
- `map_err` converts foreign error types into owned `String` errors without allocating on the success path

## OCaml Approach

OCaml defines a custom `>>=` infix operator on `result` that pattern-matches: `Error` values pass through untouched while `Ok` values are unwrapped and fed to the next function. The chain `parse_int s >>= check_positive >>= check_even` reads left-to-right and terminates at the first `Error`.

## Rust Approach

Rust's `Result` provides `and_then` as the standard bind combinator, making `.and_then(check_positive).and_then(check_even)` idiomatic. Alternatively, the `?` operator gives the same short-circuit semantics with sequential imperative style. Both forms compile to equivalent machine code; `?` is preferred in practice for readability.

## Key Differences

1. **Operator vs method:** OCaml uses a custom infix `>>=`; Rust uses the `.and_then()` method or the `?` operator — no operator overloading needed.
2. **Error conversion:** OCaml concatenates strings freely; Rust requires `map_err` to convert parse errors into the uniform `String` error type.
3. **Syntax sugar:** Rust's `?` operator provides do-notation-style sequencing without a monad typeclass — each `?` is an explicit bind step.
4. **Ownership:** Rust validation functions take `i64` by value (Copy type), avoiding any borrow complications in the chain.

## Exercises

1. Extend the error chain to use a custom error enum with distinct variants for each failure mode, and map each step's error type using `map_err`.
2. Implement `result_all` — analogous to `option_all` — that collects a `Vec<Result<T, E>>` into `Result<Vec<T>, E>`, returning the first error encountered.
3. Combine `Option` and `Result` chaining: write a function that looks up a configuration key (returning `Option`), parses its value (returning `Result`), and applies a range check (returning `Result`), threading through a unified error type.
