📖 **[View on hightechmind.io →](https://hightechmind.io/rust/311-multiple-error-types)**

---

# 311: Handling Multiple Error Types

**Difficulty:** 3  **Level:** Advanced

Unify errors from different sources so `?` works across an entire module.

## The Problem This Solves

A real application function might call the filesystem, a database, a parser, and an HTTP client — each returning its own error type. Without a unification strategy, you can't use `?` across all of them in a single function. You're stuck with nested `match` expressions or a proliferation of `map_err` calls at every site.

The standard solution is a unified error enum with one variant per source, and `impl From<SourceError> for AppError` for each variant. Once those `From` impls exist, `?` handles all conversions automatically. Your function body reads cleanly — just the happy path logic — and the error handling is centralized in the enum definition.

This pattern scales. You start with three variants. When you add a new dependency, you add one variant and one `From` impl. The call sites don't change. The function signatures stay readable. This is why virtually every non-trivial Rust application uses this pattern.

## The Intuition

One unified error enum + one `From` impl per source error type = `?` works everywhere in your module.

## How It Works in Rust

```rust
use std::num::ParseIntError;

// Step 1: One enum to rule them all
#[derive(Debug)]
enum AppError {
    Io(IoError),
    Db(DbError),
    Parse(ParseIntError),
}

impl fmt::Display for AppError { /* match on variants */ }

// Step 2: From impl for each source — this is what enables ?
impl From<IoError> for AppError {
    fn from(e: IoError) -> Self { AppError::Io(e) }
}
impl From<DbError> for AppError {
    fn from(e: DbError) -> Self { AppError::Db(e) }
}
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self { AppError::Parse(e) }
}

// Step 3: Use ? freely — no map_err anywhere
fn pipeline(path: &str) -> Result<Vec<i32>, AppError> {
    let content = read_file(path)?;    // IoError -> AppError::Io via From
    let n: i32 = content.parse()?;    // ParseIntError -> AppError::Parse via From
    let rows = query_db(n)?;           // DbError -> AppError::Db via From
    Ok(rows)
}

// Step 4: Match on specific variants at the boundary
match pipeline("data.txt") {
    Ok(rows) => process(rows),
    Err(AppError::Io(e)) => eprintln!("File error: {}", e),
    Err(AppError::Db(e)) => eprintln!("DB error: {}", e),
    Err(AppError::Parse(e)) => eprintln!("Bad data: {}", e),
}
```

The tradeoff vs `Box<dyn Error>`: the enum approach preserves variant information (you can match on it), while `Box<dyn Error>` erases it. Use the enum for libraries and anywhere callers need to distinguish error types; use the box for application-level propagation where you just log and exit.

## What This Unlocks

- **Clean function bodies** — no `map_err` noise; every `?` just works across all error types in the module
- **Matchable errors** — callers can pattern-match on `AppError::Io` vs `AppError::Db` to handle each case differently
- **Scalable pattern** — adding a new error source requires one new variant and one new `From` impl — call sites unchanged

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Multiple errors | Polymorphic variant union | Enum with one variant per source |
| Conversion at `?` | Explicit `map_error` | `From` impl — compiler inserts the call |
| Type safety | High — variants are typed | High — each variant wraps its specific type |
| vs. Box<dyn Error> | N/A | Enum = matchable; Box = type-erased; use enum for libs |
