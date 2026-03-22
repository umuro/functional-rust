# Example 1188: Huffman Encoding — Greedy Tree Building

**Difficulty:** ⭐⭐⭐
**Category:** Trees | Greedy Algorithms | Priority Queues
**OCaml Source:** Classic algorithms — Huffman (1952)

## Problem Statement

Build a Huffman prefix-free encoding tree from a list of `(char, frequency)` pairs using a greedy algorithm: repeatedly merge the two lowest-frequency subtrees until one tree remains, then read off the binary codes by path.

## Learning Outcomes

- How to represent recursive algebraic data types (sum types) in Rust using `enum` with `Box<Self>`
- How to implement a min-heap in Rust's `BinaryHeap` (max-heap) via a custom `Ord` on a wrapper struct
- How to mirror OCaml's recursive list-processing pattern with iterative mutation plus `sort_by_key`
- Why Rust requires explicit `Ord`/`PartialOrd` implementations on structs used in priority queues

## OCaml Approach

OCaml uses a variant type `htree` with `Leaf` and `Node` constructors. The `build_tree` function sorts the list on every iteration and recursively processes it until one tree remains. Pattern matching on lists makes the base cases (`[]`, `[t]`, `a :: b :: rest`) naturally exhaustive.

## Rust Approach

The idiomatic Rust version replaces the repeated O(n log n) sort with a `BinaryHeap`, achieving O(n log n) total rather than O(n² log n). Since `BinaryHeap` is a max-heap, a custom `Ord` on a wrapper `Entry` struct inverts the comparison to create a min-heap by frequency. The recursive approach mirrors OCaml directly, using `Vec::remove(0)` and `sort_by_key` each round.

## Key Differences

1. **Recursive types:** OCaml variant types are heap-allocated implicitly; Rust requires `Box<HTree>` to break the infinite-size cycle for `Node`.
2. **Priority queue:** OCaml's `List.sort` on every iteration is O(n² log n); Rust's `BinaryHeap` with custom `Ord` is O(n log n) total.
3. **Heap direction:** Rust's `BinaryHeap` is a max-heap; achieving min-heap semantics requires inverting the comparison in `Ord::cmp`, not wrapping in `std::cmp::Reverse` (which would require `HTree: Ord`).
4. **Pattern exhaustiveness:** OCaml's `function` on lists has natural `[] | [t] | a::b::rest` cases; Rust uses `match trees.len()` with `0 | 1 | _` arms.
