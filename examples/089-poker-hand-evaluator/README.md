# Example 089: Poker Hand Evaluator

**Difficulty:** ⭐⭐⭐
**Category:** Records and Variants
**OCaml Source:** Classic functional pattern-matching exercise

## Problem Statement

Given five card ranks (as integers 2–14) and a boolean indicating whether all cards share the same suit, classify the poker hand into one of nine categories from High Card up to Straight Flush.

## Learning Outcomes

- Multi-arm tuple pattern matching: `match (is_flush, is_straight, counts.as_slice())`
- Slice patterns (`[4, ..]`, `[3, 2]`, `[2, 2, ..]`) for structural decomposition
- Deriving `PartialOrd`/`Ord` on enums to get free comparison ordering
- Using a HashMap inside a block expression to produce a sorted `Vec` in one binding

## OCaml Approach

OCaml classifies the hand by first computing a descending list of rank-count frequencies (e.g., `[3; 2]` for a full house), then pattern-matching a triple `(is_flush, is_straight, counts)` against concrete list shapes. Because OCaml lists carry structural equality, `[3; 2]` matches exactly a full-house count list.

## Rust Approach

Rust mirrors the logic with a `Vec<usize>` of sorted counts passed as `counts.as_slice()` into a `match`. Slice patterns (`[4, ..]`, `[3, 2]`) replace OCaml list patterns one-to-one. The `HandType` enum derives `Ord` so hands are naturally comparable without a separate ranking function.

## Key Differences

1. **Pattern subjects:** OCaml matches `(bool, bool, int list)`; Rust matches `(bool, bool, &[usize])` using stable slice pattern syntax.
2. **Structural list patterns:** OCaml uses `4 :: _` for "starts with 4"; Rust uses `[4, ..]`.
3. **Enum ordering:** OCaml needs a separate comparison function; Rust `#[derive(Ord)]` gives ordering for free based on declaration order.
4. **Frequency counting:** OCaml threads `List.filter` through `List.map` on unique values; Rust uses a `HashMap` then converts to a sorted `Vec`.
