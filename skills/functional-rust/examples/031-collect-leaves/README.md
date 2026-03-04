# 031: Collect Leaves

**Difficulty:** ⭐  **Level:** Foundations

Collect the values of all leaf nodes into a list, left to right.

## The Problem This Solves

Counting leaves (example 030) tells you *how many* there are. Collecting them tells you *what they are*. This is the difference between knowing a file system has 42 files and actually getting their names.

The leaf sequence of a tree has a specific order: left subtree first, then right. This left-to-right ordering is meaningful — in a syntax tree it's the order tokens appear in source code. In a Huffman encoding tree, the leaves are the symbols. Getting them in the right order matters.

This example also shows off a bonus: Rust's `Vec` makes accumulation natural, and the same logic can be rewritten iteratively using an explicit stack — useful when trees are very deep and stack overflow is a concern.

## The Intuition

The recursive logic mirrors how you'd think about it out loud: "The leaves of this tree are: all the leaves in the left subtree, followed by all the leaves in the right subtree — unless this node itself *is* a leaf, in which case it's just `[value]`."

In Python:
```python
def leaves(tree):
    if tree is None: return []
    if tree.left is None and tree.right is None: return [tree.val]
    return leaves(tree.left) + leaves(tree.right)
```

In Rust, pattern matching makes the "is this a leaf?" check structural:
```rust
match (l.as_ref(), r.as_ref()) {
    (Tree::Leaf, Tree::Leaf) => vec![v.clone()],  // leaf node
    _ => { /* recurse */ }
}
```

The `.clone()` is explicit — Rust doesn't copy values silently. You know exactly when data is being duplicated.

## How It Works in Rust

```rust
fn leaves<T: Clone>(tree: &Tree<T>) -> Vec<T> {
    match tree {
        Tree::Leaf => vec![],  // empty tree → empty list

        Tree::Node(l, v, r) => match (l.as_ref(), r.as_ref()) {
            // Both children empty → this node IS a leaf
            (Tree::Leaf, Tree::Leaf) => vec![v.clone()],

            // Otherwise → collect from subtrees in order
            _ => {
                let mut result = leaves(l);    // left leaves first
                result.extend(leaves(r));       // then right leaves
                result
            }
        },
    }
}
```

**Iterative version** (avoids stack overflow on very deep trees):
```rust
fn leaves_iter<T: Clone>(tree: &Tree<T>) -> Vec<T> {
    let mut stack = vec![tree];
    let mut result = Vec::new();
    while let Some(t) = stack.pop() {
        if let Tree::Node(l, v, r) = t {
            match (l.as_ref(), r.as_ref()) {
                (Tree::Leaf, Tree::Leaf) => result.push(v.clone()),
                _ => {
                    stack.push(r);  // right pushed first → left processed first
                    stack.push(l);
                }
            }
        }
    }
    result
}
```

Both produce the same output — the tests verify they agree.

## What This Unlocks

- **Tokenization**: the leaves of a parse tree are the tokens, in source order.
- **Data extraction**: collect all values at the "edges" of any hierarchical structure.
- **Serialization prep**: leaf sequences are often the output of encoding schemes (Huffman, etc.).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Building result list | `left_leaves @ right_leaves` (list append) | `result.extend(leaves(r))` (Vec append) |
| Explicit clone | Not needed (GC / functional copy) | `v.clone()` — always explicit in Rust |
| Iterative with stack | Tail-call optimization common | Manual stack with `Vec` |
