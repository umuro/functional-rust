# Example 073: Word Count with Map

**Difficulty:** ⭐⭐
**Category:** Data Structures
**Concept:** Building a word-frequency map from text using string normalization, tokenization, and fold. A practical application of maps that combines string processing with functional accumulation.
**OCaml → Rust insight:** OCaml's `StringMap.add` returns a new immutable map on each insert; Rust's `HashMap::entry().or_insert()` mutates in place — both are O(log n) and O(1) amortized respectively.
