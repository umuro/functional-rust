📖 **[View on hightechmind.io →](https://hightechmind.io/rust/013-direct-rle)**

---

# Example 013: Direct Run-Length Encoding

**Difficulty:** ⭐⭐
**Category:** Lists, Stateful Recursion
**Concept:** Unlike the two-step RLE (pack then count), direct RLE counts consecutive duplicates in a single pass. This tests your ability to carry state through recursion or folds.
**OCaml → Rust key insight:** OCaml's recursive accumulator pattern (`aux count acc = function ...`) translates to Rust's `fold` with mutable accumulator state, or imperative `while` loops — both are idiomatic.
