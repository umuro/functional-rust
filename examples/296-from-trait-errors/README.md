📖 **[View on hightechmind.io →](https://hightechmind.io/rust/296-from-trait-errors)**

---

# 296: From Trait for Error Conversion
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Functions calling multiple libraries encounter multiple error types. Unifying them into a single application error type with `match` on every `?` usage is boilerplate. The `From<SourceError>` trait enables automatic conversion: when `impl From<ParseIntError> for AppError` is defined, the `?` operator automatically calls `AppError::from(e)` when propagating a `ParseIntError`. This is how Rust achieves zero-boilerplate error type unification.

## Learning Outcomes

- Understand that `impl From<E> for AppError` enables automatic `?` conversion from `E`
- Implement `From` for each error type a function might encounter
- Recognize the `?` desugaring as `Err(AppError::from(e))`
- Use `From` to build layered error hierarchies without explicit mapping at each call site

## Rust Application

Each `From` implementation unlocks one error source for `?` in functions returning `Result<_, AppError>`:

```rust
#[derive(Debug)]
pub enum AppError {
    Parse(ParseIntError),
    Logic(String),
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::Parse(e) }
}

// Now ? works automatically:
fn process(s: &str) -> Result<i32, AppError> {
    let n: i32 = s.parse()?;  // ParseIntError -> AppError::Parse automatically
    if n < 0 { return Err(AppError::Logic("negative".to_string())); }
    Ok(n * 2)
}
```

## OCaml Approach

OCaml does not have automatic error conversion. Each call site must explicitly wrap errors:

```ocaml
let process s =
  match int_of_string_opt s with
  | None -> Error (`Parse "not a number")
  | Some n ->
    if n < 0 then Error (`Logic "negative")
    else Ok (n * 2)
```

Libraries like `Error_monad` (from Tezos) provide richer error composition, but there is no standard mechanism for automatic conversion.

## Key Differences

1. **Automatic conversion**: Rust's `?` calls `From::from()` at every error propagation point — zero boilerplate after the `impl From`; OCaml requires manual wrapping.
2. **Type hierarchy**: `From` implementations create a directed conversion graph; adding a new library error requires one new `impl From`.
3. **Conflict prevention**: Only one `From<E>` impl per target type is allowed — prevents ambiguous conversions.
4. **`Into` derived**: `impl From<A> for B` automatically provides `impl Into<B> for A` — both directions are covered.

## Exercises

1. Define an `AppError` with four variants and implement `From` for each of: `std::io::Error`, `std::num::ParseIntError`, `serde_json::Error` (simulated), and `String`.
2. Show that without a `From` impl, the `?` operator fails to compile, then add the impl and verify it compiles.
3. Implement a function that calls three different libraries and propagates all their errors through a single `AppError` using `?` without any explicit `map_err`.
