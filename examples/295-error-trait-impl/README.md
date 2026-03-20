📖 **[View on hightechmind.io →](https://hightechmind.io/rust/295-error-trait-impl)**

---

# 295: Implementing std::error::Error

## Problem Statement

The `std::error::Error` trait is the common interface for all Rust errors, enabling error chaining, dynamic dispatch, and interoperability between libraries. Implementing it properly — with `Display` for user messages, `Debug` for developer output, and `source()` for causal chains — is the foundation of production-quality error handling. This mirrors the interface that `anyhow`, `thiserror`, and the broader ecosystem expect.

## Learning Outcomes

- Implement the full `std::error::Error` trait: `Display`, `Debug`, and optionally `source()`
- Use `source()` to create linked error chains that expose root causes
- Understand the `Box<dyn Error + Send + Sync>` pattern for type-erased error storage
- Recognize `Send + Sync` bounds as requirements for using errors across thread boundaries

## Rust Application

`std::error::Error` requires only `Debug + Display`. The `source()` method is optional and defaults to `None`:

```rust
#[derive(Debug)]
pub struct ValidationError {
    pub field: String,
    pub source: Box<dyn Error + Send + Sync>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "field '{}' is invalid", self.field)
    }
}

impl Error for ValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())  // exposes the wrapped cause
    }
}
```

## OCaml Approach

OCaml does not have a standard error interface — errors are plain values. The idiomatic approach in modern OCaml uses `Result.t` with a custom error type and provides `to_string` for display. Libraries like `Fmt` provide `pp_error` conventions:

```ocaml
type error = { field: string; cause: string }

let string_of_error { field; cause } =
  Printf.sprintf "field '%s' invalid: %s" field cause
```

OCaml lacks a standard "error chaining" mechanism — nested error types or exception causes must be manually threaded.

## Key Differences

1. **Standard interface**: Rust's `std::error::Error` is the universal error contract; OCaml has no equivalent standard trait.
2. **Chaining**: Rust's `source()` creates a traversable linked list of causes; OCaml requires manual nested error structures.
3. **Thread safety**: `Box<dyn Error + Send + Sync>` enables sending errors across thread boundaries; OCaml's GC handles this transparently.
4. **Ecosystem**: `?` operator, `Box<dyn Error>`, `anyhow`, and `thiserror` all depend on `std::error::Error` as the common interface.

## Exercises

1. Implement `std::error::Error` for a three-level error chain: `IoError` wrapping `ParseError` wrapping `ValidationError`, and traverse the chain using `source()`.
2. Write a function `print_error_chain(e: &dyn Error)` that iterates through `source()` links and prints each cause on a new line.
3. Implement a custom error that wraps `std::io::Error` using `Box<dyn Error + Send + Sync>` and test that `source()` exposes the original IO error.
