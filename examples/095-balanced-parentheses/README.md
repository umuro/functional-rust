📖 **[View on hightechmind.io →](https://hightechmind.io/rust/095-balanced-parentheses)**

---

# Example 095: Balanced Parentheses

**Difficulty:** ⭐
**Category:** Parsing
**OCaml Source:** [Exercism — Matching Brackets](https://exercism.org/tracks/ocaml/exercises/matching-brackets)

## Problem Statement

Determine if a string has balanced brackets: `()`, `[]`, `{}` must properly nest and match.

## Learning Outcomes

- Use a list/Vec as a stack in both OCaml and Rust
- Apply `try_fold` for early-exit functional iteration
- Compare recursive (OCaml-style) vs imperative (Rust-style) stack processing

## Key Insight

OCaml naturally uses a list as an immutable stack (cons to push, pattern match to pop). Rust's `Vec` serves the same role with `push`/`pop`. The `try_fold` approach adds functional elegance with early exit via `None`.
