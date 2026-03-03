# 070 — Scan Left

**Difficulty:** ⭐ Beginner
**Category:** Higher-order functions
**Concept:** Running accumulation — like fold but keeping all intermediate values
**Key Insight:** Rust has `Iterator::scan()` built-in, but its API uses mutable state (FnMut) — a pragmatic trade-off for performance over OCaml's purely functional approach.

## Run

```bash
cargo test
```
