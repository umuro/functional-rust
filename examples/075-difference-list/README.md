📖 **[View on hightechmind.io →](https://hightechmind.io/rust/075-difference-list)**

---

# 075 — Difference List

**Difficulty:** ⭐⭐ Intermediate
**Category:** Data structures
**Concept:** O(1) append using function composition instead of list concatenation
**Key Insight:** In OCaml/Haskell, difference lists solve the O(n) list append problem elegantly. In Rust, `Vec` already has O(1) amortized `push` — making difference lists educational but rarely necessary in practice.

## Run

```bash
cargo test
```
