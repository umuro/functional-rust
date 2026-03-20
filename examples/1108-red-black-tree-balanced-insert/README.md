# Example 1108: Red-Black Tree with Okasaki's Functional Balancing

**Difficulty:** ⭐⭐⭐
**Category:** Trees & Persistent Data Structures
**OCaml Source:** `example.ml` — Okasaki's original ML formulation with a single `balance` pattern match

## Problem Statement

Implement a persistent, immutable red-black tree using Okasaki's functional balancing algorithm. Every insert must preserve two structural invariants: no red node may have a red child, and every path from root to leaf must contain the same number of black nodes. The tree must support membership queries and in-order traversal, all without any mutation — each operation produces a new tree that shares structure with the original.

## Learning Outcomes

- How algebraic data types (`enum RbTree<T>` with `Empty` and `Node`) encode recursive tree structure together with color metadata in a single definition
- How Okasaki's four rotation cases all produce the same output shape `Red(Black(a,x,b), y, Black(c,z,d))` — a single canonical rebalanced form that eliminates the double-red violation
- Why Rust requires explicit `Box<RbTree<T>>` for recursive enum variants while OCaml allocates heap nodes implicitly
- How `Clone` on unchanged subtrees provides logical persistence: only the path from root to the insertion point is copied; the rest is shared
- How the root-blackening step at the end of `insert` enforces the global black-root invariant without complicating the recursive `ins` helper

## OCaml Approach

OCaml's `balance` function uses a single pattern match with four or-patterns on the `(color, left, value, right)` tuple, all mapping to one right-hand side. This is Okasaki's original formulation: the four cases for left-left, left-right, right-left, and right-right double-red violations collapse into one return expression. Garbage collection handles node lifetimes automatically, and structural sharing of unchanged subtrees is implicit — the OCaml runtime shares pointers without any programmer annotation. The total implementation fits in about 10 lines of code, demonstrating how algebraic pattern matching minimizes boilerplate for structural transformations.

## Rust Application

The tree is `enum RbTree<T> { Empty, Node(Color, Box<RbTree<T>>, T, Box<RbTree<T>>) }`. The `balance` function matches the four rotation cases with nested `if let` chains rather than or-patterns, because Rust's borrow checker requires each destructured reference to remain valid through the match arm. `insert` calls the inner `ins` helper recursively, then paints the root black. `from_iter` builds a tree from any iterator via `fold`. The invariant checker `check_invariants` walks the tree and verifies both red-child and black-height rules, returning `Ok(black_height)` or a descriptive error — useful for property-based testing.

## Key Differences

1. **Heap allocation:** Rust requires explicit `Box<RbTree<T>>` so the recursive type has a known size at compile time; OCaml heap-allocates all values implicitly and the compiler infers the indirection
2. **Structural sharing:** OCaml shares subtree pointers freely via GC with zero programmer effort; Rust uses `Clone` to copy unchanged subtrees, which is logically equivalent but copies data along the insertion path
3. **Pattern matching ergonomics:** OCaml expresses all four Okasaki cases as a single `match` with or-patterns; Rust splits them into sequential `if let` chains because or-patterns with bound variables of different types require separate arms
4. **Invariant enforcement:** Both compilers enforce exhaustive pattern matching, but Rust's borrow checker additionally prevents dangling references when destructuring `Box`-wrapped recursive variants

## Exercises

1. Add a `height` function that computes the maximum depth of the tree and verify that after inserting `n` elements in sorted (worst-case) order, the height stays within 2 log₂(n+1)
2. Implement `to_sorted_vec` via in-order traversal (it is already provided — extend it to return `(Color, T)` pairs so you can visualize the coloring of each level)
3. Implement a `delete` operation using Okasaki's double-black technique — this is significantly harder than insert and requires additional color states
4. Replace the `Clone`-based subtree copying with `Rc<RbTree<T>>` to enable true pointer-sharing persistence and measure the allocation reduction on a large insert sequence
