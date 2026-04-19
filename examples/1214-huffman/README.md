# Example 1214: Huffman Coding

**Difficulty:** ⭐⭐⭐
**Category:** Trees & Greedy Algorithms
**OCaml Source:** classic Huffman coding exercise (CLRS §16.3)

## Problem Statement

Given a list of symbols and their frequencies, build a Huffman tree and
derive an optimal prefix-free binary code for each symbol.

## Learning Outcomes

- Modelling recursive ADTs in Rust with `enum` + `Box`
- Using `BinaryHeap` plus a wrapper type to get a min-priority queue
- Why `HTree` deliberately isn't `Ord` (no total ordering on trees) and how
  to introduce a deterministic tiebreak
- Translating an OCaml sort/merge loop into an iterator-friendly Rust version

## OCaml Approach

OCaml defines `type htree = Leaf of char * int | Node of htree * htree * int`
and keeps a list sorted by frequency.  Each step: take the two smallest,
merge them into a `Node` whose frequency is the sum, re-sort, repeat until
one tree remains.  Matches naturally on `| [t] -> t | a :: b :: rest -> ...`.

## Rust Approach

Mirror the ADT with an `enum` whose recursive arms are boxed.  For the
priority queue, wrap each subtree in a `HeapEntry` that flips the default
max-heap into a min-heap.  A monotonically-increasing `tiebreak` counter
keeps pops deterministic when frequencies collide, which the OCaml version
got for free from list order.  Code extraction is a standard in-place DFS
that pushes/pops `'0'` / `'1'` on a shared `String` buffer.

## Key Differences

1. **Recursive data:** OCaml allows direct `Node of htree * htree * int`; Rust
   needs `Box<HTree>` to give the enum a known size.
2. **Priority queue:** OCaml re-sorts a list each round (O(n² log n)); Rust's
   `BinaryHeap` gives O(n log n) with the standard lazy-rebalance idiom.
3. **Ordering:** OCaml's `compare` orders any two values structurally; Rust
   requires you to spell out what "smaller" means for trees — we only compare
   frequencies, so we keep `Ord` on a wrapper, not on `HTree` itself.
4. **String building:** OCaml uses persistent `prefix ^ "0"` concatenation;
   Rust pushes/pops a single `String` to avoid `O(n²)` allocation during the
   walk.
