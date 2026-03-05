📖 **[View on hightechmind.io →](https://hightechmind.io/rust/093-isogram-check)**

---

# Example 093: Isogram Check

**Difficulty:** ⭐
**Category:** String Processing
**OCaml Source:** [Exercism — Isogram](https://exercism.org/tracks/ocaml/exercises/isogram)

## Problem Statement

Check if a word is an isogram — no letter appears more than once (ignoring case, hyphens, spaces).

## Learning Outcomes

- Map OCaml's `List.sort_uniq` to Rust's `sort_unstable()` + `dedup()`
- Use `HashSet::insert` return value for duplicate detection
- Apply bitset techniques for O(1) membership checks

## Key Insight

OCaml combines sort and dedup in one call (`List.sort_uniq`). Rust separates them (`sort_unstable()` then `dedup()`), but the `HashSet::insert` approach is more idiomatic — it returns `false` on duplicate, enabling early exit with `.all()`.
