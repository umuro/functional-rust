# Example 1207: Huffman Encoding — Greedy Tree Building

**Difficulty:** ⭐⭐⭐  
**Category:** Trees | Greedy Algorithms | Pattern Matching  
**OCaml Source:** Classic Huffman encoding algorithm

## Problem Statement

Build an optimal prefix-free binary code tree from a list of `(symbol, frequency)` pairs using a greedy strategy: repeatedly merge the two lowest-frequency trees until a single tree remains.

## Learning Outcomes

- How Rust enums model OCaml algebraic data types with `Box<T>` for recursive variants
- Using `BinaryHeap` as a min-heap via a newtype that reverses the `Ord` implementation
- Translating OCaml's `List.sort` + recursive inner function into idiomatic Rust Vec + recursion
- Why Rust needs `Box<HTree>` inside `Node` while OCaml allocates subtrees implicitly

## OCaml Approach

OCaml defines `htree` as an algebraic type with `Leaf` and `Node` constructors. `build_tree` creates leaf nodes, sorts them by frequency, then the inner `go` function repeatedly picks the two cheapest, merges them into a `Node`, re-sorts, and recurses. Code extraction walks the tree accumulating a `"0"`/`"1"` prefix string.

## Rust Approach

Rust mirrors the ADT with an `enum HTree`. The idiomatic solution uses `std::collections::BinaryHeap` with a `MinTree` newtype that reverses `Ord` so the heap acts as a min-priority queue — O(n log n) total. The recursive solution keeps the OCaml structure: `Vec<HTree>` sorted by frequency with the two cheapest always at the front, recursing until one tree remains.

## Key Differences

1. **Recursive types:** OCaml allocates `Node` children on the heap automatically; Rust requires `Box<HTree>` to give the compiler a known size.
2. **Priority queue:** OCaml's stdlib has no built-in heap, so the OCaml code re-sorts on every merge (O(n² log n)). Rust's `BinaryHeap` enables true O(n log n).
3. **Error handling:** OCaml raises `Failure "empty"` on an empty input; Rust returns `Option<HTree>` — no exceptions, enforced by the type system.
4. **Pattern matching:** Both languages use pattern matching on the tree constructor, but Rust must bind via `Box` derefs (`HTree::Node(left, right, _)` where `left`/`right` are `Box<HTree>`).
