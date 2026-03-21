# Example 1143: Huffman Encoding — Greedy Tree Building
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Given a list of `(symbol, frequency)` pairs, build a Huffman tree by repeatedly merging the two lowest-frequency trees until one tree remains. Then traverse the tree to assign a binary prefix code to each symbol.

## Learning Outcomes

- Modelling algebraic data types as Rust `enum` with `Box` for recursive structure
- Using `BinaryHeap` with a `Reverse` wrapper to simulate a min-heap
- Implementing `Ord`/`PartialOrd` on a newtype to drive priority-queue ordering
- Translating OCaml's `List.sort`-based greedy loop into a heap-based O(n log n) algorithm

## OCaml Approach

OCaml uses an algebraic type `htree = Leaf of char * int | Node of htree * htree * int` and an inner recursive function `go` that sorts the list on each iteration to find the two smallest trees, merges them, and recurses. The code extraction uses pattern-matched accumulation via string concatenation.

## Rust Approach

Rust mirrors the same `HTree` enum with `Box<HTree>` children. The idiomatic version replaces the repeated-sort with a `BinaryHeap<MinTree>` where `MinTree` is a newtype implementing `Ord` by frequency. Code generation uses a recursive helper that threads a `String` prefix through the tree. The recursive version preserves the OCaml `go` structure using `Vec::sort_by_key` on each step.

## Key Differences

1. **Recursive types:** OCaml values are heap-allocated automatically; Rust needs explicit `Box<HTree>` to break the cycle.
2. **Priority queue:** OCaml re-sorts the full list every iteration; Rust uses `BinaryHeap` for O(log n) insert/pop.
3. **Ordering:** OCaml uses a comparison function passed to `List.sort`; Rust requires `Ord` implemented on the element type (via newtype).
4. **String accumulation:** OCaml uses `^` for string concatenation; Rust uses `format!("{prefix}0")` on owned `String`.

## Exercises

1. Implement the Huffman decoder: given the encoding tree and a string of bits, reconstruct the original text.
2. Extend the encoder to serialize the Huffman tree structure into a compact binary header so that the decoder does not need the tree separately.
3. Implement adaptive Huffman coding: build the tree incrementally as you encode each symbol rather than in a separate pre-pass, and compare compression ratio with the static variant.
