# Example 003: Pattern Matching

**Difficulty:** ⭐⭐
**Category:** Pattern Matching, Algebraic Data Types
**Concept:** Pattern matching is the primary control flow in both OCaml and Rust. Using a Shape type, we demonstrate exhaustive matching, guard clauses, nested patterns, and how both languages force you to handle every case.
**OCaml → Rust key insight:** OCaml's `type shape = Circle of float | ...` maps directly to Rust's `enum Shape { Circle(f64), ... }` — the translation is almost mechanical.
