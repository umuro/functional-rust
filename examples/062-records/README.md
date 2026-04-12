📖 **[View on hightechmind.io →](https://hightechmind.io/rust/062-records)**

---

# 062 — Records (Structs)
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Records (called structs in Rust) are the product type of a type system — a value that bundles multiple named fields. OCaml's `type point = { x: float; y: float }` and Rust's `struct Point { x: f64, y: f64 }` are direct equivalents. Records are the foundation for representing real-world entities: users, configurations, geometric shapes, network requests.

The record update syntax — creating a new record with most fields from an existing one, changing only a few — is a functional programming staple. It appears in immutable state management (Redux reducers, Elm architecture), configuration management, and "builder" patterns.

## Learning Outcomes

- Define structs with named fields and implement methods via `impl`
- Use struct update syntax `Config { debug: true, ..Config::default_config() }` for partial updates
- Derive `Debug`, `Clone`, `Copy` for common struct utilities
- Understand when to use `Copy` (small, stack-allocated values) vs `Clone` (heap-allocated)
- Pattern-match on struct fields using destructuring

- Define Rust `struct` as the equivalent of OCaml records with named fields and implement methods in an `impl` block
- Use `#[derive(Debug, Clone, PartialEq)]` to auto-generate common trait implementations without boilerplate

## Rust Application

`Point { x: f64, y: f64 }` with a `distance` method. `Config` demonstrates update syntax: `Config { debug: true, port: 3000, ..Config::default_config() }` creates a new `Config` with `debug` and `port` overridden and everything else from the default. The `#[derive(Copy)]` on `Point` makes it cheap to copy (stack-only). `Config` derives `Clone` but not `Copy` because it contains a `String`.

## OCaml Approach

OCaml record: `type point = { x: float; y: float }`. Record creation: `{ x = 1.0; y = 2.0 }`. Update syntax: `{ config with debug = true; port = 3000 }` — directly parallel to Rust's `..config`. Pattern matching: `let { x; y } = point in ...`. OCaml records are immutable by default; mutable fields use `mutable x: float`.

## Key Differences

1. **Update syntax**: OCaml: `{ record with field = value }`. Rust: `StructName { field: value, ..record }`. Both create a new record with specified fields overridden.
2. **Mutability**: OCaml records are immutable by default; add `mutable` per field. Rust structs are immutable by default; `let mut s = Struct {...}` makes the entire binding mutable.
3. **Methods**: Rust methods are defined in `impl` blocks separate from the struct. OCaml has module-level functions; methods are a convention, not a language feature.
4. **`Copy` trait**: Rust's `Copy` trait marks types that can be copied by value on assignment (stack-only types). OCaml's uniform representation means all values are either boxed (heap) or unboxed (stack) based on size, without explicit marking.

1. **Struct vs record:** OCaml's `type point = { x: float; y: float }` and Rust's `struct Point { x: f64, y: f64 }` are isomorphic. Both are product types with named fields.
2. **Functional update:** OCaml's record update `{ p with x = 1.0 }` creates a new record with one field changed. Rust has no built-in equivalent — use `Point { x: 1.0, ..p }` (struct update syntax).
3. **Deriving traits:** `#[derive(Debug, Clone, PartialEq)]` in Rust auto-generates common impls. OCaml uses `[@@deriving show, eq]` (ppx_deriving) for the same effect.
4. **Destructuring:** `let Point { x, y } = point` in Rust extracts fields by name. OCaml: `let { x; y } = point`. Both support nested destructuring in patterns.

## Exercises

1. **Builder pattern**: Write a `ConfigBuilder` struct with setter methods that each return `Self` (for chaining) and a `build() -> Config` method. This is idiomatic Rust for structs with many optional fields.
2. **Serde serialization**: Add `#[derive(serde::Serialize, serde::Deserialize)]` to `Config` and serialize/deserialize to/from JSON using `serde_json`.
3. **Default trait**: Implement `Default` for `Config` using `#[derive(Default)]` (set all fields to their defaults) or a manual `impl Default`. Compare with the manual `default_config()` function.

4. **Builder pattern**: Implement a builder for a `Config` struct with many optional fields, using the builder pattern (a struct `ConfigBuilder` with a chain of setters and a final `.build() -> Result<Config, String>`).
5. **Record lenses**: Implement getter and setter functions for each field of a `Point` struct, then compose them to update nested fields — introducing the concept of lenses without a dedicated library.
