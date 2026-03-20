📖 **[View on hightechmind.io →](https://hightechmind.io/rust/581-pattern-result-ok-err)**

---

# Result Pattern Matching
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Error handling is one of the most consequential design decisions in a language. C uses integer return codes (frequently ignored). Java uses exceptions (can be swallowed silently). Rust uses `Result<T, E>` — a value that is either `Ok(T)` or `Err(E)`, with the compiler enforcing that error paths are handled. The `?` operator makes error propagation as concise as exceptions while keeping the error path explicit in function signatures. OCaml's `result` type serves the same purpose and is the direct ancestor of Rust's `Result`.

## Learning Outcomes

- How `match res { Ok(v) => ..., Err(e) => ... }` handles both success and failure
- How `?` propagates errors up the call stack in `fn -> Result<_, E>` functions
- How custom error enums express all possible failure modes for a function
- How `map`, `and_then`, `map_err` transform `Result` values without nested `match`
- Where `Result` replaces exceptions: I/O, parsing, validation, network operations

## Rust Application

`MyError` enum has `Parse(String)`, `Range(i32)`, `DivZero` variants. Functions return `Result<i32, MyError>`. The `?` operator on a function returning `Result` — if the sub-result is `Err`, it returns early with `Err`. `map_err` converts between error types. `and_then` chains fallible operations. `match res { Ok(v) => ..., Err(MyError::Parse(s)) => ..., Err(MyError::Range(n)) => ... }` handles specific error variants.

Key patterns:
- `res?` — early return on `Err`, propagate error
- `res.map(|v| transform(v))` — transform success value
- `res.and_then(|v| another_result(v))` — chain fallible
- `res.map_err(|e| convert_error(e))` — convert error type

## OCaml Approach

OCaml's `result` type is the same concept:

```ocaml
type error = Parse of string | Range of int | DivZero
let (>>=) r f = match r with Ok x -> f x | Error e -> Error e
let parse_and_divide s n =
  match int_of_string_opt s with
  | None -> Error (Parse s)
  | Some v -> if n = 0 then Error DivZero else Ok (v / n)
```

## Key Differences

1. **`?` operator**: Rust's `?` is syntactic sugar for `match res { Ok(v) => v, Err(e) => return Err(e.into()) }`; OCaml uses `let*` or explicit bind (`>>=`).
2. **`From` trait**: Rust's `?` automatically converts error types via `From` trait implementations; OCaml requires explicit `map_err` or pattern matching for type conversion.
3. **`std::error::Error`**: Rust has a standard `Error` trait for composable error types; OCaml uses ad-hoc `exn` or custom result types.
4. **`anyhow`/`thiserror`**: Rust's ecosystem has `anyhow` for application-level errors and `thiserror` for library errors; OCaml has no direct equivalent.

## Exercises

1. **Error chain**: Write `fn parse_config(s: &str) -> Result<Config, MyError>` that parses a "key=value" string using `?` to propagate parse errors and a guard for invalid keys.
2. **Error conversion**: Add `impl From<ParseIntError> for MyError` and verify that `?` on a `str::parse::<i32>()` now automatically converts the error type.
3. **Result combinator chain**: Implement `fn process(input: &str) -> Result<String, MyError>` using only `map`, `and_then`, and `map_err` without any `match` or `if let`.
