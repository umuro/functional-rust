📖 **[View on hightechmind.io →](https://hightechmind.io/rust/297-thiserror-pattern)**

---

# 297: The thiserror Pattern

## Problem Statement

Implementing `Display`, `Debug`, `Error`, and `From` impls for every error type is mechanical boilerplate. The `thiserror` crate generates this boilerplate via derive macros. This example implements what `thiserror` generates manually — understanding the generated code demystifies the macro and provides a foundation for working with the pattern in production codebases where `thiserror` is a standard dependency.

## Learning Outcomes

- Understand what code `#[derive(thiserror::Error)]` generates for common patterns
- Implement error formatting with `#[error("message {field}")]` template patterns manually
- Implement `#[from]` conversions that wrap nested error types automatically
- Recognize when manual `Error` impls are needed vs when `thiserror` suffices

## Rust Application

The `DbError` enum demonstrates the manual equivalent of `thiserror`'s generated code:

```rust
// What thiserror generates for:
// #[derive(thiserror::Error, Debug)]
// pub enum DbError {
//     #[error("connection to '{host}' failed")]
//     ConnectionFailed { host: String },
//     #[error("query failed: {0}")]
//     QueryFailed(String),
// }

#[derive(Debug)]
pub enum DbError {
    ConnectionFailed { host: String },
    QueryFailed(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::ConnectionFailed { host } =>
                write!(f, "connection to '{}' failed", host),
            DbError::QueryFailed(sql) =>
                write!(f, "query failed: {}", sql),
        }
    }
}
impl std::error::Error for DbError {}
```

## OCaml Approach

OCaml uses `ppx_deriving` or plain variant types with a `to_string` or `pp` function. There is no standard equivalent to `thiserror`:

```ocaml
type db_error =
  | ConnectionFailed of { host: string }
  | QueryFailed of string

let string_of_db_error = function
  | ConnectionFailed { host } -> Printf.sprintf "connection to '%s' failed" host
  | QueryFailed sql -> Printf.sprintf "query failed: %s" sql
```

## Key Differences

1. **Boilerplate reduction**: `thiserror` eliminates repetitive `Display` impls; OCaml's `ppx_sexp_conv` or `ppx_deriving` provide similar code generation.
2. **Template syntax**: `thiserror`'s `#[error("message {field}")]` embeds formatting directly in the variant definition.
3. **Source chaining**: `#[from]` on a field generates both `From` impl and sets `source()` — two things in one annotation.
4. **Library boundary**: `thiserror` is for library errors (precise, structured); `anyhow` is for application errors (flexible, dynamic).

## Exercises

1. Manually implement what `thiserror` would generate for a `FileError` with `NotFound`, `PermissionDenied`, and `IoError(#[from] std::io::Error)` variants.
2. Compare the generated code from a simple `#[derive(thiserror::Error)]` usage with the manual implementation in this example.
3. Create a two-level error hierarchy where `AppError` wraps `DbError` and `IoError`, implementing all necessary `From` conversions manually.
