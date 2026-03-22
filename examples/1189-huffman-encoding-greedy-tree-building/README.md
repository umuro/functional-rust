# Example 1189: Huffman Encoding — Greedy Tree Building

**Difficulty:** ⭐⭐⭐
**Category:** Trees | Greedy Algorithms | Priority Queues
**OCaml Source:** Classic data compression algorithm

## Problem Statement

Build an optimal Huffman prefix code tree from a list of (character, frequency) pairs using a greedy algorithm: repeatedly merge the two lowest-frequency trees until one tree remains, then extract codes by traversing left ("0") and right ("1") branches.

## Learning Outcomes

- Modeling recursive algebraic data types with Rust `enum` (direct parallel to OCaml's `type htree`)
- Using `BinaryHeap` as a min-heap via a newtype wrapper that reverses `Ord`
- Recursive tree traversal with string prefix accumulation in Rust
- Implementing custom `Ord`/`PartialOrd` for a priority queue ordering

## OCaml Approach

OCaml uses a sorted list and a recursive `go` function: sort the list, take the two front elements, merge them, re-sort, recurse. The `type htree` algebraic data type maps directly onto the problem. Pattern matching on the list shape (`[t]`, `a :: b :: rest`, `[]`) makes the algorithm readable as a spec.

## Rust Approach

Rust provides two natural implementations: an idiomatic `BinaryHeap`-based version (O(n log n), no re-sorting) and a functional Vec-based version that mirrors OCaml exactly. The `enum HTree` maps one-to-one with the OCaml type. Because `BinaryHeap` is a max-heap, a newtype `MinFreq` reverses the comparison to achieve min-heap semantics.

## Key Differences

1. **Data types:** OCaml's `type htree = Leaf of char * int | Node of htree * htree * int` maps to Rust's `enum HTree { Leaf { ch, freq }, Node { left, right, freq } }` — struct variants replace tuples for clarity.
2. **Priority queue:** OCaml re-sorts a list after each merge; idiomatic Rust uses `BinaryHeap` with a custom `Ord` wrapper for O(log n) insertion instead of O(n log n) re-sort.
3. **Ownership:** Rust requires `Box<HTree>` for recursive enum variants; OCaml handles this transparently via GC heap allocation.
4. **Pattern matching:** OCaml matches on list shape directly (`[t]`, `a :: b :: rest`); Rust matches on `trees.len()` and uses `.remove(0)` to destructure.
