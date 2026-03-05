# Error Chaining — Comparison

## Core Insight
Both languages can wrap errors with context, but Rust's `map_err` and extension traits make it idiomatic and composable at the type level.

## OCaml Approach
- Wrap errors in records or variant constructors manually
- Can write `with_context` helpers that pattern-match on `Result`
- No standard trait system for error chaining
- Context is ad-hoc — each project invents its own pattern

## Rust Approach
- `map_err(|e| ...)` transforms errors inline during propagation
- Extension traits like `WithContext` mimic what `anyhow` provides
- The `Error::source()` method creates a standard chain
- Pattern is so common that crates like `anyhow` and `thiserror` standardize it

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Context addition | Manual record wrapping | `map_err` / extension trait |
| Inline ergonomics | Verbose match | Fluent `.map_err(...)` |
| Error chain | Custom nesting | `Error::source()` standard |
| Ecosystem support | Ad-hoc | `anyhow`, `thiserror` crates |
| Lazy context | Closure in helper | `FnOnce` in extension trait |
