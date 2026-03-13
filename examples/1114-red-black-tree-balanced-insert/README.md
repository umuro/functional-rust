# Example 1114: Red-Black Tree — Balanced Insert

**Difficulty:** ⭐⭐⭐
**Category:** Trees | Balanced Data Structures | Functional Patterns
**OCaml Source:** Okasaki, *Purely Functional Data Structures* (1998), Chapter 3

## Problem Statement

Implement a persistent red-black tree with O(log n) insert and membership operations, using Okasaki's functional balancing approach: four structural pattern-match cases that each rewrite a doubly-red violation into a balanced sub-tree.

## Learning Outcomes

- How recursive algebraic data types map from OCaml to Rust when `Box` indirection is required for heap allocation
- Why Rust cannot pattern-match through `Box` in a single match arm, and how nested `match *box_value` solves it
- How `fold` over an iterator replaces OCaml's `List.fold_left` for building persistent structures
- Why the root-forced-black step (after `ins`) is the key invariant-restoration step

## OCaml Approach

OCaml's `type 'a rbtree = E | T of color * 'a rbtree * 'a * 'a rbtree` stores children inline — no heap pointer indirection — so pattern matching can destructure arbitrarily deep in a single `match` arm. Okasaki's `balance` function exploits this with four or-patterns that all collapse to the same right-hand side, making the code almost a direct transcription of the structural property.

## Rust Approach

Rust requires `Box<RBTree<T>>` for the recursive children (to bound the type's size). This means pattern matching cannot see through Box in a single arm. The solution: match the outer constructor first to bind the `Box`, then use `match *box` in a nested arm to unbox and inspect the inner value. The rest of the logic (insert, mem, to_list) is a near-direct translation using `Ord` bounds instead of structural comparison.

## Key Differences

1. **Recursive types:** OCaml's `'a rbtree` is a value type stored inline; Rust needs `Box<RBTree<T>>` to break the infinite-size cycle.
2. **Pattern matching depth:** OCaml matches arbitrarily deep in one arm; Rust requires nested `match *box` to deref-and-inspect Box fields.
3. **Polymorphism:** OCaml uses implicit parametric polymorphism `'a`; Rust requires explicit `<T: Ord>` trait bounds.
4. **Persistent fold:** OCaml's `List.fold_left (fun t x -> insert x t) E xs` becomes `iter.fold(E, |tree, v| insert(v, tree))` — identical intent, different syntax.
