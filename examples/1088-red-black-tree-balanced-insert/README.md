# Example 1088: Red-Black Tree — Balanced Insert

**Difficulty:** ⭐⭐⭐
**Category:** Trees & Balanced Data Structures
**OCaml Source:** Okasaki, *Purely Functional Data Structures*, Chapter 3.3

## Problem Statement

Implement a persistent (immutable) red-black tree with balanced insertion. The tree must maintain all red-black invariants: the root is black, no red node has a red child, and every path from root to leaf has the same number of black nodes.

## Learning Outcomes

- How Rust's `enum` + `Box` models recursive algebraic data types like OCaml's variants
- Pattern matching on nested enums as the Rust equivalent of OCaml's deep pattern matching
- Ownership transfer in tree rotations — why `balance` consumes subtrees by value
- Persistent data structures in Rust via `Clone` — each `insert` returns a new tree

## OCaml Approach

OCaml represents the tree as a recursive variant `'a rbtree = E | T of color * 'a rbtree * 'a * 'a rbtree`. The `balance` function uses a single `function` match with four or-patterns to catch all red-red violations and rotate to a uniform balanced shape. Garbage collection handles the old nodes automatically.

## Rust Approach

Rust uses `enum RBTree<T> { Empty, Node(Color, Box<RBTree<T>>, T, Box<RBTree<T>>) }` — `Box` provides the indirection that OCaml gets for free. The `balance` function matches on `(color, &left, &right)` with guard clauses, then destructures by value to move subtrees into the new rotation. `Clone` enables persistence since Rust has no GC.

## Key Differences

1. **Heap allocation:** OCaml allocates variant nodes on the GC heap implicitly; Rust requires explicit `Box::new()` for recursive types
2. **Pattern matching depth:** OCaml's or-patterns (`| case1 | case2 -> result`) merge four rotation cases into one arm; Rust needs separate arms with guards since or-patterns can't bind different variables
3. **Ownership in rotations:** OCaml freely shares subtrees between old and new nodes via GC; Rust must `clone()` shared subtrees or carefully move ownership through destructuring
4. **Persistence cost:** OCaml's GC makes persistent trees nearly free; Rust's `Clone`-based persistence has explicit allocation cost but predictable performance

## Exercises

1. Implement an `iter` method on the red-black tree that yields elements in sorted order using an explicit stack-based in-order traversal.
2. Add a `len` method that returns the number of elements in the tree, maintaining a count field updated on each insert (handle duplicate inserts as no-ops).
3. Implement `from_sorted_iter` that constructs a balanced red-black tree directly from a sorted iterator without repeated single-element insertions.
