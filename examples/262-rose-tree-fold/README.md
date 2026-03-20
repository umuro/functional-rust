📖 **[View on hightechmind.io →](https://hightechmind.io/rust/262-rose-tree-fold)**

---

# Example 262: Rose Tree — Multi-Way Tree with Fold

**Difficulty:** ⭐⭐  
**Category:** Trees  
**OCaml Source:** https://cs3110.github.io/textbook/chapters/ds/trees.html

## Problem Statement

Implement a rose tree (n-ary tree) where each node holds a value and a list of children. Define a generic fold operation that processes the tree bottom-up, then derive size, depth, and string representation from fold.

## Learning Outcomes

- Modeling n-ary trees with struct + Vec (vs OCaml's tuple-in-variant)
- Higher-order fold over recursive structures using trait objects (`&dyn Fn`)
- Deriving multiple operations from a single generic fold
- Understanding bottom-up vs top-down tree processing

## OCaml Approach

OCaml wraps the value and children list in a single variant: `Rose of 'a * 'a rose list`. The `fold` function takes a combining function and recursively maps fold over children. Partial application makes `size`, `depth`, and `to_string` concise one-liners.

## Rust Approach

Rust uses a struct with a `Vec<Rose<T>>` for children (no Box needed since Vec already heap-allocates). The fold takes a `&dyn Fn` trait object for the combining function. Rust can't partially apply fold as elegantly, so derived functions are standalone.

## Key Differences

1. **Data representation:** OCaml uses `Rose of 'a * 'a rose list` (variant with tuple); Rust uses a named struct with fields
2. **Higher-order functions:** OCaml's fold returns a partially-applied function; Rust needs explicit function signatures
3. **Children storage:** OCaml uses a linked list; Rust uses Vec (better cache locality, random access)
4. **Fold closure:** OCaml passes closures freely; Rust uses `&dyn Fn` trait objects for recursive fold

## Exercises

1. Implement a `map` for the rose tree that transforms every node value using a provided function, analogous to `Vec::iter().map()`.
2. Write a `depth` function that returns the maximum depth of the rose tree using a fold.
3. Implement `flatten` for the rose tree that returns all values in pre-order traversal order, and use `fold` as the underlying mechanism.
