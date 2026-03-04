# Example 072: Result Type — Railway-Oriented Error Handling

**Difficulty:** ⭐⭐
**Category:** Error Handling
**Concept:** Chaining fallible operations using the Result type so errors short-circuit the pipeline automatically. OCaml defines a custom `>>=` bind operator; Rust provides both `and_then` and the `?` operator built into the language.
**OCaml → Rust insight:** OCaml's `>>=` (bind) operator is Rust's `.and_then()` method, but Rust's `?` operator provides even more concise syntax for the same pattern — it's railway-oriented programming built into the language.
