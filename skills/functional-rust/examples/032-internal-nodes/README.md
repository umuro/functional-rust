# 032: Internal Nodes

**Difficulty:** ⭐  **Level:** Foundations

Collect the values of all internal nodes — nodes that have at least one child.

## The Problem This Solves

Internal nodes are the decision points in a tree. In a decision tree, they're the questions ("Is it an animal?"). In a file system, they're the directories. In a syntax tree, they're the operators and keywords — the structure, not the data.

Splitting a tree into "leaves" (example 031) and "internal nodes" (this example) lets you analyze the two parts separately. How many decisions vs outcomes? How much structure vs how much data? This decomposition shows up in compression, AI, and compiler design.

## The Intuition

The definition of an internal node is the opposite of a leaf: a node is internal if at least one of its children is *not* `Leaf`. In our tree, a `Node(Leaf, v, Leaf)` is a leaf node — no real children. A `Node(something, v, Leaf)` or `Node(Leaf, v, something)` is internal — at least one real subtree.

In Python:
```python
def is_internal(node):
    return node.left is not None or node.right is not None
```

In Rust, we flip the leaf pattern — match `(Leaf, Leaf)` to skip, everything else is internal:
```rust
match (l.as_ref(), r.as_ref()) {
    (Tree::Leaf, Tree::Leaf) => vec![],  // leaf node — skip it
    _ => { /* this is internal — collect it */ }
}
```

The ordering in the result (left internals, then current node, then right internals) is in-order — internal nodes are visited in left-root-right order.

## How It Works in Rust

```rust
fn internals<T: Clone>(tree: &Tree<T>) -> Vec<T> {
    match tree {
        Tree::Leaf => vec![],  // empty tree → nothing

        Tree::Node(l, v, r) => match (l.as_ref(), r.as_ref()) {
            // Both children are empty → leaf node, not internal
            (Tree::Leaf, Tree::Leaf) => vec![],

            // At least one real child → this is an internal node
            _ => {
                let mut result = internals(l);  // left subtree internals
                result.push(v.clone());          // this node
                result.extend(internals(r));     // right subtree internals
                result
            }
        },
    }
}
```

For this tree:
```
       a         ← internal (has children b, c)
      / \
     b   c       ← b is internal; c has no children → leaf
    / \
   d   e         ← d, e are leaves
```
`internals(&t)` returns `['b', 'a']` — in-order.

## What This Unlocks

- **Tree analysis**: ratio of internal/leaf nodes measures branching factor.
- **Compiler passes**: internal nodes are the operations; traversing them applies transformations.
- **Decision trees in ML**: internal nodes are the split conditions you'd extract for interpretation.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Negating a leaf pattern | `match (l,r) with (Leaf,Leaf) -> []` | Same — match the case to skip, `_` catches the rest |
| In-order collection | `(internals l) @ [v] @ (internals r)` | `result` Vec: push left, push v, extend right |
| Wildcard pattern | `_` | `_` — same syntax |
