# 072 — Matrix Operations

**Difficulty:** ⭐⭐ Intermediate
**Category:** Math/recursion
**Concept:** Matrix transpose and multiply using nested collections
**Key Insight:** Rust's `iter().zip().map().sum()` chain for dot product is as clean as OCaml's `fold_left2`, and the borrow checker ensures you never accidentally mutate the input matrices.

## Run

```bash
cargo test
```
