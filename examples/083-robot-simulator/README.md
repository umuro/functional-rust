📖 **[View on hightechmind.io →](https://hightechmind.io/rust/083-robot-simulator)**

---

# 083: Robot Simulator — State with Immutable Records

**Difficulty:** Intermediate
**Category:** Records and Variants
**Concept:** Functional state updates using record syntax and fold
**Key Insight:** OCaml's `{ r with field = value }` maps directly to Rust's `Robot { field: value, ..self }`. Both create new values rather than mutating.
