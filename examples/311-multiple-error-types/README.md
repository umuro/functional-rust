📖 **[View on hightechmind.io →](https://hightechmind.io/rust/311-multiple-error-types)**

---

# 311: Handling Multiple Error Types
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Real functions call multiple operations that each have different error types: parsing, I/O, database queries. Returning `Result<T, String>` loses precision; returning a massive union type is unwieldy. The standard approach is a custom error enum with one variant per error source, and `impl From<SourceError>` for each — enabling `?` to automatically wrap errors at each call site. This is the pattern `thiserror` automates.

## Learning Outcomes

- Define an application error enum that unifies multiple library error types
- Implement `From<E>` for each error source to enable `?` conversion
- Use the `?` operator with multiple error types in a single function body
- Recognize this as the "error type tower" pattern used in production Rust code

## Rust Application

One enum variant per error source, one `From` impl per variant:

```rust
#[derive(Debug)]
pub enum AppError {
    Parse(ParseIntError),
    Io(IoError),
    Db(DbError),
}

impl From<ParseIntError> for AppError { fn from(e: ParseIntError) -> Self { AppError::Parse(e) } }
impl From<IoError> for AppError { fn from(e: IoError) -> Self { AppError::Io(e) } }
impl From<DbError> for AppError { fn from(e: DbError) -> Self { AppError::Db(e) } }

fn process(s: &str) -> Result<(), AppError> {
    let n: i32 = s.parse()?;    // ParseIntError -> AppError::Parse
    read_file()?;                // IoError -> AppError::Io
    write_db(n)?;                // DbError -> AppError::Db
    Ok(())
}
```

## OCaml Approach

OCaml uses polymorphic variants for extensible error types, allowing different error families to be mixed without a common super-enum:

```ocaml
type app_error = [`Parse of string | `Io of string | `Db of string]

let process s : (unit, app_error) result =
  let* n = int_of_string_opt s |> Option.to_result ~none:(`Parse "not a number") in
  let* () = read_file () |> Result.map_error (fun e -> `Io e) in
  write_db n |> Result.map_error (fun e -> `Db e)
```

## Key Differences

1. **Nominal vs structural**: Rust uses nominal enum variants (closed); OCaml's polymorphic variants are structural (open) — you can add new variants without changing the union type.
2. **From automation**: Rust's `?` calls `From::from()` automatically; OCaml requires explicit `Result.map_error` at each site.
3. **Exhaustive matching**: Both require exhaustive match on the error type — adding a variant is a breaking change (Rust) or extends the union (OCaml polymorphic variants).
4. **thiserror**: `#[derive(thiserror::Error)]` with `#[from]` attributes generates all the `From` impls, making the manual boilerplate optional.

## Exercises

1. Add a fourth error variant to an existing `AppError` enum, implement its `From` conversion, and use it in a new function.
2. Implement the same three-error function using `Box<dyn Error>` instead of a custom enum — compare the tradeoffs.
3. Write a test that matches on each variant of `AppError` to verify that error type conversion is correct for each source.
