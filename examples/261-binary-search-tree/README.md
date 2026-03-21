📖 **[View on hightechmind.io →](https://hightechmind.io/rust/261-binary-search-tree)**

---

# Example 261: Binary Search Tree — Insert and Search
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Implement an immutable binary search tree with functional insert, membership check, and in-order traversal. Each insert returns a new tree while the original remains unchanged (persistence).

## Learning Outcomes

- Using `enum` with `Box` for recursive data structures in Rust
- Understanding persistent data structures through clone-on-write semantics
- Translating OCaml's pattern matching on algebraic types to Rust's `match`
- Using `Ord` trait for generic comparison instead of OCaml's structural equality

## OCaml Approach

OCaml defines a polymorphic BST with `type 'a bst = Leaf | Node of 'a bst * 'a * 'a bst`. Structural equality and comparison operators work automatically for all types. The recursive structure is naturally heap-allocated by the GC.

## Rust Approach

Rust requires `Box<T>` for recursive enum variants since the compiler needs to know the size at compile time. The `Ord` trait replaces OCaml's polymorphic comparison. Cloning subtrees is explicit, making the cost of persistence visible.

## Key Differences

1. **Heap allocation:** OCaml GC handles it implicitly; Rust needs `Box` for recursive types
2. **Comparison:** OCaml uses built-in `<`, `>`, `=` for all types; Rust requires `Ord` trait bound
3. **Persistence cost:** OCaml shares unchanged subtrees via GC; Rust must `.clone()` explicitly
4. **Type parameters:** OCaml uses `'a` with no constraints; Rust needs `T: Ord + Clone`

## Exercises

1. Implement `delete` for the BST: remove an arbitrary node while maintaining the BST invariant (use in-order successor replacement for nodes with two children).
2. Write an `is_valid_bst` checker that verifies the BST property holds for every node (not just locally), using range constraints propagated through the recursion.
3. Implement a balanced BST construction from a sorted `Vec<T>` using divide-and-conquer (select the median as root), and verify the resulting tree has O(log n) height.
