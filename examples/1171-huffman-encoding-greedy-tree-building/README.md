# Example 1171: Huffman Encoding — Greedy Tree Building

**Difficulty:** ⭐⭐⭐
**Category:** Trees | Greedy Algorithms | Functional Algorithm
**OCaml Source:** Classic CLRS algorithm — Introduction to Algorithms, Chapter 16

## Problem Statement

Build a Huffman prefix-free binary encoding tree from a list of `(char, frequency)` pairs using a greedy algorithm. Traverse the resulting tree to produce the shortest possible binary code for each character.

## Learning Outcomes

- Modeling recursive tree types in Rust with `Box<T>` for heap-allocated children
- Using `BinaryHeap` with `Reverse` and a newtype wrapper to implement a min-heap
- Replacing OCaml's recursive `let rec go = function` with a Rust inner function
- Understanding why `PartialEq`/`Eq`/`Ord` must be manually implemented for types used in heap ordering that should ignore structural equality

## OCaml Approach

OCaml's algebraic data type `htree` directly models the tree. `build_tree` converts the input to a sorted list of leaves, then calls a recursive helper `go` that merges the two lowest-frequency nodes by re-sorting after every merge. `codes` is a straightforward recursive traversal accumulating a string prefix.

## Rust Approach

Rust represents the tree as an `enum HTree` where `Node` variants hold `Box<HTree>` children to allow recursive heap allocation. The idiomatic solution uses `std::collections::BinaryHeap` with `Reverse<ByFreq>` to create a min-heap ordered by frequency, reducing per-step cost from O(n log n) to O(log n). A second implementation mirrors the OCaml sort-per-step approach using a `Vec` sorted with `sort_by_key`.

## Key Differences

1. **Recursive types:** OCaml's `htree` is directly recursive (GC-managed); Rust requires `Box<HTree>` for heap indirection since the compiler must know the size of every type at compile time.
2. **Priority queue:** OCaml re-sorts the entire list each iteration; Rust's `BinaryHeap` maintains heap order incrementally — O(log n) vs O(n log n) per merge.
3. **Min-heap:** `BinaryHeap` is a max-heap; getting min-heap behavior requires `Reverse<T>` or a custom `Ord` impl that reverses comparison.
4. **Pattern matching ownership:** OCaml destructures `[a; b; rest]` by value freely; Rust must remove elements from a `Vec` explicitly via `remove(0)` to transfer ownership.
