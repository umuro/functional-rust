# Example 1091: Red-Black Tree — Balanced Insert (Iterator + Free Functions)

**Difficulty:** ⭐⭐⭐
**Category:** Trees & Balancing
**OCaml Source:** Okasaki, *Purely Functional Data Structures* (1998); [CS 3110 textbook](https://cs3110.github.io/textbook/chapters/ds/rb.html)

## Problem Statement

Implement a persistent red-black tree with Okasaki's four-case balance rotation, membership lookup, and in-order traversal — using free functions and a lazy stack-based iterator.

## Learning Outcomes

- Translating OCaml's module-level free functions (`balance`, `insert`, `mem`) into Rust public functions
- Implementing a proper `Iterator` trait for tree traversal (stack-based, O(log n) space)
- Ownership transfer in `balance` — destructuring `Box<Node>` to reuse subtrees without cloning
- Wrapping free functions with methods to offer both OCaml-style and Rust-style APIs

## OCaml Approach

OCaml defines `balance` and `insert` as module-level functions operating on the `'a rbtree` type. The `balance` function uses a single pattern match with four or-patterns (all mapping to the same rebalanced node). `to_list` is a recursive function that eagerly builds a full list via append (`@`).

## Rust Approach

Rust mirrors the OCaml structure with public free functions `balance`, `insert`, and `mem`, then wraps them in `impl` methods for ergonomic use. The key addition is a stack-based `InOrder` iterator implementing the `Iterator` trait — yielding elements lazily in O(log n) space rather than building a full `Vec` eagerly. `FromIterator` provides fold-based construction.

## Key Differences

1. **Function style:** OCaml uses module-level `let` bindings naturally; Rust needs `pub fn` at module scope plus method wrappers in `impl` blocks
2. **Or-patterns:** OCaml collapses four balance cases with `|`; Rust uses separate match arms with `matches!` guards
3. **Traversal:** OCaml's `to_list` eagerly builds a list via `@`; Rust implements `Iterator` for lazy, O(log n)-space traversal
4. **Ownership:** Rust's `balance` takes ownership of subtrees, destructuring `Box` to reuse allocations in rotations

## Exercises

1. Extend the red-black tree with a `to_sorted_vec` method that performs an in-order traversal and collects all elements.
2. Implement a `merge` function that combines two red-black trees into a single balanced tree.
3. Write property-based tests verifying that after inserting a random sequence of integers the resulting tree is a valid BST and satisfies all red-black invariants (color and black-height rules).
