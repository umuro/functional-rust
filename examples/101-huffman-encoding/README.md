# Example 101: Huffman Encoding

**Difficulty:** ⭐⭐⭐
**Category:** Trees & Greedy Algorithms
**OCaml Source:** Classic data-structures exercise; see CLRS §16.3

## Problem Statement

Build an optimal prefix-free binary code for a set of characters with given
frequencies using Huffman's greedy algorithm: repeatedly merge the two
lowest-frequency nodes until a single tree remains, then assign "0"/"1" on
left/right branches to derive each character's code.

## Learning Outcomes

- How to turn an OCaml algebraic type (`type htree = Leaf | Node`) into an owned
  Rust enum with `Box<HTree>` children
- Using `BinaryHeap<Reverse<W>>` for a min-heap without unsafe or external crates
- Why a custom `Ord` wrapper (`FreqOrd`) keeps ordering logic separate from the
  data type
- How OCaml's list-sort loop maps to a sorted `Vec` with `remove(0)` in Rust,
  and why the BinaryHeap version is the idiomatic upgrade

## OCaml Approach

OCaml represents the tree as a sum type and builds it by sorting a list on each
iteration.  `List.sort` re-runs on every merge step, giving O(n² log n) total
work, but the code is compact and transparent.  Code generation is a natural
structural recursion over the tree, concatenating prefix strings.

## Rust Approach

The idiomatic Rust solution wraps each `HTree` in a `FreqOrd` newtype that
implements `Ord` by frequency, then uses `BinaryHeap<Reverse<FreqOrd>>` to get
O(log n) insert/remove instead of O(n) re-sort.  The recursive solution mirrors
OCaml's list-sort style exactly, trading efficiency for directness.  Both share
the same `codes` traversal function.

## Key Differences

1. **Recursive types:** OCaml `type htree = … | Node of htree * htree * int`
   is natively recursive; Rust requires `Box<HTree>` to give the enum a known
   size on the stack.
2. **Min-heap:** OCaml's `List.sort` re-sorts each step; Rust offers
   `BinaryHeap` (max-heap) which becomes a min-heap via `Reverse<W>`.
3. **Custom ordering:** OCaml pattern-matches a comparison function inline;
   Rust externalises ordering into a newtype (`FreqOrd`) implementing the `Ord`
   trait, keeping `HTree` itself free of ordering concerns.
4. **String building:** OCaml uses `^` (string concat) naturally in a recursive
   call; Rust uses `format!("{prefix}0")` passing an owned `String` down the
   call stack to avoid repeated allocations at the top level.
