📖 **[View on hightechmind.io →](https://hightechmind.io/rust/117-recursive-types)**

---

# Recursive Types

## Problem Statement

A recursive type is one whose definition refers to itself — a tree node contains child nodes, a linked list node points to the next node. This is fundamental to representing hierarchical data: file systems, abstract syntax trees, JSON documents, and parse trees. Rust requires recursive types to have a known size at compile time, which forces explicit indirection via `Box<T>` at recursive positions.

## Learning Outcomes

- Understand why Rust requires `Box<T>` (or other heap-indirected pointers) for recursive type definitions
- Learn to implement a binary search tree using recursive enums
- See how insert, search, and traversal operations work on recursive structures
- Compare Rust's explicit `Box` wrapping to OCaml's implicit heap allocation for algebraic types

## Rust Application

The code defines `Tree<T>` as an enum with `Leaf` and `Node(Box<Tree<T>>, T, Box<Tree<T>>)` variants. Without `Box`, the compiler cannot determine `Tree`'s size because it would be self-referential. `Box<Tree<T>>` is pointer-sized (8 bytes on 64-bit), breaking the cycle. Insert consumes `self` and returns the updated tree, maintaining the functional style — the old tree is moved into the new one, and only the changed path is newly allocated.

## OCaml Approach

OCaml's type system handles recursive algebraic types without any annotation:
```ocaml
type 'a tree = Leaf | Node of 'a tree * 'a * 'a tree
```
Every OCaml value is heap-allocated and pointer-accessed through the GC, so there is no size-at-compile-time problem. Pattern matching on trees is identical in structure to Rust's `match`, but without `Box::new` or dereferencing.

## Key Differences

1. **Indirection**: OCaml allocates all variants on the heap automatically; Rust requires explicit `Box<T>` at recursive positions to break size cycles.
2. **Ownership on insert**: Rust's `insert(self, x)` consumes the old tree (move semantics); OCaml's `insert` returns a new tree while the old one stays alive via GC.
3. **Pattern matching**: Both use structural pattern matching, but Rust must dereference `Box` in patterns (handled transparently by the compiler in modern Rust).
4. **Leaf representation**: OCaml's `Leaf` is a zero-size value; Rust's `Tree::Leaf` is similarly zero-size, but `Node` pays for two `Box` pointers.

## Exercises

1. Add a `contains` method to `Tree<T>` that searches for a value using binary search.
2. Implement `to_sorted_vec` using in-order traversal and verify it returns elements in ascending order.
3. Add a `height` method that computes the maximum depth of the tree from root to any leaf.
