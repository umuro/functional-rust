📖 **[View on hightechmind.io →](https://hightechmind.io/rust/045-result-basics)**

---

# 045 — Result Basics

## Problem Statement

`Result<T, E>` extends `Option<T>` with error information: instead of `None`, a failure produces `Err(e)` carrying a typed error value `e`. This is how Rust handles all recoverable errors — file I/O, network calls, parsing, validation — without exceptions or error codes. The type system forces callers to handle both `Ok(value)` and `Err(error)` cases.

`Result` originates from Haskell's `Either` type and OCaml's `result` type. Unlike exceptions (which are invisible in type signatures), `Result` makes the possibility of failure explicit: a function returning `Result<i32, ParseError>` clearly communicates "this can fail with a ParseError". This is the basis of Rust's famous "fearless error handling".

## Learning Outcomes

- Construct and destructure `Result<T, E>` with `Ok(value)` and `Err(error)`
- Use `match` to handle both success and error cases
- Use `.is_ok()`, `.is_err()`, `.unwrap()`, `.unwrap_or(default)` for common patterns
- Understand the difference between `Option` (no value vs value) and `Result` (success vs error with info)
- Use `.ok()` to convert `Result<T,E>` to `Option<T>` when the error type does not matter

- Use `Result<T, E>` to represent operations that can fail with a typed error — avoiding sentinel values and exceptions
- Use `expect("message")` over `unwrap()` to provide context when panicking in tests or when `Err` is impossible

## Rust Application

`Result::Ok(42)` wraps a success value; `Result::Err("not found")` wraps an error. `match result { Ok(v) => use(v), Err(e) => handle(e) }` handles both cases. `result.unwrap_or(0)` returns the value or a default on error. `result.unwrap_or_else(|e| fallback(e))` calls a function with the error. `result.ok()` converts to `Option<T>` discarding the error. The `.is_ok()` and `.is_err()` predicates check the variant without consuming the result.

## OCaml Approach

OCaml's `result` type: `type ('a, 'b) result = Ok of 'a | Error of 'b`. (Note: OCaml uses `Error`, not `Err`.) Usage: `match r with Ok v -> ... | Error e -> ...`. `Result.value r ~default:x` returns the value or default. `Result.get_ok r` panics on `Error`. `Result.is_ok r` and `Result.is_error r` are predicates. `Result.to_option r` converts to `option`.

## Key Differences

1. **`Err` vs `Error`**: Rust uses `Err(e)` for the error variant; OCaml uses `Error e`. This is purely a naming difference.
2. **Generic error type**: Rust's `Result<T, E>` is generic over both the success and error types. OCaml's `('a, 'b) result` is the same. This means errors can be strings, enums, custom structs, or any type.
3. **`?` propagation**: Rust's `?` operator works on `Result` — `let x = fallible()?;` returns `Err(e)` early if the function fails. OCaml needs `let*` syntax with ppx_let or explicit match.
4. **Exception vs Result**: OCaml code often uses exceptions for errors (e.g., `Not_found` from `List.find`); functional OCaml code prefers `result`. Rust has no exceptions at all — `Result` is the only mechanism.

1. **Typed errors:** `Result<T, E>` carries a typed error `E`. This forces the caller to decide how to handle each specific error case. `Option` only distinguishes "value" from "no value" — `Result` also says "why there's no value."
2. **`match` for exhaustive handling:** A `match` on `Result` must handle both `Ok(v)` and `Err(e)` — the compiler enforces this. The error is not silently ignored.
3. **`expect(msg)` for context:** `result.expect("context message")` panics with a descriptive message if the result is `Err`. Prefer `expect` over `unwrap` so panics include useful context.
4. **OCaml `result` type:** `type ('a, 'b) result = Ok of 'a | Error of 'b`. Usage identical to Rust. OCaml 4.03+ has `Ok` and `Error` constructors in scope by default.

## Exercises

1. **Integer parse**: Write `parse_positive(s: &str) -> Result<u32, String>` that parses a string to a positive integer, returning descriptive error messages for non-integer and negative inputs.
2. **Result combination**: Write `add_results(a: Result<i32, String>, b: Result<i32, String>) -> Result<i32, String>` that adds the values if both are `Ok`, or returns the first error.
3. **Collect results**: Write `parse_all(ss: &[&str]) -> Result<Vec<i32>, String>` that parses all strings, returning `Err` with the first parse failure. Use `.collect::<Result<Vec<_>, _>>()`.

4. **Chain of validations**: Write `validate_age(input: &str) -> Result<u8, String>` that parses a string to a number and checks it's between 0 and 150, returning descriptive errors for each failure mode.
5. **Result to Option**: Write `ok(result: Result<T, E>) -> Option<T>` that discards the error and returns `Some(v)` or `None`. Then write `err(result: Result<T, E>) -> Option<E>` for the error side.
