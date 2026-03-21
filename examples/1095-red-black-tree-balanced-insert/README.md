# Example 1095: Red-Black Tree with Balanced Insert
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement a purely functional red-black tree supporting insert and membership, using Okasaki's elegant four-case balancing rule that collapses all rotation cases into a single pattern match.

## Learning Outcomes

- How OCaml's multi-case pattern matching on tuples translates to Rust match + guard patterns
- Ownership-based tree rebuilding: each insert creates a new path from root to leaf
- Using `Box` for recursive enum variants and the stable-Rust workarounds for the lack of `box` patterns
- Encoding algebraic data types (`color * 'a rbtree * 'a * 'a rbtree`) as Rust enums with tuple variants

## OCaml Approach

OCaml expresses the balance function as a single `function` with four or-patterns that all destructure nested tuples of `(color, tree, value, tree)`. Because OCaml values are heap-allocated and GC'd, there is no ownership concern — the old tree is simply discarded when unreachable. The `|` syntax makes the four rotation cases visually obvious and compact.

## Rust Approach

Rust cannot use `box` patterns on stable, so the balance function uses match guards (`if matches!(*ll, T(Red, ..))`) to detect the nested red-red violation, then a `let` destructure inside each arm. All subtrees are moved (ownership transfer), mirroring the functional style: each insert produces a fresh tree path. A helper `balanced()` function factors out the common result constructor shared by all four cases.

## Key Differences

1. **Pattern depth:** OCaml matches 3 levels deep in one pattern; Rust needs match guard + inner destructure since `box` patterns are nightly-only
2. **Memory management:** OCaml GC vs. Rust `Box<T>` ownership — each tree node is `Box`-allocated, old nodes are dropped when the new path replaces them
3. **Algebraic types:** OCaml's `type 'a rbtree = E | T of color * 'a rbtree * 'a * 'a rbtree` maps directly to `enum RBTree<T> { E, T(Color, Box<RBTree<T>>, T, Box<RBTree<T>>) }`
4. **Generics:** OCaml uses parametric polymorphism (`'a`) with structural equality; Rust requires `Ord` trait bound for comparison

## Exercises

1. Add a `find_all_in_range` method that returns all elements `e` where `lo <= e <= hi` in sorted order using a single in-order traversal that prunes irrelevant subtrees.
2. Implement a `map_values` function that applies a transformation `f: T -> U` to every element in the tree and returns a new `RedBlackTree<U>` (valid only when `U: Ord`).
3. Write a benchmark comparing insertion performance between the functional red-black tree and Rust's `BTreeSet` for 10,000 sequential and 10,000 random integers.
