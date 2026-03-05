# Custom Error Types — Comparison

## Core Insight
OCaml exceptions are dynamic and bypass the type checker; Rust errors are typed enums that the compiler tracks through `Result<T, E>`.

## OCaml Approach
- `exception` declarations create runtime-only error types
- Exceptions don't appear in function signatures
- Callers have no compile-time indication a function can fail
- Polymorphic variants offer a typed alternative but lack the `Error` trait ecosystem

## Rust Approach
- Error types are regular enums implementing `Display` and `Error`
- `Result<T, E>` in the return type makes fallibility explicit
- Pattern matching on error variants is exhaustive
- The `Error` trait enables interop with `Box<dyn Error>` and error-handling crates

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Error declaration | `exception Foo of string` | `enum MyError { Foo(String) }` |
| Type visibility | Not in signature | In `Result<T, E>` return type |
| Pattern matching | `try ... with` | `match result { Ok/Err }` |
| Exhaustiveness | No (catch-all needed) | Yes (compiler enforced) |
| Display | Manual `string_of_*` | `impl Display` trait |
| Composability | Limited | `Error` trait + `From` + `?` |
