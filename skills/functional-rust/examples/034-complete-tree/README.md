# 034: Complete Binary Tree

**Difficulty:** ⭐  **Level:** Foundations

Construct a complete binary tree with exactly n nodes, and verify whether a tree is complete.

## The Problem This Solves

A *complete* binary tree fills levels strictly left to right — no gaps. This specific shape is important: binary heaps (the data structure behind priority queues) are complete binary trees stored in arrays. The heap you use when you call `sort()` in most languages is built on this idea.

Constructing a complete binary tree from scratch and verifying completeness are both useful skills. The construction algorithm here uses a clever property: if you number the nodes starting at 1, node `i` always has its left child at `2i` and right child at `2i+1`. This is the same indexing trick that lets heaps work as flat arrays with no pointers at all.

## The Intuition

Picture filling a tree row by row, left to right:
```
Level 1:        1
Level 2:      2   3
Level 3:    4  5 6  7
Level 4:  8  9 ...
```

Node `1` is the root. Its children are `2` and `3`. Node `2`'s children are `4` and `5`. Node `k`'s children are always at `2k` and `2k+1`. 

To build a tree of `n` nodes: start at position `1`. If `pos > n`, it's a `Leaf`. Otherwise it's a `Node` with children at `2*pos` and `2*pos+1`. The recursion naturally stops building once positions exceed `n`.

Verification is the reverse: for a tree of size `n`, every node's position must be `<= n`.

## How It Works in Rust

```rust
/// Build a complete binary tree with n nodes (values 1..=n).
fn complete_binary_tree(n: usize) -> Tree<usize> {
    build(1, n)
}

fn build(pos: usize, n: usize) -> Tree<usize> {
    if pos > n {
        Tree::leaf()  // position exceeds count → empty
    } else {
        Tree::node(
            build(2 * pos, n),     // left child
            pos,                    // this node's value = its position
            build(2 * pos + 1, n), // right child
        )
    }
}

/// Check if a tree is complete using the same indexing property.
fn is_complete<T>(tree: &Tree<T>) -> bool {
    fn check<T>(tree: &Tree<T>, pos: usize, n: usize) -> bool {
        match tree {
            Tree::Leaf => true,
            Tree::Node(l, _, r) => {
                pos <= n                         // position in range
                && check(l, 2 * pos, n)          // left subtree ok
                && check(r, 2 * pos + 1, n)      // right subtree ok
            }
        }
    }
    let n = /* size of tree */;
    check(tree, 1, n)
}
```

**Results:**
```
n=0: empty tree, complete ✓
n=1: single root, complete ✓
n=3: full level 2, complete ✓
n=4: level 3 starts on the left, complete ✓
n=7: full 3 levels, complete ✓
```

## What This Unlocks

- **Binary heaps**: heap = complete binary tree stored as an array using this exact indexing.
- **Priority queues**: `BinaryHeap` in Rust's standard library is built on this structure.
- **Balanced tree checks**: completeness is one measure of balance; AVL and red-black trees use related ideas.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive constructor | `build pos n = if pos > n then Leaf else Node(...)` | Same logic, explicit `Box::new` in `node()` helper |
| Index arithmetic | `2*pos`, `2*pos+1` | Same — `usize` arithmetic |
| Nested functions | Local `let rec` | Nested `fn` inside function body |
