# Example 1003: Map and Fold on Trees

## Problem Statement
Implement `map_tree` and `fold_tree` as fundamental higher-order operations on binary trees, then derive `size`, `depth`, `sum`, `preorder`, and `inorder` from `fold_tree` alone.

## Learning Outcomes
- Understand the tree catamorphism: `fold_tree` captures the full recursive structure of a tree, replacing `Leaf` with the identity and `Node` with a combining function
- Derive common tree operations from a single fold without writing separate recursions
- See how `map_tree` preserves tree shape while transforming values — the tree functor

## Rust Application
`fold_tree<T, U, F>(tree, acc, f)` takes a combining function `f: Fn(T, U, U) -> U` — called with `(value, left_fold, right_fold)` — and an initial `acc` for leaves. Both subtrees fold independently with the same `acc`, so `U: Clone` is required. `map_tree` passes a reference `&F` through recursion to avoid consuming the closure.

## OCaml Approach
OCaml's `fold_tree f acc` is a curried `let rec` using `function` pattern matching. The `acc` flows to both subtrees unchanged, and `f v (fold_tree f acc l) (fold_tree f acc r)` combines node value with left and right results. Polymorphism is implicit — no explicit `Clone` bound needed.

## Key Differences
1. **Clone requirement:** Rust needs `U: Clone` because `acc` must be passed to both subtrees independently; OCaml shares the value freely under GC
2. **Closure passing:** Rust passes `&F` by reference through recursion to avoid moving the closure; OCaml closures are always by reference
3. **Derived operations:** Both languages define `size`, `depth`, `sum`, `preorder` as one-liners on top of `fold_tree` — the expressiveness is identical

## Exercises
1. Implement `filter_tree` that removes nodes whose values fail a predicate, replacing them with `Leaf` — and verify the resulting tree has fewer nodes
2. Implement `zip_tree` that combines two trees of the same shape into a tree of pairs, returning `None` if the shapes differ
3. Add a `flatten` function using `fold_tree` that collects all node values into a `Vec<T>` in preorder, then compare its output with the `preorder` function defined directly
