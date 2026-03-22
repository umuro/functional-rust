# Example 1187: Huffman Encoding — Greedy Tree Building

**Difficulty:** ⭐⭐⭐
**Category:** Algorithms
**OCaml Source:** Classic algorithm, OCaml textbook exercise

## Problem Statement

Build an optimal prefix-free binary encoding tree from a table of character frequencies. The Huffman algorithm repeatedly merges the two lowest-frequency nodes until only one tree remains, producing a tree where high-frequency characters get short codes and rare characters get long codes. This greedy strategy provably produces the minimum expected code length.

## Learning Outcomes

- How to implement a greedy algorithm using a priority queue (min-heap) in Rust with `BinaryHeap<Reverse<...>>`
- Why `BinaryHeap` is a max-heap by default and how `std::cmp::Reverse` converts it to a min-heap without a custom `Ord` implementation
- How recursive tree traversal to generate prefix codes mirrors the OCaml pattern-matching style
- How to use a tie-breaking counter to make a min-heap with tuple keys produce deterministic ordering

## OCaml Approach

OCaml's implementation uses a `list` as a sorted priority queue: after each merge, the new node is inserted and the list is re-sorted with `List.sort`. This approach is elegant and concise but O(n log n) per step, giving O(n² log n) overall. The tree type is an algebraic data type with `Leaf` and `Node` variants, and code generation is a recursive function that accumulates the bit string as a prefix string.

## Rust Approach

The idiomatic Rust version uses `BinaryHeap<Reverse<(u32, usize, HTree)>>` as a proper min-heap, giving O(n log n) overall. The tuple `(freq, counter, node)` provides two levels of ordering: frequency first (min), then insertion order (FIFO tie-breaking via the counter). The recursive approach mirrors the OCaml code structurally — sorting a `Vec<HTree>` after each merge — and is useful when the OCaml translation is the primary goal.

## Key Differences

1. **Priority queue:** OCaml re-sorts a list after each step (O(n log n) per step); Rust uses a `BinaryHeap` with `Reverse` for true O(log n) per step.
2. **Max-heap to min-heap:** Rust's `BinaryHeap` is a max-heap; wrapping entries in `std::cmp::Reverse` inverts the comparison to get min-heap semantics without a custom `Ord` impl.
3. **Recursive type representation:** Both languages use an algebraic data type with leaf and node variants. OCaml heap-allocates all constructors automatically; Rust requires explicit `Box<HTree>` to create a recursive type with finite size.
4. **Tie-breaking:** OCaml's `compare` on tagged values has implicit structural tie-breaking; Rust's tuple key `(freq, counter, node)` adds an explicit insertion counter to break frequency ties deterministically.
