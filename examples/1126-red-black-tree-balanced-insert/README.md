# Example 1126: Red-Black Tree — Balanced Insert

**Difficulty:** ⭐⭐⭐  
**Category:** Trees | Balanced BST | Functional Data Structures  
**OCaml Source:** Okasaki, *Purely Functional Data Structures*, Chapter 3

## Problem Statement

Implement a persistent red-black tree supporting O(log n) insert and membership
using Okasaki's elegant four-case balance function.

## Learning Outcomes

- How to encode algebraic data types with recursive `Box` in Rust
- Why `Clone` is semantically correct for persistent (path-copying) trees
- How Rust's match ergonomics require care when nesting reference patterns
- How to translate Okasaki's multi-clause function match into sequential `if let` checks

## OCaml Approach

OCaml's `match` directly handles tuples of variants, letting all four balance
cases be written as a single pattern-match with structural sharing implied by
the garbage collector. The `balance` function is a single expression with four
symmetric cases and a default.

## Rust Approach

Rust encodes the tree with `Box<RbTree<T>>` for heap allocation and `Clone` for
path-copying (the persistent-tree idiom). The `balance` function borrows `left`
and `right` to detect which case applies, then clones only the subtrees that
form new nodes — avoiding full-tree copies while producing a new root.

## Key Differences

1. **Memory management:** OCaml uses GC-shared nodes; Rust uses path-copying with `Clone`, making persistence explicit and allocation-bounded.
2. **Pattern matching:** OCaml's tuple-of-variant patterns map cleanly to nested `if let` in Rust; using `ref` inside reference patterns creates double-references (`&&T`) — avoid it.
3. **Root invariant:** Both make the root black after insert; OCaml does it in the outer `match`, Rust does it by destructuring and rebuilding the root node.
4. **Type bounds:** The `Ord + Clone` bounds on `impl<T>` make the constraints explicit, whereas OCaml's type-class inference handles this implicitly.
