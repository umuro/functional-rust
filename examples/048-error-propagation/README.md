📖 **[View on hightechmind.io →](https://hightechmind.io/rust/048-error-propagation)**

---

# 048 — Error Propagation with the ? Operator

## Problem Statement

The `?` operator is Rust's ergonomic solution to error propagation. Without it, every fallible call requires an explicit `match` or `unwrap`. With `?`, errors bubble up automatically: `let x = fallible_op()?;` returns the error early if the operation fails, otherwise binds `x` to the success value. This transforms nested error handling into sequential, readable code.

Error propagation appears in every real application: reading a config file (may not exist), parsing its contents (may be malformed), connecting to a database (may fail), executing a query (may fail). The `?` operator makes each step explicit without repetitive boilerplate.

## Learning Outcomes

- Use `?` to propagate errors in functions returning `Result<T, E>`
- Understand what `?` desugars to: `match result { Ok(v) => v, Err(e) => return Err(e.into()) }`
- Combine `?` with `map`, `and_then` for complex pipelines
- Understand when to use `?` vs explicit `match` (when you need to handle the error)
- See how `From` trait conversion enables `?` to automatically convert error types

- Use `?` operator to propagate errors: `let x = fallible()?` returns `Err(e)` if the operation failed
- Understand `?` requires the enclosing function to return `Result` and the error type to implement `From<E>`

## Rust Application

A function `fn load_config(path: &str) -> Result<Config, AppError>` uses `?` on each step: `let contents = std::fs::read_to_string(path)?;` — returns `Err(AppError::from(io_error))` if the file cannot be read. `let config = parse_toml(&contents)?;` — returns on parse error. The function body reads like sequential code; error propagation is implicit. The `From` trait must be implemented (or derived) for automatic error type conversion.

## OCaml Approach

OCaml without ppx_let requires explicit `match`: `match read_file path with | Error e -> Error e | Ok contents -> match parse_toml contents with | Error e -> Error e | Ok config -> Ok config`. With `let*` (ppx_let): `let* contents = read_file path in let* config = parse_toml contents in Ok config`. This is equivalent to Rust's `?` operator in readability.

## Key Differences

1. **Built-in operator**: Rust's `?` is part of the language (since 1.13). OCaml's `let*` requires a PPX extension. Rust code can use `?` without any imports; OCaml needs setup.
2. **`From` conversion**: Rust's `?` automatically calls `From::from(e)` to convert error types. OCaml's `let*` with `Result.bind` uses the error type directly — no automatic conversion.
3. **`Option` support**: Rust's `?` also works in functions returning `Option<T>`. OCaml's `let*` can work with both `option` and `result` by defining appropriate `let*` operators.
4. **Explicit vs implicit**: OCaml's explicit `match` propagation is verbose but visible. Rust's `?` is concise but requires knowing that it can cause early returns — this matters for reasoning about control flow.

1. **`?` vs `and_then`:** Both propagate errors. `?` is used in functions that return `Result` — it's syntactic sugar that reads like imperative code. `and_then` is a method on `Result` values — more functional style.
2. **Return type must be `Result`:** `?` can only be used inside a function that returns `Result<_, E>` (or `Option<_>`). Using `?` in `main()` requires `fn main() -> Result<(), Box<dyn Error>>`.
3. **Error type compatibility:** For `?` to work, the error type of the called function must be convertible to the error type of the enclosing function (via `From` trait). This is why error libraries like `thiserror` and `anyhow` are popular.
4. **OCaml exceptions vs Result:** OCaml has both exceptions and the `Result` type. Idiomatic OCaml error handling has shifted toward `Result` with `let*`, but legacy code uses exceptions extensively.

## Exercises

1. **Multi-file**: Write `read_and_merge(path1: &str, path2: &str) -> Result<String, std::io::Error>` that reads two files and concatenates them. Use `?` for both file reads.
2. **Parse pipeline**: Write `load_int_from_file(path: &str) -> Result<i32, String>` that reads a file, trims whitespace, and parses as integer. Convert each error to `String` using `map_err` before `?`.
3. **Rewrite without `?`**: Take a function using `?` and rewrite it using explicit `match` statements. Count the additional lines. Then rewrite using `and_then` chains. Compare all three.

4. **Custom `?` conversion**: Create a custom error type `AppError` and implement `From<ParseIntError>` and `From<std::io::Error>` so that both types can be propagated with `?` in the same function.
5. **try_main**: Write `fn main() -> Result<(), Box<dyn std::error::Error>>` that reads a filename from argv, opens the file, reads the first line, parses it as an integer, and prints it — using `?` throughout.
