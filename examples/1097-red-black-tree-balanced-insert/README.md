# Example 1097: Red-Black Tree — Balanced Insert

**Difficulty:** ⭐⭐⭐
**Category:** Trees & Balancing
**OCaml Source:** Chris Okasaki, *Purely Functional Data Structures* (1998), Chapter 3.3

## Problem Statement

Implement a red-black tree that maintains balance invariants through a `balance` function invoked after every insertion, guaranteeing O(log n) search and insert.

## Learning Outcomes

- How Rust enums with `Box` model OCaml's recursive algebraic data types
- Pattern matching with match guards as a substitute for OCaml's or-patterns
- Ownership transfer through tree rotations — each node is consumed and rebuilt
- The `FromIterator` trait as the idiomatic Rust equivalent of OCaml's `List.fold_left`

## OCaml Approach

OCaml uses algebraic data types (`type 'a rbtree = E | T of color * 'a rbtree * 'a * 'a rbtree`) with garbage-collected recursive references. The `balance` function uses four or-patterns to match all red-red violation cases in a single `function` expression, producing the canonical balanced form. Insertion is purely functional — every operation returns a new tree.

## Rust Approach

Rust uses `enum` variants with `Box<RBTree<T>>` for heap-allocated recursive children. The `balance` function uses match guards (`if matches!(...)`) to distinguish the four violation cases, since Rust lacks OCaml's or-patterns with bindings. Insertion consumes `self` and returns a new tree, preserving the purely functional style while making ownership explicit.

## Key Differences

1. **Heap allocation:** OCaml's GC handles recursive references transparently; Rust requires explicit `Box` for each child node.
2. **Pattern matching:** OCaml's or-patterns (`| case1 | case2 -> result`) bind variables across alternatives; Rust needs separate match arms with guards.
3. **Ownership in rotations:** OCaml's GC allows sharing subtrees freely; Rust's `balance` must destructure and re-box each subtree, making the ownership flow explicit.
4. **Trait integration:** OCaml uses `List.fold_left` as a standalone function; Rust implements `FromIterator` so `.collect()` works natively with the type system.

## Exercises

1. Implement `select` — given index `k`, return the `k`-th smallest element (0-based) in O(log n) by augmenting each node with its subtree size.
2. Write a `split` function that takes a value and returns two trees: one with all elements less than the value, and one with all elements greater.
3. Implement a fully functional ordered map (`RBMap<K, V>`) backed by the red-black tree, with `insert`, `get`, and `remove` operations.
