📖 **[View on hightechmind.io →](https://hightechmind.io/rust/407-default-trait)**

---

# 407: The Default Trait
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Large configuration structs become painful to construct when callers must specify every field. Languages with named parameters or optional fields handle this naturally, but Rust requires all fields to be specified in struct literals. The `Default` trait solves this: implementing `Default` for a struct lets callers use `..Default::default()` to fill in unspecified fields, and the struct update syntax to customize only what differs from the default. This is the idiomatic Rust approach to optional constructor parameters.

`Default` appears everywhere: `HashMap::new()` uses `Default::default()` internally, `Vec::new()` returns an empty vec (the default), and derive macros require `Default` for many generated methods.

## Learning Outcomes

- Understand the `Default` trait as the standard way to create "empty" or "starter" values
- Learn the difference between derived `Default` (zeros/empty) and custom `Default` implementations
- See the struct update syntax `..Default::default()` for partial initialization
- Understand how `Default` enables the builder pattern and `#[derive(Default)]` on config types
- Learn which standard types implement `Default` and what they return

## Rust Application

In `src/lib.rs`, `ServerConfig` derives `Default` — getting empty string, 0 port, 0 connections, false, and 0.0 timeout. `AppConfig` implements `Default` manually with sensible defaults: "localhost", port 8080, 100 connections, 30 second timeout. The `with_port` constructor uses `..Default::default()` to copy all defaults except port. A `HashMap<String, AppConfig>` demonstrates `Default` integration with collections.

## OCaml Approach

OCaml achieves default values through optional parameters with `~` and `?` syntax: `let make_config ?(host="localhost") ?(port=8080) () = { host; port }`. This is more flexible than Rust's `Default` since each field can have an independent default without a special trait. OCaml's named optional arguments eliminate the need for a builder pattern or `Default` trait entirely in most cases.

## Key Differences

1. **Mechanism**: Rust uses a trait with a single `default()` method; OCaml uses optional function parameters — fundamentally different approaches.
2. **Granularity**: Rust's `Default` returns the entire struct; OCaml's optional params default each field independently at the call site.
3. **Struct update**: Rust's `..Default::default()` copies all remaining fields; OCaml has no equivalent (you specify each field).
4. **Derive**: Rust's `#[derive(Default)]` works for any struct where all fields implement `Default`; OCaml's `deriving` requires a ppx extension.

## Exercises

1. **HTTP request builder**: Create `HttpRequest { method: String, url: String, headers: Vec<(String, String)>, body: Option<Vec<u8>>, timeout: Duration }` with a custom `Default` (GET, empty url, no headers, no body, 30s timeout). Show construction with `..Default::default()`.
2. **Nested defaults**: Create a nested config `DatabaseConfig` with `Default` and use it as a field in `AppConfig`. Show that `AppConfig::default()` initializes the nested struct correctly.
3. **Default impl for custom types**: Implement `Default` for a `Matrix<f64>` type that returns a 3x3 identity matrix. Explain in a comment why the identity matrix is the appropriate default for numeric matrix operations.
