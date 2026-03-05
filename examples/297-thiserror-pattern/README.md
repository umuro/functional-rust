📖 **[View on hightechmind.io →](https://hightechmind.io/rust/297-thiserror-pattern)**

---

# 297: The thiserror Pattern

**Difficulty:** 3  **Level:** Advanced

Eliminate error boilerplate with derive macros — understand what gets generated.

## The Problem This Solves

Every production error type in Rust requires the same boilerplate: a `Display` impl, an `Error` impl, and `From` impls for each wrapped variant. For an enum with five variants, that's 30+ lines of mechanical code that adds noise without adding meaning. It's the same pattern every single time.

The `thiserror` crate condenses this to a handful of annotations. `#[error("connection to '{host}' failed")]` generates the entire `Display` impl. `#[from]` generates the `From` impl. What was 30 lines becomes 5. But more importantly: the intent is visible right next to the type declaration — you don't have to hunt through impl blocks to understand what each variant displays as.

This example shows the full manual implementation *as if* you were writing what `thiserror` generates. Understanding the shape of the generated code is essential — it's what you'll read in stack traces, it's what gets compiled, and it's what you need to write when you're working in a no-std environment or can't add the crate.

## The Intuition

`thiserror`'s derive macro generates `Display`, `Error`, and `From` impls from annotations — what was 30 lines of boilerplate becomes 5 lines of intent.

## How It Works in Rust

```rust
// With thiserror (what you'd write in production):
// #[derive(thiserror::Error, Debug)]
// pub enum DbError {
//     #[error("connection to '{host}' failed")]
//     ConnectionFailed { host: String },
//     #[error("query failed: {0}")]
//     QueryFailed(String),
//     #[error(transparent)]  // delegate Display AND source() to the wrapped error
//     Io(#[from] std::io::Error),
// }

// Without thiserror — the manual equivalent:
#[derive(Debug)]
pub enum DbError {
    ConnectionFailed { host: String },
    QueryFailed(String),
}

impl fmt::Display for DbError {  // what #[error("...")] generates
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::ConnectionFailed { host } =>
                write!(f, "connection to '{}' failed", host),
            DbError::QueryFailed(sql) =>
                write!(f, "query failed: {}", sql),
        }
    }
}

impl Error for DbError {}  // what #[derive(thiserror::Error)] generates

// What #[from] on a variant generates:
impl From<DbError> for AppError {
    fn from(e: DbError) -> Self { AppError::Db(e) }
}
```

The `source()` method is also generated automatically when a variant holds a `#[source]` or `#[from]` field — it returns `Some(&inner_error)` for error chaining.

## What This Unlocks

- **Readable error enums** — error messages live next to their variant definitions, not in distant impl blocks
- **Automatic `From` conversion** — `#[from]` on a field generates the `impl From<>` and marks it as `source()`
- **Zero-cost** — everything is compile-time; the generated code is identical to hand-written

## Key Differences

| Concept | OCaml | Rust (manual) | Rust (thiserror) |
|---------|-------|---------------|-----------------|
| Error display | Format string in handler | `impl Display` block | `#[error("...")]` annotation |
| Wrapping errors | Manual constructor | `impl From<E>` | `#[from]` field attribute |
| Source chain | Manual field access | `fn source()` impl | Auto-generated from `#[from]` |
| Boilerplate | Minimal — no trait | ~30 lines per error type | ~5 lines per error type |
