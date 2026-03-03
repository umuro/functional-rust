# Example 066: Map and Fold on Trees

**Difficulty:** ⭐⭐
**Category:** Higher-Order Functions
**Concept:** Lifting `map` and `fold` from lists to binary trees. Once `fold_tree` is defined, size, depth, sum, and traversals are all one-liners — no explicit recursion needed. This is the catamorphism pattern applied to trees.
**OCaml → Rust insight:** OCaml's `fold_tree` takes a 3-argument function naturally via currying; Rust needs closures with explicit references (`&impl Fn`) and `Clone` bounds for the accumulator.
