📖 **[View on hightechmind.io →](https://hightechmind.io/rust/006-function-composition)**

---

# Example 006: Function Composition

**Difficulty:** ⭐⭐
**Category:** Higher-Order Functions, Composition
**Concept:** Function composition builds complex transformations from simple, reusable parts. OCaml uses the `|>` pipe and custom composition operators; Rust uses iterator method chaining and closures to achieve the same composability.
**OCaml → Rust key insight:** OCaml's `x |> f |> g` pipeline maps directly to Rust's `data.iter().map(f).filter(g).collect()` — method chaining IS composition.
