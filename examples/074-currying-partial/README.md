# Example 074: Currying, Partial Application, and Sections

**Difficulty:** ⭐
**Category:** Higher-Order Functions
**Concept:** Every OCaml function is curried by default, enabling free partial application. Rust functions are not curried — closures and explicit returns of `impl Fn` achieve the same effect with more ceremony but equal power.
**OCaml → Rust insight:** OCaml's `let add5 = add 5` is effortless partial application; Rust requires `let add5 = |y| add(5, y)` or a function returning `impl Fn` — the power is equal but the ergonomics differ significantly.
