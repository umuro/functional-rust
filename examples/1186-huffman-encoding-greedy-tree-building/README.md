# Example 1186: Huffman Encoding — Greedy Tree Building

**Difficulty:** ⭐⭐⭐
**Category:** Greedy Algorithms | Trees | Priority Queues
**OCaml Source:** Classic Huffman algorithm (CLRS Chapter 16)

## Problem Statement

Build a Huffman encoding tree from a list of (character, frequency) pairs using a greedy approach: repeatedly merge the two lowest-frequency nodes until one tree remains. Then traverse the tree to assign binary codes.

## Learning Outcomes

- How to implement a min-heap in Rust using `BinaryHeap<Reverse<...>>` via a local wrapper struct with custom `Ord`
- Why recursive enums require `Box<T>` for heap allocation of child nodes
- How OCaml's pattern-matching-on-list style maps to `match vec.len()` + `remove(0)` in Rust
- The trade-off between the sort-each-round approach (simple, O(n² log n)) and the heap approach (idiomatic, O(n log n))

## OCaml Approach

OCaml sorts a list of trees by frequency at each step, takes the two smallest, merges them, and recurses. The algebraic data type `htree` cleanly unifies leaves and nodes, and pattern matching on the list drives the recursion naturally.

## Rust Approach

The idiomatic Rust version replaces repeated sorting with a `BinaryHeap`. Because `BinaryHeap` needs `Ord`, and adding `Ord` to `HTree` (a recursive enum) is semantically wrong, a local `Item` wrapper struct implements `Ord` by frequency only. The recursive version mirrors OCaml exactly: sort a `Vec`, remove the front two elements, merge, push back.

## Key Differences

1. **ADT vs enum:** OCaml `htree` is a native sum type; Rust uses `enum` with `Box<HTree>` for recursive variants to keep the size finite on the stack.
2. **Priority queue:** OCaml lacks a built-in priority queue so the example sorts each round. Rust's `BinaryHeap` makes the O(n log n) algorithm straightforward once the `Ord` wrapper is in place.
3. **Custom ordering without polluting the type:** Rust requires `Ord` on heap elements. Rather than deriving it on `HTree` (which has no natural order), a local `Item` struct implements `Ord` by frequency — a common Rust pattern for domain-specific ordering.
4. **Ownership of tree nodes:** OCaml manages memory automatically. In Rust, `Box::new(a)` explicitly moves child nodes onto the heap when constructing a `Node`, making ownership transfer explicit.
