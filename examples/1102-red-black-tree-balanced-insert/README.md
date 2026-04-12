# Example 1102: Red-Black Tree — Balanced Insert
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement a purely functional red-black tree supporting insert and membership
test. Every insert returns a *new* tree; the old tree is unchanged. Balance is
maintained after each insert using Okasaki's four-case rewrite rule.

## Learning Outcomes

- How Rust `enum` with `Box` models recursive algebraic data types
- How Okasaki's elegant multi-case balance function translates from OCaml
  pattern matching to nested Rust `if let` chains
- Why `ref` bindings are needed when pattern-matching on owned enum values
  without consuming them
- How `impl FromIterator<T>` integrates a custom collection with Rust's
  iterator ecosystem

## OCaml Approach

OCaml's balance is a single `function` on a 4-tuple `(color, left, val, right)`.
All four red-red violation patterns are listed in one match, sharing the same
right-hand side. The type system ensures completeness with a catch-all arm.
Immutability is free — all values are persistent by default.

## Rust Approach

Rust cannot destructure deeply into `Box<T>` in a single match arm (no stable
box patterns). Instead, the four cases are written as nested `if let` chains.
`ref` bindings borrow the child nodes without moving them, keeping the owned
`left`/`right` values available for the default fallthrough. Immutability is
explicit — `insert` takes `&self` and returns a new `RbTree<T>`.

## Key Differences

1. **Pattern depth:** OCaml matches three levels deep in one arm; Rust nests
   `if let` blocks because `Box` can't be destructured in a stable match.
2. **Ownership:** OCaml values are GC-managed; Rust needs `Box` for the
   recursive tree nodes and `Clone` to copy subtrees that appear in both
   the old and new tree.
3. **Root invariant:** Both paint the root Black after insert — OCaml via a
   second `match ins t with`, Rust via the same idiom on the `ins` result.
4. **FromIterator:** Rust's trait system lets `RbTree` participate in `.collect()`
   idiomatically; OCaml uses `List.fold_left` ad hoc.

## Exercises

1. Implement `contains` on the red-black tree and write a test that verifies every inserted element can be found and no non-inserted element is found.
2. Add a method `black_height` that returns the number of black nodes on any root-to-leaf path and assert it is the same for all paths.
3. Implement a `from_iter` constructor that builds a balanced red-black tree from an unsorted iterator and verify it produces a valid BST by checking that `to_sorted_vec` yields elements in order.
