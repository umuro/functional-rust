📖 **[View on hightechmind.io →](https://hightechmind.io/rust/318-error-display-debug)**

---

# 318: Display vs Debug for Errors
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Error messages have two audiences: end users who need human-readable descriptions ("Cannot connect to server"), and developers who need complete diagnostic information including internal state (field names, codes, stack frames). Rust encodes this distinction in two traits: `Display` for user-facing messages, `Debug` for developer diagnostics. Both are required by `std::error::Error`, and using them correctly separates user experience from debugging information.

## Learning Outcomes

- Understand `Display` as user-facing output (error messages shown to end users)
- Understand `Debug` as developer-facing output (detailed diagnostic information)
- Implement both traits on the same error type for different audiences
- Recognize that `{:?}` uses `Debug`, `{}` uses `Display` in format strings

## Rust Application

Same error type, two different outputs:

```rust
#[derive(Debug)]  // auto-derives detailed struct/enum representation
pub enum DbError {
    ConnectionFailed(String),
    QueryTimeout(f64),
    NotFound(String),
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionFailed(h) => write!(f, "Cannot connect to {h}"),
            Self::QueryTimeout(s) => write!(f, "Query timed out after {s:.1}s"),
            Self::NotFound(k) => write!(f, "Record not found: {k}"),
        }
    }
}

// User sees: "Cannot connect to db.example.com"
// Developer sees: DbError::ConnectionFailed("db.example.com")
```

## OCaml Approach

OCaml distinguishes `pp` (pretty-printer for structured output) from `to_string` (human-readable). `ppx_sexp_conv` auto-derives structured output similar to `#[derive(Debug)]`:

```ocaml
(* ppx_sexp_conv generates structured output like Debug *)
[@@deriving sexp_of]

(* Manual display function like Display *)
let to_user_string = function
  | ConnectionFailed h -> Printf.sprintf "Cannot connect to %s" h
  | QueryTimeout s -> Printf.sprintf "Query timed out after %.1fs" s
```

## Key Differences

1. **Two required traits**: Rust's `std::error::Error` requires both `Display` and `Debug`; OCaml has no such requirement — any type can be an error.
2. **Auto-derive Debug**: `#[derive(Debug)]` generates a complete structural representation; implementing it manually is rarely needed.
3. **Error propagation format**: `eprintln!("Error: {}", e)` shows user message; `eprintln!("Debug: {:?}", e)` shows developer details.
4. **Testing**: Use `assert_eq!(format!("{}", err), "expected user message")` to test `Display`; use `{:?}` for debugging assertions.

## Exercises

1. Implement `Display` for an error type that produces a one-line user message, and verify that `format!("{}", err)` produces the expected output.
2. Show the difference between `format!("{}", err)` and `format!("{:?}", err)` output for the same `DbError` value.
3. Write a test that verifies both `Display` and `Debug` outputs meet their respective format expectations for all variants.
