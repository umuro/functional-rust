📖 **[View on hightechmind.io →](https://hightechmind.io/rust/031-collect-leaves)**

---

# 031 — Collect the Leaves of a Binary Tree in a List
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Collecting all leaf values into a list (OCaml 99 Problems #31) extends the leaf-counting pattern: instead of incrementing a counter, we collect the actual values. This is the leaf traversal — the first step in algorithms like Huffman decoding, expression evaluation (leaves are operands), and tree serialization.

The problem introduces a key pattern in functional tree processing: building a result list by appending contributions from left and right subtrees. The naive approach using `@` (OCaml) or `extend` (Rust) is O(n·d) where d is depth. The efficient approach uses an accumulator or difference lists to achieve O(n).

## Learning Outcomes

- Collect values from tree leaves using structural recursion
- Return values in left-to-right order (inorder leaf traversal)
- Use `extend` to combine results from left and right subtrees
- Understand the efficiency difference between appending and accumulator-based collection
- Recognize that leaf collection is a degenerate form of tree flattening

## Rust Application

`collect_leaves<T: Clone>` returns a `Vec<T>`. Base case: `Tree::Leaf` returns `vec![]` (our leaf type has no value). For a tree where leaves carry values (`Node(val, Leaf, Leaf)` would be a leaf node), we collect the value. Recursive case for `Node`: `let mut left_leaves = collect_leaves(l); left_leaves.extend(collect_leaves(r)); left_leaves`. This builds the list in left-to-right order. The `T: Clone` bound is needed to clone values into the result.

## OCaml Approach

OCaml's version: `let rec leaves = function | Leaf -> [] | Node (x, Leaf, Leaf) -> [x] | Node (_, l, r) -> leaves l @ leaves r`. The middle case identifies a node with two leaf children (a proper leaf node in a "full" tree where only leaf nodes carry values). The `@` concatenation builds the list. For efficiency, use accumulator style: `let rec leaves_acc acc = function | Leaf -> acc | Node (x, Leaf, Leaf) -> x :: acc | Node (_, l, r) -> leaves_acc (leaves_acc acc r) l`.

## Key Differences

1. **Value at leaves**: In our `Tree<T>` type, the value is at `Node`, not at `Leaf`. OCaml 99 Problems uses the same type: value is at `Node`. "Leaf nodes" are `Node(x, Leaf, Leaf)` — nodes with two null children.
2. **`@` vs extend**: OCaml's `leaves l @ leaves r` copies the left result — O(|left|). Rust's approach of extending a mutable Vec is O(|right|). Both are O(n) total.
3. **Accumulator efficiency**: OCaml's accumulator version `leaves_acc (leaves_acc acc r) l` processes right then left, building in reverse. Pass `List.rev` at the end or process left then right.
4. **Difference lists**: For maximum efficiency when collecting from many nodes, use difference lists (example 081) or Rust's `Vec::extend` which is amortized O(1) per element.

## Exercises

1. **Internal nodes**: Write `internal_nodes<T: Clone>(tree: &Tree<T>) -> Vec<T>` that collects values from nodes that are not leaves (nodes with at least one non-Leaf child).
2. **Nodes at depth**: Write `at_depth<T: Clone>(tree: &Tree<T>, d: usize) -> Vec<T>` that collects all node values at exactly depth d (root is depth 0).
3. **Fringe equality**: Two trees have the same fringe if they have the same sequence of leaf values. Write `same_fringe<T: Clone + PartialEq>(t1: &Tree<T>, t2: &Tree<T>) -> bool`. Can you do it without materializing both fringes?
