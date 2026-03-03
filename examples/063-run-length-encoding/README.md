# 063 — Run-Length Encoding

**Difficulty:** ⭐ Beginner
**Category:** String processing
**Concept:** Encoding consecutive character runs into count+char pairs
**Key Insight:** Rust's ownership model means building strings requires explicit `String::push`/`push_str` — no implicit buffer like OCaml's `Buffer.t`.

## Run

```bash
cargo test
```
