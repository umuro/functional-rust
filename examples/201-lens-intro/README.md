📖 **[View on hightechmind.io →](https://hightechmind.io/rust/201-lens-intro)**

---

# Introduction to Lenses — The Nested Update Problem

## Problem Statement

Updating a deeply nested field in an immutable data structure requires manually reconstructing every level of nesting: `AppConfig { db: DbConfig { host: new_host, ..config.db }, ..config }`. For three levels of nesting this is already verbose; for five levels it is unmaintainable. Lenses solve this by encapsulating the path to a field as a first-class value that can be composed, passed, and applied generically. They are fundamental to functional programming with immutable data.

## Learning Outcomes

- Understand why deeply nested immutable updates are problematic without lenses
- Learn what a Lens is: a composable pair of `get` (read) and `set` (write) functions
- See how lens composition allows updating deeply nested fields with one call
- Appreciate the real-world motivation: configuration management, state management, ORMs

## Rust Application

The code demonstrates the problem first: updating `config.app.server.db.host` requires reconstructing all three levels by hand. Then it shows the lens solution: composing `db_lens` and `host_lens` gives a `db_host_lens` that reads and writes the host field directly. `over(lens, f, config)` applies a function to the focused field and returns a new config with only that field changed — all intermediate levels are rebuilt automatically by the lens.

## OCaml Approach

OCaml's optics libraries (`optics`, `lens`) provide the same abstraction. The Haskell tradition of `data-lens` influenced both languages. OCaml's record update syntax `{ r with field = value }` handles one level natively. For deeper nesting, lenses are essential. OCaml's functorial approach wraps lenses in modules; the `ppx_lens` preprocessor generates lenses automatically from type definitions.

## Key Differences

1. **Derive macros**: Rust crates like `lens-rs` provide `#[derive(Lens)]` to auto-generate lenses; OCaml's `ppx_lens` does the same — both reduce boilerplate.
2. **Record syntax**: OCaml's `{ r with field = v }` updates one level in-place syntactically; Rust's `StructName { field: v, ..r }` is similar but slightly more verbose.
3. **Composability**: Lens composition `(|>>)` in OCaml and `.compose()` in Rust are equivalent; both produce lenses that traverse multiple levels in one shot.
4. **Practical use**: Facebook's `Recoil`, Redux's selectors, and game engine component systems all use lens-like patterns.

## Exercises

1. Write a `over` function that applies `f` to the field focused by a lens and returns the updated structure.
2. Compose three lenses to focus on `config.server.db.port` and increment it.
3. Implement `modify_all(lens, f, configs: Vec<Config>) -> Vec<Config>` using `map` + `over`.
