📖 **[View on hightechmind.io →](https://hightechmind.io/rust/005-currying)**

---

# Example 005: Currying and Partial Application

**Difficulty:** ⭐⭐
**Category:** Higher-Order Functions, Closures
**Concept:** Currying transforms a multi-argument function into a chain of single-argument functions. In OCaml this is automatic — every function is curried. In Rust, closures and `impl Fn` achieve the same effect but require explicit construction.
**OCaml → Rust key insight:** OCaml's `let add a b = a + b` (where `add 5` returns a function) becomes Rust's `fn add(n: i64) -> impl Fn(i64) -> i64 { move |x| x + n }`.
