# Example 004: Option and Result

**Difficulty:** ⭐⭐
**Category:** Error Handling, Monadic Types
**Concept:** Both OCaml and Rust replace null pointers and exceptions with Option (maybe a value) and Result (success or error). These types force explicit handling at compile time, eliminating null pointer exceptions and uncaught errors.
**OCaml → Rust key insight:** OCaml's `|>` pipeline with `Option.map`/`Result.bind` maps to Rust's method chains with `.map()`, `.and_then()`, and the `?` operator.
