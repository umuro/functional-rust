# Example 1156: Huffman Encoding — Greedy Tree Building

**Difficulty:** ⭐⭐⭐
**Category:** Trees | Greedy Algorithms | Priority Queues
**OCaml Source:** Classic data structures — Huffman (1952) encoding algorithm

## Problem Statement

Build a Huffman encoding tree from a list of (character, frequency) pairs using a greedy algorithm, then derive variable-length binary codes from the resulting tree. Characters with higher frequencies receive shorter codes, minimizing total encoded length.

## Learning Outcomes

- Recursive algebraic data types in Rust using `enum` with `Box<Self>` for tree nodes
- Implementing `Ord`/`PartialOrd` on a wrapper type to create a min-heap with `BinaryHeap`
- How Rust's `BinaryHeap` (max-heap by default) becomes a min-heap via reversed comparison
- The trade-off between `BinaryHeap` (O(n log n)) and sorted-Vec (O(n²)) approaches, and when each mirrors the OCaml idiom

## OCaml Approach

OCaml uses a recursive variant type `htree` with `Leaf` and `Node` constructors, directly mirroring the mathematical definition. The `build_tree` function repeatedly sorts a list and merges the two lowest-frequency trees, relying on OCaml's garbage collector for the tree allocation. Code generation is a straightforward structural recursion that accumulates a string prefix.

## Rust Approach

Rust represents the tree as a `pub enum HTree` with `Box<HTree>` for child references — the `Box` is required because recursive types need indirection for a known stack size. The idiomatic solution uses `BinaryHeap<MinFreq>` where `MinFreq` wraps `Box<HTree>` and reverses comparison to get min-heap behavior. The functional solution uses a sorted `Vec` with `partition_point` for O(log n) insertion, closely paralleling the OCaml `List.sort` approach.

## Key Differences

1. **Recursive types:** OCaml's `htree` is self-referential by default; Rust requires `Box<HTree>` to break the cycle and place children on the heap.
2. **Priority queue:** OCaml sorts the whole list on each merge step; Rust's idiomatic version uses `BinaryHeap` with a reversed `Ord` impl for O(log n) per operation.
3. **Memory management:** OCaml's GC handles tree allocation automatically; Rust transfers ownership through `Box`, making the tree's lifetime explicit and statically verified.
4. **Pattern matching:** Both languages use exhaustive match on the tree variant, but Rust's borrow checker ensures references into the tree are valid without needing a GC.

## Exercises

1. Add a `verify` function that checks a Huffman code table for prefix-free property — no code is a prefix of another code.
2. Implement run-length encoding as a pre-processing step before Huffman coding and compare compression ratios on highly repetitive vs. random inputs.
3. Extend to a multi-symbol (byte-pair) Huffman variant: compute frequencies of adjacent byte pairs, encode the most frequent pairs as single symbols, then apply standard Huffman — and compare compression ratios.
