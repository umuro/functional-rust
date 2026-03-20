# Example 1096: Red-Black Tree with Okasaki's Functional Balancing

**Difficulty:** ⭐⭐⭐
**Category:** Trees & Functional Data Structures
**OCaml Source:** Chris Okasaki, *Purely Functional Data Structures* (1998), §3.3

## Problem Statement

Implement a persistent red-black tree with insertion that maintains balance
using Okasaki's elegant four-case rebalancing rule expressed as pattern matching.

## Learning Outcomes

- Translating OCaml algebraic data types (`type 'a rbtree`) to Rust enums with `Box<T>` for recursive types
- Expressing Okasaki's balance cases without nightly `box` patterns using match guards and `let-else`
- Using method-style (`impl` blocks) for functional data structures — idiomatic Rust OOP wrapping a pure functional core
- Understanding why `Clone` is needed for in-order traversal when the tree retains ownership of its values

## OCaml Approach

OCaml represents the tree as a sum type `'a rbtree` with constructors `E` and `T`. The `balance`
function uses a single `function` match with four or-patterns to detect all red-red violations,
collapsing them into one canonical balanced node. Insertion is a nested `let rec ins` with the root
repainted black afterward. The entire implementation is around 20 lines.

## Rust Approach

Rust mirrors the OCaml type with `enum RBTree<V>` using `Box` for heap-allocated children. The
four balance cases use match guards (`if matches!(*ll, T(Red, ..))`) since Rust doesn't support
nested `box` patterns on stable. Methods live in `impl<V: Ord>` blocks, giving a fluent API
(`tree.insert(x).insert(y)`) while the internals remain purely functional (each `insert` returns
a new tree).

## Key Differences

1. **Heap allocation:** OCaml GC handles recursive types automatically; Rust requires explicit `Box<T>` for indirection
2. **Pattern depth:** OCaml matches nested constructors in one pattern; Rust needs match guards + `let-else` destructuring
3. **Ownership semantics:** `insert` takes `self` by value (consuming the old tree); OCaml shares structure via GC
4. **Method style:** Rust uses `impl` blocks for a natural `tree.insert(x)` API; OCaml uses free functions `insert x t`

## Exercises

1. Implement `is_balanced` — a function that verifies all red-black invariants (no two consecutive red nodes, equal black heights on all paths) and returns a descriptive error on violation.
2. Add a `remove_min` operation that deletes the smallest element while maintaining balance.
3. Implement a `MultiSet` variant of the red-black tree that allows duplicate keys by storing a count alongside each element.
