# Example 1083: Red-Black Tree

**Difficulty:** ⭐⭐⭐
**Category:** Trees
**OCaml Source:** Cornell CS3110 — Functional Programming in OCaml (Okasaki's algorithm)

## Problem Statement

Implement a purely functional red-black tree supporting insert and membership lookup, using Okasaki's elegant balancing technique where all four rotation cases collapse into a single rewrite rule.

## Learning Outcomes

- How Rust enums model algebraic data types (sum types with data) just like OCaml variants
- Ownership-based tree restructuring via path copying — Rust's move semantics naturally implement persistent data structures
- Pattern matching on nested owned data requires a peek-then-destructure strategy, unlike OCaml's direct nested pattern matching
- The `Box<T>` heap allocation pattern for recursive data structures

## OCaml Approach

OCaml defines the tree as a simple algebraic type `'a rbtree = E | T of color * 'a rbtree * 'a * 'a rbtree`. The `balance` function uses a single `match` with four or-patterns to catch all red-red violations and rewrite them into one canonical balanced form. This is the essence of Okasaki's insight — the code is remarkably concise because OCaml allows deep nested pattern matching across multiple cases in one arm.

## Rust Approach

Rust uses `enum RBTree<T>` with `Box` for recursive children. The `balance` function cannot match four nested patterns in one arm like OCaml, so it peeks at the structure via references (to determine which case applies) then destructures by moving owned values out. The tree is generic over `T: Ord` and uses `Ordering` for comparisons. Insert takes `self` by value and returns a new tree — natural path-copying via Rust's move semantics.

## Key Differences

1. **Pattern matching depth:** OCaml matches 3 levels deep across 4 cases in one arm; Rust requires sequential peek-then-destructure for each case
2. **Memory management:** OCaml's GC handles sharing automatically; Rust uses `Box<T>` with explicit heap allocation and move semantics for path copying
3. **Polymorphism:** OCaml uses `'a` with structural equality; Rust uses `T: Ord` trait bound for ordered comparisons
4. **Conciseness:** The OCaml `balance` is ~6 lines; Rust's is ~80 lines due to explicit destructuring — but both encode the same four-case logic

## Exercises

1. Implement `contains` for the red-black tree that searches for a value without modifying the tree structure.
2. Add an `in_order` method that returns all elements of the red-black tree as a sorted `Vec<T>` by performing an in-order traversal.
3. Implement `delete` for the red-black tree (the hardest operation) and verify that red-black invariants are maintained after each deletion using a property-based test.
