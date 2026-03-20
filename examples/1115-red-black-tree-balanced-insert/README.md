# Example 1115: Red-Black Tree — Balanced Insert

**Difficulty:** ⭐⭐⭐
**Category:** Trees | Balanced Data Structures | Functional Patterns
**OCaml Source:** Okasaki, "Purely Functional Data Structures" (Chapter 3)

## Problem Statement

Implement a functional red-black tree with insertion that maintains the red-black invariants: every red node has only black children, and all paths from root to leaf contain the same number of black nodes.

## Learning Outcomes

- How Okasaki's elegant 4-case `balance` function restores BST invariants after insertion
- Pattern matching on boxed recursive enums using guard clauses in stable Rust
- Why the root is always recolored black after insertion
- How functional trees achieve O(log n) insert without mutation

## OCaml Approach

OCaml's algebraic data types and exhaustive pattern matching make `balance` a single-match function covering all 4 red-red violation cases via or-patterns. The recursive structure is natural and the garbage collector handles sharing.

## Rust Approach

Rust uses `Box<RBTree<T>>` to break the recursive type. Because stable Rust cannot directly use nested box patterns, the 4 balance cases are expressed with guard clauses (`if matches!(*a, T(Red, ..))`), then destructuring via `let ... else`. The root-recoloring step in `insert` guarantees the black-root invariant.

## Key Differences

1. **Recursive types:** OCaml allows `'a rbtree` directly; Rust requires `Box<RBTree<T>>` for heap indirection.
2. **Pattern matching through boxes:** OCaml or-patterns cover all 4 cases in one match arm; Rust needs guard clauses to inspect boxed children.
3. **Memory model:** OCaml shares unchanged subtrees via GC; Rust moves/owns subtrees — no implicit sharing.
4. **Generics vs polymorphism:** OCaml's `'a rbtree` is implicitly polymorphic; Rust uses explicit `<T: Ord>` bounds.

## Exercises

1. Write a `fold` over the red-black tree that applies a binary function to accumulate a result in sorted order, and use it to produce a comma-separated string of all values.
2. Implement `intersection` — return a new tree containing only elements present in both input trees — using the `contains` method and insertion.
3. Profile insertion of 1 million random `u64` values into your red-black tree vs. `std::collections::BTreeSet` and explain performance differences in terms of allocation strategy and cache behavior.
