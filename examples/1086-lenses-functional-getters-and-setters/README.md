# Example 1086: Lenses — Functional Getters and Setters
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement lenses — composable, first-class getter/setter pairs that allow reading and updating deeply nested immutable data structures without boilerplate.

## Learning Outcomes

- How to encode OCaml record-of-functions as a Rust struct holding boxed closures
- Lens composition via `compose` — chaining focus through nested structures
- The `over` combinator as a functional "modify in place" on immutable data
- Lens laws (get-set, set-get, set-set) and how they guarantee correctness

## OCaml Approach

OCaml models a lens as a record with `get` and `set` fields — both are plain functions. Composition is a function that takes two lens records and returns a new one whose `get` chains inner after outer, and whose `set` threads the update back through both layers. Record-with update (`{ p with addr = a }`) makes immutable updates concise.

## Rust Approach

Rust models a lens as a struct with two `Box<dyn Fn(...)>` fields. Composition consumes both lenses and uses `Rc` to share the closure pointers between the composed getter and setter. The `over` combinator applies a transformation function through the lens. Immutable updates are done by constructing new structs — Rust has no `{ p with ... }` syntax, so fields are rebuilt explicitly.

## Key Differences

1. **Closure storage:** OCaml records hold functions directly; Rust requires `Box<dyn Fn>` for type erasure and heap allocation.
2. **Composition ownership:** OCaml freely copies closures; Rust `compose` consumes both lenses and wraps internals in `Rc` so getter and setter can share access.
3. **Immutable update syntax:** OCaml has `{ record with field = value }`; Rust reconstructs the entire struct.
4. **Clone requirements:** OCaml copies values implicitly; Rust needs explicit `.clone()` when the getter must return an owned value from a borrow.

## Exercises

1. Implement a `zoom` helper that takes a lens and a function operating on the focused value, returning an updated outer structure.
2. Create a lens for each field of a nested `Config { server: Server { host: String, port: u16 }, timeout: u64 }` struct and compose them to update the host in a single expression.
3. Implement an `optional_lens` (prism) that focuses on the `Some` variant of an `Option` field, returning `None` from get when the field is absent.
