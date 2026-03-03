# 065 — Church Numerals

**Difficulty:** ⭐⭐⭐ Advanced
**Category:** Higher-order functions
**Concept:** Lambda calculus encoding of natural numbers as function application counts
**Key Insight:** OCaml's uniform closure representation makes Church numerals trivial. Rust requires `Box<dyn Fn>` and `Rc` for shared ownership — revealing the cost of first-class functions in a systems language.

## Run

```bash
cargo test
```
