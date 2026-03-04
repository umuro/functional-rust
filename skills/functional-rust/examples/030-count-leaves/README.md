# 030: Count Leaves

**Difficulty:** ⭐  **Level:** Foundations

Count the leaf nodes of a binary tree — nodes that have no children.

## The Problem This Solves

Once you have a tree, you immediately need to ask questions about it. "How many nodes are there?" is `size()`. But a more specific question is: "How many nodes are *at the edges* of the tree?" — leaf nodes, the endpoints of every path from the root.

Leaves are important in practice: in a decision tree (like a game of 20 questions), leaves are the final answers. In a parse tree, leaves are the tokens — the actual text. In a file system tree, leaves are the files (not directories). Being able to count or identify them cleanly is a core skill.

Rust's enum and pattern matching make this feel almost declarative: you describe *what a leaf is*, not *how to find one*.

## The Intuition

A leaf in our tree is a `Node` where both children are `Leaf`. That's the definition. In Python you'd write:
```python
def is_leaf(node):
    return node.left is None and node.right is None
```

In Rust, pattern matching lets you test the shape of a value directly. Instead of separate `if` checks, you match on the structure:
```rust
match (left, right) {
    (Tree::Leaf, Tree::Leaf) => 1,   // both empty — this node IS a leaf
    _ => count_leaves(left) + count_leaves(right),
}
```

The compiler checks that you've handled every case. If you add a new variant to the enum later, every `match` in the codebase that doesn't handle it will fail to compile. This is the "impossible states impossible" guarantee in action.

## How It Works in Rust

```rust
fn count_leaves<T>(tree: &Tree<T>) -> usize {
    match tree {
        // Empty tree: no leaves
        Tree::Leaf => 0,

        // Node: check if both children are empty
        Tree::Node(l, _, r) => match (l.as_ref(), r.as_ref()) {
            (Tree::Leaf, Tree::Leaf) => 1,          // this node is a leaf
            _ => count_leaves(l) + count_leaves(r), // recurse into subtrees
        },
    }
}
```

`l.as_ref()` converts `&Box<Tree<T>>` to `&Tree<T>` so we can pattern match on the inner tree. Boxes dereference transparently in most contexts, but `match` needs a nudge.

**Try it:**
```rust
//        a
//       / \
//      b   c      ← c is a leaf
//     / \
//    d   e        ← d, e are leaves
// Total: 3 leaves
println!("{}", count_leaves(&sample_tree())); // 3
```

## What This Unlocks

- **Decision trees**: count final outcomes at the leaves.
- **Expression trees**: count literal values (leaves) vs operations (internal nodes).
- **Tree metrics**: leaf count vs total size tells you the "branchiness" of a tree.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Pattern matching nested structure | `match (l, r) with (Leaf, Leaf) -> 1` | `match (l.as_ref(), r.as_ref()) { (Tree::Leaf, Tree::Leaf) => 1 }` |
| Unwrapping Box | Automatic | `.as_ref()` to get `&Tree<T>` from `&Box<Tree<T>>` |
| Exhaustiveness check | Compiler error on missing arms | Same — Rust enforces all arms are covered |
