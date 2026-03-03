# 074 — Catamorphism

**Difficulty:** ⭐⭐⭐ Advanced
**Category:** Monadic patterns
**Concept:** Generalized fold that replaces constructors with functions
**Key Insight:** OCaml's labeled arguments (`~leaf`, `~node`) make catamorphism definitions beautifully readable. Rust requires `&dyn Fn` trait objects or generics, making the pattern more explicit but more verbose.

## Run

```bash
cargo test
```
