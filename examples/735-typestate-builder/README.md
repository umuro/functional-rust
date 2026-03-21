📖 **[View on hightechmind.io →](https://hightechmind.io/rust/735-typestate-builder)**

---

# 735-typestate-builder — Typestate Builder
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

The builder pattern is ubiquitous for constructing complex objects. The naive implementation calls `build()` at runtime and returns `Result<T, Error>` when required fields are missing. The typestate builder uses phantom type parameters to track which required fields have been set, making `build()` available only when all required fields are present — missing a required field becomes a compile error, not a runtime panic. This technique is used in `tokio::net::TcpStream::builder` and database query builders.

## Learning Outcomes

- Track required-field completion using phantom type parameters `Set` and `Unset`
- Implement transitions from `Unset` to `Set` by consuming the builder and returning a new type
- Restrict `build()` to only the fully-set state using a `where HasHost = Set, HasPort = Set` bound
- Understand how optional fields can be set in any order without affecting the required-field tracking
- See how this pattern eliminates an entire class of runtime configuration errors

## Rust Application

`HttpClientBuilder<HasHost, HasPort>` carries two phantom parameters. `Default` is implemented only for `HttpClientBuilder<Unset, Unset>`, providing the entry point. `impl<HasPort> HttpClientBuilder<Unset, HasPort>` exposes `host()`, which transitions `HasHost` to `Set`. The `build()` method lives in `impl HttpClientBuilder<Set, Set>` — it is unreachable unless both required fields are set. Optional fields `timeout_ms` and `max_retries` are settable from any builder state.

## OCaml Approach

OCaml achieves the same guarantee using phantom types and abstract module signatures. A builder module exposes a type `('host, 'port) builder` and marks completion with abstract type aliases. Jane Street's `ppx_fields` generates typed accessors for required/optional field separation. OCaml records with optional fields and a `validate` function are a simpler but runtime-checked alternative.

## Key Differences

1. **Type parameters**: Rust uses two explicit phantom type parameters per required field; OCaml's phantom approach uses type variables in the same position but with module-level abstraction.
2. **Ergonomics**: Rust requires verbose `PhantomData<(HasHost, HasPort)>` bookkeeping; OCaml's type inference often makes phantom variables implicit.
3. **Error message quality**: Rust's compile errors for missing required fields can be cryptic (`trait bound not satisfied`); OCaml's module signature mismatch messages are similarly opaque.
4. **Alternative**: Rust's `bon` crate generates typestate builders via derive macros, matching OCaml's `ppx_fields` in ergonomics.

## Exercises

1. Add a required `timeout_ms` field to the builder so that `build()` is only accessible after `host`, `port`, and `timeout_ms` are all set.
2. Implement a `DatabaseClientBuilder` with required `host`, `port`, `database` fields and optional `username`, `password`, `pool_size` fields.
3. Write a test that demonstrates the compile-time error when attempting to call `build()` with a missing required field — capture the error message in a comment.
