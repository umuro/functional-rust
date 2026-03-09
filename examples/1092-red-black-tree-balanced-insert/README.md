# Example 1092: Red-Black Tree — Balanced Insert

**Difficulty:** ⭐⭐⭐
**Category:** Trees & Balancing
**OCaml Source:** Okasaki, *Purely Functional Data Structures* (1998); [CS 3110 textbook](https://cs3110.github.io/textbook/chapters/ds/rb.html)

## Problem Statement

Implement a persistent (immutable) red-black tree with insertion that maintains balance via Okasaki's four-case rotation, plus membership lookup and in-order traversal.

## Learning Outcomes

- Translating OCaml's multi-case pattern matching into Rust's `match` with guards
- Modeling recursive algebraic data types with `Box<enum>` in Rust
- Ownership transfer in tree rotations — reusing subtrees without cloning
- `FromIterator` as Rust's analog of OCaml's `List.fold_left`

## OCaml Approach

OCaml defines `color` and `'a rbtree` as algebraic types and uses a single `function` match on a 4-tuple `(color, left, value, right)` with four or-patterns that all map to the same rebalanced node. The GC handles all allocation and sharing automatically.

## Rust Approach

Rust uses `enum Color` and `enum RBTree<T>` with `Box` for heap children. The `balance` function matches on a `(Color, RBTree<T>, RBTree<T>)` tuple, using `matches!` guards to peek at grandchild colors, then destructures by ownership to reassemble without extra cloning. `FromIterator` provides the fold-based construction.

## Key Differences

1. **Allocation:** OCaml's GC vs Rust's explicit `Box` for recursive children
2. **Or-patterns:** OCaml collapses four cases with `|`; Rust needs separate arms with guards
3. **Ownership:** Rust's `balance` takes ownership of subtrees, reusing boxes in rotation
4. **Persistence cost:** OCaml shares structure freely via GC; Rust must `.clone()` unchanged subtrees
