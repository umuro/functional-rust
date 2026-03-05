📖 **[View on hightechmind.io →](https://hightechmind.io/rust/275-yacht-dice-scoring)**

---

# Example 275: Yacht Dice Scoring

**Difficulty:** ⭐⭐
**Category:** Pattern Matching | Algebraic Data Types
**OCaml Source:** Classic Yacht dice game scoring exercise

## Problem Statement

Score a roll of five dice against one of twelve Yacht categories. Each category
has different rules: number categories sum matching dice, Yacht awards 50 for
five-of-a-kind, FullHouse awards the sum when dice form a 2+3 pair, and
straights award 30 for the sequences 1-5 or 2-6.

## Learning Outcomes

- Modeling a closed set of alternatives with Rust enums vs OCaml variant types
- Using fixed-size arrays (`[u8; 5]`) instead of lists for fixed-length data
- Replacing OCaml's exception-driven `List.find / Not_found` with `Option`-based `.find().unwrap_or()`
- Writing frequency-counting helpers that avoid fragile sorted-pattern matching

## OCaml Approach

OCaml uses a variant type for categories and dispatches with a multi-arm
`function` expression. FullHouse is detected by matching sorted list patterns
with guards, and FourOfAKind uses `List.find` with a `try/with Not_found`
fallback. Sorted comparison against literal lists handles the straights.

## Rust Approach

Rust models the same logic with a `#[derive]`-annotated enum and an exhaustive
`match`. Fixed-size `[u8; 5]` arrays replace OCaml lists; `sort_unstable`
replaces `List.sort`. FullHouse uses a frequency-count approach (more robust
than pattern-matching on sorted values). FourOfAKind uses iterator
`.find().map().unwrap_or(0)`, replacing the exception-based OCaml idiom cleanly.

## Key Differences

1. **Variant / enum:** OCaml `type category = Ones | ...` maps directly to `pub enum Category { Ones, ... }` — the concept is identical, the syntax is similar.
2. **Fixed-length data:** OCaml uses `'a list` for dice; Rust uses `[u8; 5]`, making the "exactly five dice" invariant a compile-time guarantee.
3. **Error handling:** OCaml `List.find` raises `Not_found`; Rust `Iterator::find` returns `Option`, handled with `.map().unwrap_or(0)` — no exceptions.
4. **FullHouse detection:** OCaml matches sorted patterns; Rust builds a frequency table and checks sorted frequency counts equal `[2, 3]` — avoids fragile guard expressions that clippy flags.
