# Example 1203: Huffman Encoding — Greedy Tree Building

**Difficulty:** ⭐⭐⭐  
**Category:** Trees | Greedy Algorithms | Priority Queues  
**OCaml Source:** Classic algorithm — standard functional programming exercise

## Problem Statement

Given a list of `(symbol, frequency)` pairs, build a Huffman tree using the greedy algorithm: repeatedly merge the two lowest-frequency trees into a combined node until a single tree remains. Then extract the binary code for each symbol by traversing the tree.

## Learning Outcomes

- How to model algebraic data types (ADTs) in Rust using `enum` with variants holding data
- How to implement a min-heap in Rust with `BinaryHeap` by reversing the `Ord` implementation
- How `Box<T>` enables recursive data structures (trees) in Rust where OCaml uses implicit heap allocation
- The difference between OCaml's structural pattern matching and Rust's exhaustive `match`

## OCaml Approach

OCaml defines `htree` as a variant type with `Leaf` and `Node` constructors and uses `List.sort` at each merge step to maintain order. The `go` recursive function processes a sorted list, always taking the first two (lowest-frequency) items. Code extraction uses tail-recursive pattern matching with string concatenation.

## Rust Approach

Rust models the same ADT with `enum HTree { Leaf, Node }` using `Box` for recursive heap allocation. The idiomatic version uses `std::collections::BinaryHeap` with a custom `Ord` implementation (reversed for min-heap behavior), giving O(n log n) complexity. The functional version mirrors OCaml's `List.sort` approach with `Vec::sort_by_key`. Both versions use recursive tree traversal for code extraction.

## Key Differences

1. **Heap allocation:** OCaml allocates all values on the heap implicitly; Rust requires explicit `Box<HTree>` for recursive enum variants to have a known size.
2. **Priority queue:** OCaml uses repeated `List.sort` (O(n²) in the worst case); idiomatic Rust uses `BinaryHeap` with a reversed `Ord` impl for O(n log n).
3. **Pattern matching:** OCaml's `match t with Leaf(_,f) -> f | Node(_,_,f) -> f` maps directly to Rust's `match self { HTree::Leaf(_, f) | HTree::Node(_, _, f) => *f }` — `|` works in both.
4. **Ownership:** Rust's `remove(0)` on a `Vec` transfers ownership of the element, enabling move into a new `Node` without cloning; OCaml never needs to think about this.
