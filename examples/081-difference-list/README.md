📖 **[View on hightechmind.io →](https://hightechmind.io/rust/081-difference-list)**

---

# 081: Difference List — O(1) Append

**Difficulty:** Intermediate
**Category:** Data Structures
**Concept:** Using function composition for efficient list concatenation
**Key Insight:** OCaml difference lists are simply functions ('a list -> 'a list). Rust needs Box<dyn FnOnce> because closures have unique types and must be heap-allocated for storage.
