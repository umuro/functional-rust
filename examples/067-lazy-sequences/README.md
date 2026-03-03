# 067 — Lazy Sequences (Seq Module)

**Difficulty:** ⭐⭐ Intermediate
**Category:** Lazy/infinite sequences
**Concept:** Lazy evaluation of potentially infinite sequences
**Key Insight:** Rust iterators are lazy by default — `(0..).filter().map().take(10)` computes nothing until consumed. OCaml needed `Seq` module (4.14+) to get similar ergonomics.

## Run

```bash
cargo test
```
