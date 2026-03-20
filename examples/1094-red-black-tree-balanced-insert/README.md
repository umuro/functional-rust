# Example 1094: Red-Black Tree — Okasaki's Functional Balanced Insert

**Difficulty:** ⭐⭐⭐
**Category:** Trees & Persistent Data Structures
**OCaml Source:** Chris Okasaki, *Purely Functional Data Structures* (1998), Chapter 3.3

## Problem Statement

Implement a persistent (immutable) red-black tree with balanced insertion using Okasaki's elegant functional approach, where a single `balance` function captures all four rotation cases via pattern matching.

## Learning Outcomes

- Translating OCaml's algebraic data types with nested pattern matching to Rust enums with `Box`-based recursion
- Understanding how `Box<T>` provides heap allocation for recursive types — Rust's analog to OCaml's GC-managed variants
- Seeing how Rust's ownership system interacts with persistent data structures (clone-on-write vs OCaml's free structural sharing)
- Appreciating that Okasaki's four-case balance function translates almost directly, but Rust's move semantics require a two-phase match-then-destructure approach

## OCaml Approach

OCaml uses algebraic data types (`type 'a rbtree = E | T of ...`) with garbage-collected nodes, making structural sharing free. The `balance` function uses a single `function` match with four or-patterns that destructure three levels deep, producing the rotated subtree in one expression. Insertion is a pure recursive function with a local `ins` helper.

## Rust Approach

Rust uses `enum` with `Box<RBTree<T>>` for heap-allocated children. The `balance` function must work around Rust's ownership rules: we first match on references to detect the violation pattern, then destructure by value to move children into the new rotated shape. The `Clone` bound is necessary because persistent insertion must copy the path from root to insertion point.

## Key Differences

1. **Heap allocation:** OCaml GC manages tree nodes automatically; Rust requires explicit `Box::new()` for each child pointer
2. **Pattern matching depth:** OCaml matches three levels deep in one pattern; Rust uses guard clauses (`if matches!`) then nested `if let` to destructure by value
3. **Structural sharing cost:** OCaml shares subtrees for free via GC; Rust must `.clone()` unchanged subtrees along the insertion path
4. **Type constraints:** OCaml's polymorphism is implicit; Rust requires `T: Ord + Clone` bounds to compare and copy values

## Exercises

1. Implement a `height` function that computes the maximum depth of the red-black tree and verify it stays within O(log n) bounds after bulk insertions.
2. Add `predecessor` and `successor` methods that return the next smaller or larger element relative to a given value.
3. Implement a persistent (immutable) set of integers backed by the red-black tree and demonstrate structural sharing: inserting a new value creates a new tree root while sharing unchanged subtrees.
