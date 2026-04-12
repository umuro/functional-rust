# Example 1003: Map and Fold on Trees
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement `map_tree` and `fold_tree` as the two fundamental higher-order operations on binary trees, then derive `size`, `depth`, `sum`, `preorder`, and `inorder` from `fold_tree` alone — without writing any additional recursion. `fold_tree` is the tree catamorphism: it captures the complete recursive structure of the type, replacing each `Leaf` with a base value and each `Node` with a combining function. Understanding it reveals why higher-order functions eliminate boilerplate and why functional programmers reach for folds as their first tool when reducing a data structure.

## Learning Outcomes

- What a catamorphism is and why `fold_tree` completely characterizes the `Tree` type's recursive structure
- How to derive `size`, `depth`, `sum`, `preorder`, and `inorder` as one-liners using a single fold with no additional recursion
- Why `U: Clone` is required in Rust's `fold_tree` but not in OCaml's version, and how the garbage collector changes the picture
- How `map_tree` preserves the tree's shape while transforming values — making `Tree` a functor over its element type
- Why closures in Rust must be passed as `&F` through recursive calls to avoid consuming the closure on the first use

## OCaml Approach

OCaml's `fold_tree f acc` is a curried two-argument `let rec` function using `function` for pattern matching. On a `Leaf` it returns `acc`; on a `Node(v, l, r)` it computes `f v (fold_tree f acc l) (fold_tree f acc r)` — calling the user's function with the node value and the results of folding both subtrees. The same `acc` is shared freely to both subtrees because OCaml's GC manages value lifetime. All five derived operations are single `fold_tree` applications: `size t = fold_tree (fun _ l r -> 1 + l + r) 0 t`. The elegance is in the zero additional recursion needed.

## Rust Application

`fold_tree<T, U, F>(tree: Tree<T>, acc: U, f: &F) -> U` takes ownership of the tree and a combining function `F: Fn(T, U, U) -> U`. The function receives the node value and the results of folding both subtrees: `f(v, left_result, right_result)`. Because `acc` must be passed independently to both the left and right subtrees, `U: Clone` is required — the clone happens once per node. The closure is passed as `&F` throughout recursion so it is borrowed rather than moved, enabling re-use across all node visits. `map_tree` takes ownership of the source tree and returns a `Tree<U>` of identical shape using the same `&F` borrow pattern.

## Key Differences

1. **Clone requirement:** Rust requires `U: Clone` because `acc` must be duplicated for the two independent subtree folds; OCaml shares the same value freely since the GC tracks all references
2. **Closure ownership:** Rust passes `&F` through every recursive call to prevent consuming the closure; OCaml closures are heap-allocated reference values and always shareable
3. **Derived operations:** Both languages express `size`, `depth`, `sum`, `preorder`, and `inorder` as one-liners over `fold_tree` — the expressive power is identical, only the syntax differs
4. **Tree ownership:** Rust's `fold_tree` consumes the tree (`tree: Tree<T>`) transferring ownership through the recursion; OCaml's fold borrows implicitly since the GC handles memory

## Exercises

1. Implement `filter_tree` that removes nodes whose values fail a predicate, replacing each failing node with `Leaf`, and verify that the resulting tree has strictly fewer nodes than the original
2. Implement `zip_tree` that takes two trees of the same shape and combines them node-by-node into a `Tree<(A, B)>`, returning `None` if the shapes differ at any point
3. Add a `flatten` function implemented purely via `fold_tree` that collects all node values into a `Vec<T>` in preorder order, then write a test confirming its output matches the `preorder` function defined in this module
