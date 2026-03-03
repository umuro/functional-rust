# Example 064: Map.Make Functor — String→Int Dictionary

**Difficulty:** ⭐⭐
**Category:** Modules / Functors
**Concept:** Creating type-safe immutable dictionaries. OCaml uses functors (`Map.Make`) to generate specialized map modules; Rust uses generic collections (`BTreeMap`, `HashMap`) with trait bounds on keys.
**OCaml → Rust insight:** OCaml's `Map.Make(String)` functor pattern is replaced by Rust's generic `BTreeMap<String, V>` — generics with trait bounds achieve the same type safety without module-level parameterization.
