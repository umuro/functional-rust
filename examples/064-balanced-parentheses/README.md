# 064 — Balanced Parentheses

**Difficulty:** ⭐ Beginner
**Category:** Parsing
**Concept:** Stack-based bracket matching using a list/vec as stack
**Key Insight:** OCaml uses a list as an immutable stack (cons/pattern match); Rust uses `Vec` with `push`/`pop`, where `pop()` returns `Option<T>`.

## Run

```bash
cargo test
```
