# 069 — Unfold

**Difficulty:** ⭐⭐ Intermediate
**Category:** Higher-order functions
**Concept:** The dual of fold — building lists/sequences from a seed value
**Key Insight:** Rust has `std::iter::successors` and `std::iter::from_fn` as built-in unfold equivalents, but a custom `unfold` function is easy to write and educational.

## Run

```bash
cargo test
```
