📖 **[View on hightechmind.io →](https://hightechmind.io/rust/291-result-combinators)**

---

# 291: Result Combinators
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Imperative error handling with nested `if/else` or try/catch blocks forces control flow restructuring whenever an operation might fail. The `Result<T, E>` type in Rust models explicit failure as a value, and its combinator methods enable functional-style error handling: transform successes, chain operations, and recover from failures — all without breaking out of an expression context. This mirrors OCaml's `Result.bind` and Haskell's `Either` monad.

## Learning Outcomes

- Use `map()` to transform the `Ok` value while preserving `Err`
- Use `map_err()` to transform the `Err` value while preserving `Ok`
- Chain fallible operations with `and_then()` — the monadic bind for `Result`
- Recover from errors with `or_else()` and `unwrap_or_else()`

## Rust Application

Result combinators compose without nested match expressions:

```rust
pub fn parse_and_divide(s: &str, divisor: i32) -> Result<i32, String> {
    parse_int(s)           // Result<i32, String>
        .and_then(|n| divide(n, divisor))  // chain: apply if Ok
        .map(|x| x * 2)   // transform Ok value
}

// map_err: convert error type
let result: Result<i32, String> = "42".parse::<i32>()
    .map_err(|e| format!("parse failed: {}", e));

// or_else: recover from error
let with_default = parse_int("bad").or_else(|_| Ok::<i32, String>(0));
```

## OCaml Approach

OCaml's `Result` module provides `Result.map`, `Result.bind` (`and_then` equivalent), and `Result.map_error`. The `let*` syntax sugar (OCaml 4.08+) desugars to `bind`:

```ocaml
let parse_and_divide s divisor =
  let* n = parse_int s in
  let* q = divide n divisor in
  Ok (q * 2)
(* let* desugars to Result.bind *)
```

## Key Differences

1. **Naming**: Rust uses `and_then` (from Haskell convention); OCaml uses `bind` and `let*` syntax.
2. **Error transformation**: Rust provides `map_err()` for transforming the error type; OCaml uses `Result.map_error`.
3. **Recovery**: `or_else()` in Rust; OCaml uses `Result.fold` or pattern matching.
4. **Type conversion**: `from()` and `?` operator automate error type conversion in Rust; OCaml requires explicit wrapping.

## Exercises

1. Chain five fallible operations using only `and_then()` without any `match` expressions, and verify the error from the second failure propagates correctly.
2. Implement a parser pipeline that parses a string as `"name:age"` using `and_then()` to split, parse the age, and validate it is between 0 and 150.
3. Use `or_else()` to implement a "try primary, fallback to secondary" pattern where a primary parse is retried with a fallback parser on failure.
