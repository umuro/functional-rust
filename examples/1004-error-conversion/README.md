[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 1004 — Error Conversion

## Problem Statement

Implement `From<SubError> for AppError` to enable automatic error conversion with the `?` operator. When a function returns `Result<T, AppError>` and calls a sub-function returning `Result<T, ParseIntError>`, the `?` operator automatically wraps the inner error via `From::from`. Compare with OCaml's explicit manual wrapping.

## Learning Outcomes

- Implement `impl From<IoError> for AppError` and `impl From<ParseIntError> for AppError`
- Understand that `?` desugars to `map_err(From::from)` — calling the `From` impl
- Implement `Error::source` to expose the wrapped error for error chain inspection
- Chain multiple `?` calls in a single function without explicit `map_err`
- Map Rust's automatic `From`-based conversion to OCaml's manual `IoError(e)` wrapping
- Recognise the `AppError` unified error enum pattern as the idiomatic Rust design

## Rust Application

`AppError` wraps `IoError(IoError)` and `Parse(ParseIntError)`. Implementing `From<IoError> for AppError` and `From<ParseIntError> for AppError` lets callers write `parse_int_str(s)?` in a function returning `Result<_, AppError>` without a `map_err`. `Error::source` returns the wrapped inner error, enabling error chain traversal. The `?` desugaring is: `expr?` becomes `match expr { Ok(v) => v, Err(e) => return Err(AppError::from(e)) }`.

## OCaml Approach

OCaml wraps errors manually: `Error (IoError (FileNotFound path))`. There is no `?`-equivalent or automatic conversion. Functions returning `app_error Result` must explicitly tag sub-errors: `Result.map_error (fun e -> IoError e) (read_file path)`. OCaml 4.08+ provides `let*` (monadic bind) for result chaining, but conversion is still explicit.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Auto-conversion | `From` impl + `?` | Manual wrapping `IoError(e)` |
| `?` operator | `map_err(From::from)` + early return | `let*` bind + manual error lifting |
| Error chain | `Error::source()` | No standard protocol |
| Wrapper enum | `AppError::Io(e)`, `AppError::Parse(e)` | Same variant wrapping |
| From boilerplate | 5-line impl per error type | Manual `fun e -> IoError e` at each call |
| thiserror | `#[from]` attribute eliminates From | No equivalent |

The `From` + `?` pattern is one of Rust's most important ergonomic features. Writing `some_fallible_call()?` in a function returning `Result<T, AppError>` automatically converts any matching sub-error type. This enables clean, readable error propagation without noise.

## Exercises

1. Add a third sub-error `DbError(String)` to `AppError` with a `From<DbError> for AppError` impl.
2. Implement `fn process_all(items: Vec<&str>) -> Result<Vec<i32>, AppError>` that parses all items, collecting the first error.
3. Use `Result::map_err` manually to convert an `IoError` to `AppError` without the `From` impl, and compare verbosity.
4. Add `impl std::error::Error::source` chaining for three levels: `AppError` → `IoError` → `std::io::Error`.
5. In OCaml, implement `map_error : ('a -> 'b) -> ('c, 'a) result -> ('c, 'b) result` and use it to build a clean error-lifting pipeline.
