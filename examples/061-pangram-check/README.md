# 061 — Pangram Check

**Difficulty:** ⭐ Beginner
**Category:** String processing
**Concept:** Set-based string analysis to verify all alphabet letters are present
**Key Insight:** Rust's `HashSet` and iterator chains make set operations elegant, while a bitflag approach avoids heap allocation entirely.

## What it does

Checks whether a sentence is a pangram — a sentence containing every letter of the alphabet at least once. Three approaches: HashSet, bitflag (u32), and recursive.

## Run

```bash
cargo test
```
