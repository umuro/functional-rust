# Example 1149: Huffman Encoding — Greedy Tree Building

**Difficulty:** ⭐⭐⭐
**Category:** Trees | Greedy Algorithms | Priority Queues
**OCaml Source:** Classic greedy algorithm — Cormen et al. CLRS §16.3

## Problem Statement

Build a Huffman prefix-free binary encoding tree from a list of (symbol, frequency) pairs using a greedy strategy: repeatedly merge the two lowest-frequency nodes until one tree remains, then read off binary codes by traversing left (0) and right (1).

## Learning Outcomes

- How to implement a min-heap in Rust using `BinaryHeap` with a custom `Ord` wrapper
- Why `Box<HTree>` is required for recursive enum variants in Rust
- How or-patterns (`Leaf(_, f) | Node(_, _, f)`) collapse match arms
- The OCaml→Rust translation of recursive list processing into a priority-queue loop

## OCaml Approach

OCaml uses an algebraic data type (`type htree = Leaf of char * int | Node of htree * htree * int`) and a recursive function that sorts the list each iteration, takes the two smallest nodes, merges them, and recurses. Pattern matching on the list head makes the termination condition explicit.

## Rust Approach

The idiomatic Rust version wraps `HTree` in a `MinHeapNode` newtype that reverses `BinaryHeap`'s default max-ordering, turning it into a min-heap. Each iteration pops two minimum-frequency nodes, merges them, and pushes the result back — O(log n) per step instead of O(n log n) for list re-sorting. The functional variant preserves the OCaml sort-then-recurse style for clarity.

## Key Differences

1. **Recursive types:** OCaml's `htree` is naturally recursive; Rust requires `Box<HTree>` to give the inner node a known size on the stack.
2. **Priority queue:** OCaml uses `List.sort` (O(n log n) per merge); idiomatic Rust uses `BinaryHeap` (O(log n) per merge) with a reversed-comparison wrapper.
3. **Or-patterns:** `match self { Leaf(_, f) | Node(_, _, f) => *f }` is a direct parallel to OCaml's `match t with Leaf (_,f) -> f | Node (_,_,f) -> f`, available since Rust 1.53.
4. **String concatenation:** OCaml uses `^` for string append; Rust uses `prefix.to_string() + "0"` or `format!`, returning owned `String` from each recursive call.
