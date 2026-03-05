📖 **[View on hightechmind.io →](https://hightechmind.io/rust/092-pangram-check)**

---

# Example 092: Pangram Check

**Difficulty:** ⭐
**Category:** String Processing
**OCaml Source:** [Exercism — Pangram](https://exercism.org/tracks/ocaml/exercises/pangram)

## Problem Statement

Check if a string contains every letter of the English alphabet at least once.

## Learning Outcomes

- Compare OCaml's `Set.Make(Char)` functor with Rust's `HashSet<char>`
- Learn bitset tricks for alphabet membership (26 bits in a `u32`)
- See functional vs imperative approaches to the same problem

## Key Insight

OCaml requires a functor `Set.Make(Char)` to create a char set module. Rust's `HashSet<char>` works out of the box because `char` already implements `Hash + Eq`. For small alphabets, a bitset (`u32`) is the fastest approach in both languages.
