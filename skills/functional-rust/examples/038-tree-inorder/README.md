# 038: Tree Inorder

**Difficulty:** ⭐  **Level:** Foundations

Generate inorder and postorder traversals, then reconstruct the original tree from both sequences.

## The Problem This Solves

Example 037 showed reconstruction from preorder + inorder. This example completes the picture with postorder traversal and reconstruction from inorder + postorder — the third combination you'll encounter in interviews, compiler courses, and tree-processing algorithms.

Postorder (left → right → root) visits children before parents. This order is used when you need to process children before acting on the parent: deleting a tree (free children before freeing parent), evaluating an expression tree (compute sub-expressions before the operator), and bottom-up compilation passes.

Understanding all three traversal orders and their reconstruction properties gives you a complete toolkit for working with binary trees.

## The Intuition

The three traversal orders differ only in when you visit the root relative to the subtrees:

| Name | Order | "Root position in sequence" |
|------|-------|-----------------------------|
| Preorder | root → left → right | **First** element |
| Inorder | left → root → right | **Middle** element |
| Postorder | left → right → root | **Last** element |

This "root position" property is the key to reconstruction. With postorder + inorder:
- **Root** = last element of postorder sequence
- **Split** = find root in inorder → left side is left subtree, right side is right subtree
- Count elements in left inorder slice = size of left subtree in postorder
- Recurse on both sides

In Python:
```python
def build_from_inorder_postorder(ino, post):
    if not post: return None
    root = post[-1]              # last element is root
    split = ino.index(root)      # find root in inorder
    return Tree(
        build_from_inorder_postorder(ino[:split], post[:split]),
        root,
        build_from_inorder_postorder(ino[split+1:], post[split:-1])
    )
```

## How It Works in Rust

```rust
fn postorder<T: Clone>(tree: &Tree<T>) -> Vec<T> {
    match tree {
        Tree::Leaf => vec![],
        Tree::Node(l, v, r) => {
            let mut result = postorder(l);  // left first
            result.extend(postorder(r));     // then right
            result.push(v.clone());          // root last
            result
        }
    }
}

fn build_from_inorder_postorder<T: PartialEq + Clone>(
    ino: &[T],
    post: &[T],
) -> Tree<T> {
    if ino.is_empty() || post.is_empty() { return Tree::leaf(); }

    let root = post.last().unwrap().clone();  // last = root
    let split = ino.iter().position(|x| x == &root).unwrap();

    Tree::node(
        build_from_inorder_postorder(&ino[..split], &post[..split]),
        root,
        build_from_inorder_postorder(&ino[split + 1..], &post[split..post.len() - 1]),
    )
}
```

For the sample tree:
- Inorder:   `[d, b, e, a, c]`
- Postorder: `[d, e, b, c, a]`
- Root = `a` (last in postorder)
- Split at index 3 in inorder → left = `[d,b,e]`, right = `[c]`
- Left postorder = `[d,e,b]` (first 3), right postorder = `[c]` (rest minus `a`)

## What This Unlocks

- **Bottom-up algorithms**: postorder naturally expresses "process children before parent."
- **Expression evaluation**: postorder = reverse Polish notation (RPN), used in calculators and stack machines.
- **Memory management**: postorder deletion (free leaves before freeing their parent) is the safe traversal order.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Postorder list | `postorder l @ postorder r @ [v]` | `extend` left, `extend` right, `push` root |
| Last element | `List.nth xs (List.length xs - 1)` | `slice.last().unwrap()` |
| Slice without last | `List.filteri ...` | `&post[..post.len()-1]` |
| Vs preorder reconstruction | Root first in pre | Root last in post — mirror pattern |
