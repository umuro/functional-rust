📖 **[View on hightechmind.io →](https://hightechmind.io/rust/097-zipper)**

---

# Example 097: Zipper — Functional List Cursor

**Difficulty:** ⭐⭐
**Category:** Data Structures
**OCaml Source:** [CS3110 — Zippers](https://cs3110.github.io/textbook/chapters/ds/zippers.html)

## Problem Statement

Implement a zipper: a data structure that provides O(1) access to a "focus" element in a list, with efficient navigation left and right.

## Learning Outcomes

- Map OCaml's record type to a Rust struct with generics
- Understand the zipper pattern: split a list into (reversed-left, focus, right)
- Compare immutable (clone-based) vs mutable approaches in Rust

## Key Insight

The zipper trades random access for O(1) local operations. OCaml's `{ z with focus = ... }` (functional record update) maps to Rust cloning individual fields. For mutable zippers, `VecDeque` gives O(1) front operations.
