# 037: Tree Preorder

**Difficulty:** ⭐  **Level:** Foundations

Generate preorder and inorder traversals, then reconstruct the original tree from both sequences.

## The Problem This Solves

Traversal order is fundamental to what you can do with a tree. Preorder (root → left → right) visits each node before its children — useful for copying trees, serializing them, and prefix notation in expression trees. Inorder (left → root → right) visits nodes in sorted order for binary search trees, and gives the natural left-to-right reading order.

The killer application: given *both* sequences together, you can reconstruct the exact original tree. This is used in database recovery, compiler symbol tables, and anywhere you need to transmit a tree structure efficiently.

## The Intuition

Think of traversal orders as different ways to "read" a tree:

- **Preorder**: "Visit me first, then my left subtree, then my right subtree." Like reading a directory listing where you see the folder name before its contents.
- **Inorder**: "Visit my left subtree, then me, then my right subtree." For a binary search tree, this reads values in sorted order.

In any language the pattern is the same — just change which step comes first:

```python
def preorder(node):   # root, left, right
    if not node: return []
    return [node.val] + preorder(node.left) + preorder(node.right)

def inorder(node):    # left, root, right
    if not node: return []
    return inorder(node.left) + [node.val] + inorder(node.right)
```

**Reconstruction insight**: In preorder, the first element is always the root. Find that root in the inorder sequence — everything to its left is the left subtree, everything to its right is the right subtree. Recurse. This uniquely identifies the tree.

## How It Works in Rust

```rust
fn preorder<T: Clone>(tree: &Tree<T>) -> Vec<T> {
    match tree {
        Tree::Leaf => vec![],
        Tree::Node(l, v, r) => {
            let mut result = vec![v.clone()];  // root first
            result.extend(preorder(l));         // then left
            result.extend(preorder(r));         // then right
            result
        }
    }
}

fn inorder<T: Clone>(tree: &Tree<T>) -> Vec<T> {
    match tree {
        Tree::Leaf => vec![],
        Tree::Node(l, v, r) => {
            let mut result = inorder(l);   // left first
            result.push(v.clone());         // then root
            result.extend(inorder(r));      // then right
            result
        }
    }
}

fn build_from_preorder_inorder<T: PartialEq + Clone>(
    pre: &[T],
    ino: &[T],
) -> Tree<T> {
    if pre.is_empty() { return Tree::leaf(); }

    let root = pre[0].clone();
    // Find root's position in inorder → splits left/right subtrees
    let split = ino.iter().position(|x| x == &root).unwrap();

    Tree::node(
        build_from_preorder_inorder(&pre[1..1 + split], &ino[..split]),
        root,
        build_from_preorder_inorder(&pre[1 + split..], &ino[split + 1..]),
    )
}
```

For the sample tree `a(b(d,e),c)`:
- Preorder:  `[a, b, d, e, c]`
- Inorder:   `[d, b, e, a, c]`
- Reconstruct: root=`a`, left inorder=`[d,b,e]`, right inorder=`[c]` → recurse ✓

Slice syntax `&pre[1..1+split]` is Rust's way of taking a subarray — safe, bounds-checked, no allocation.

## What This Unlocks

- **Tree serialization**: two traversal sequences uniquely encode a tree structure.
- **Expression evaluation**: preorder = prefix notation (Polish notation); inorder = infix (standard math).
- **Database recovery**: B-tree serialization for crash recovery often uses traversal sequences.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Preorder list | `[v] @ preorder l @ preorder r` | `vec![v]` then `extend` |
| Slice/subarray | `Array.sub` or list slice | `&slice[start..end]` |
| Find in sequence | `List.find_index` | `.iter().position(...)` |
| Trait bounds for equality | `eq` typeclass | `T: PartialEq` |
