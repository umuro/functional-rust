📖 **[View on hightechmind.io →](https://hightechmind.io/rust/296-from-trait-errors)**

---

# 296: From Trait for Error Conversion

**Difficulty:** 2  **Level:** Intermediate

Make `?` automatically convert between error types — no manual `map_err` needed.

## The Problem This Solves

You're calling three functions that each return a different error type: `ParseIntError`, `io::Error`, and your own `DbError`. To propagate them all with `?`, you'd normally write a `map_err` at every call site — wrapping each error into your unified type by hand. Ten call sites means ten `map_err` chains.

The `From` trait eliminates all of that. When you implement `From<ParseIntError> for AppError`, the `?` operator sees it and calls the conversion automatically. Your call sites stay clean.

This is how the Rust standard library is designed to be used. Any real application has a unified error enum at the top and `From` impls for each source error type. Once that boilerplate exists, every function in the module can use `?` freely regardless of which subsystem it touches.

## The Intuition

`impl From<SpecificError> for MyError` teaches `?` how to convert that error type — automatically, at every call site.

## How It Works in Rust

```rust
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    Parse(ParseIntError),
    Logic(String),
}

// Step 1: impl From for each source error type
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)  // wrap it in the right variant
    }
}

// Step 2: ? calls From::from() automatically when the types don't match
fn process(input: &str) -> Result<i32, AppError> {
    let n: i32 = input.parse()?;  // ParseIntError -> AppError via From, no map_err needed
    if n <= 0 { return Err(AppError::Logic(format!("{} is not positive", n))); }
    Ok(n * 2)
}
```

The `?` operator desugars to something like:
```rust
match result {
    Ok(v) => v,
    Err(e) => return Err(AppError::from(e)),  // calls your From impl
}
```

The compiler selects the correct `From` impl based on the error type at each `?` site.

## What This Unlocks

- **Clean function bodies** — no `map_err` noise; every `?` just works, regardless of the source error
- **Unified error type for a module** — one enum captures all failure modes; callers match on variants
- **Composable pipelines** — mix calls to the filesystem, parser, and database in one function without manual wrapping

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Error conversion | `Result.map_error` at every call site | `impl From<E> for MyErr` once, then `?` everywhere |
| How it triggers | Explicit, manual | Implicit — `?` calls `From::from()` |
| Type inference | N/A | Compiler selects the right `From` impl per call site |
| Boilerplate location | Scattered at call sites | Centralized in one `From` impl |
