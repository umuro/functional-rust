📖 **[View on hightechmind.io →](https://hightechmind.io/rust/073-topological-sort)**

---

# 073 — Topological Sort

**Difficulty:** ⭐⭐⭐ Advanced
**Category:** Graphs
**Concept:** Ordering nodes in a DAG so all edges point forward
**Key Insight:** OCaml's purely functional approach threads `(visited, order)` through recursive calls. Rust uses `HashSet` and `Vec` with mutable references — more explicit about who owns and mutates the state.

## Run

```bash
cargo test
```
