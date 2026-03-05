# 1049: Persistent HashMap

**Difficulty:** Advanced
**Category:** Data Structures
**Concept:** Functional update — operations return new versions while old versions remain valid
**Key Insight:** OCaml's `Map` is inherently persistent with structural sharing (O(log n) per update). Rust has no built-in persistent map — `clone()` simulates persistence but copies everything. For real persistence, use the `im` crate (HAMT-based).
