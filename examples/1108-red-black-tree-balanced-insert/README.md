# Example 1108: Red-Black Tree with Okasaki's Functional Balancing

## Problem Statement
Implement a persistent, immutable red-black tree using Okasaki's functional balancing algorithm. The tree must maintain balance invariants through purely functional transformations — no mutation allowed.

## Learning Outcomes
- Understand how algebraic data types encode recursive tree structure with color metadata
- See how Okasaki's 4-case pattern match collapses all double-red violations into one canonical rebalanced form
- Learn how structural sharing via `Clone` gives persistence without full copies on every insert

## Rust Application
The tree is defined as a recursive enum `RbTree<T>` with variants `Empty` and `Node(Color, Box<RbTree<T>>, T, Box<RbTree<T>>)`. `balance()` matches on the four rotational cases and always returns `Red(Black(a,x,b), y, Black(c,z,d))`, restoring the red-black invariant in a single pass.

## OCaml Approach
OCaml's native pattern matching on nested constructors expresses the four Okasaki cases with no boilerplate; algebraic variants and automatic garbage collection eliminate the need for explicit `Box` or ownership bookkeeping.

## Key Differences
1. **Heap allocation:** Rust requires explicit `Box<RbTree<T>>` for recursive variants; OCaml heap-allocates values implicitly
2. **Sharing vs. cloning:** Rust's `Clone` copies unchanged subtrees to satisfy ownership rules, giving logical persistence; OCaml shares subtree pointers freely via GC
3. **Exhaustiveness:** Both compilers enforce exhaustive pattern matching, but Rust requires explicit binding when destructuring nested `Box` values

## Exercises
1. Add a `contains()` function that traverses the tree and verify it respects the BST ordering invariant
2. Implement `to_sorted_vec()` via an in-order traversal and confirm the output is always sorted after a sequence of random inserts
3. Extend to a left-leaning red-black tree (LLRB) variant, which restricts red links to left children only and simplifies the case count
