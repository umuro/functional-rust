📖 **[View on hightechmind.io →](https://hightechmind.io/rust/080-catamorphism)**

---

# 080: Catamorphism — Generalized Fold on ADTs

**Difficulty:** Advanced
**Category:** Monadic Patterns / Data Structures
**Concept:** Replacing constructors with functions to fold any algebraic data type
**Key Insight:** OCaml's labeled arguments (~leaf, ~node) make catamorphisms elegant. In Rust, closures with `&dyn Fn` achieve the same but require `Clone` bounds.
