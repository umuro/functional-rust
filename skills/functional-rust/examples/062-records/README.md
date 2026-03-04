# Example 062: Records — Immutable Update and Pattern Matching

**Difficulty:** ⭐
**Category:** Algebraic Data Types
**Concept:** Named product types (records/structs) with functional update syntax. Both OCaml and Rust support creating modified copies of records without mutation, making immutable data structures ergonomic and expressive.
**OCaml → Rust insight:** OCaml's `{ r with field = value }` maps directly to Rust's `Struct { field: value, ..old }` — both are syntactic sugar for constructing a new value reusing unchanged fields.
