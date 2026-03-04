# 035: Layout Binary Tree

**Difficulty:** ⭐⭐  **Level:** Foundations

Assign (x, y) coordinates to each node: x = in-order position, y = depth.

## The Problem This Solves

Visualizing a tree requires knowing *where* to draw each node. The most natural layout for a binary tree assigns horizontal position based on in-order traversal (left to right across the leaves), and vertical position based on depth. This is exactly how tree diagrams in textbooks are drawn.

This problem is a template for all tree layout algorithms: renderers for graphviz, IDE code outline views, directory tree displays. The pattern — thread a mutable counter through a recursive traversal — appears constantly in real parsing and rendering code.

## The Intuition

In-order traversal visits nodes left-root-right. If you count from 1 each time you visit, you get the natural left-to-right reading order. The depth is just the recursion level.

In Python you might use a closure or mutable default arg to carry the counter:
```python
def layout(node, depth=1, counter=[0]):
    if not node: return []
    result = layout(node.left, depth + 1, counter)
    counter[0] += 1
    result.append((node.val, counter[0], depth))
    result.extend(layout(node.right, depth + 1, counter))
    return result
```

In Rust, we pass the counter as `&mut usize` — a mutable reference. The function signature makes it explicit that this function *modifies* the counter. No hidden state, no surprise mutations.

**Why not a return value for the counter?** The counter is shared across the entire traversal — both the left and right subtree calls need to see each other's updates. A mutable reference threads state through the recursion cleanly.

## How It Works in Rust

```rust
#[derive(Debug, PartialEq, Clone)]
struct LayoutNode<T> {
    val: T,
    x: usize,  // in-order position (1-based)
    y: usize,  // depth (1-based, root = 1)
}

fn layout<T: Clone>(tree: &Tree<T>) -> Vec<LayoutNode<T>> {
    fn aux<T: Clone>(
        tree: &Tree<T>,
        depth: usize,
        counter: &mut usize,   // shared mutable counter
    ) -> Vec<LayoutNode<T>> {
        match tree {
            Tree::Leaf => vec![],
            Tree::Node(l, v, r) => {
                let mut result = aux(l, depth + 1, counter); // left first
                *counter += 1;                                // visit root
                result.push(LayoutNode { val: v.clone(), x: *counter, y: depth });
                result.extend(aux(r, depth + 1, counter));   // right last
                result
            }
        }
    }
    let mut counter = 0;
    aux(tree, 1, &mut counter)
}
```

For the sample tree:
```
       a          → x=4, y=1
      / \
     b   c        → b: x=2, y=2  |  c: x=5, y=2
    / \
   d   e          → d: x=1, y=3  |  e: x=3, y=3
```
In-order: d(1,3) → b(2,2) → e(3,3) → a(4,1) → c(5,2)

## What This Unlocks

- **Tree visualization**: convert any tree to a 2D diagram for SVG, console, or UI rendering.
- **Compiler output**: assign source positions to AST nodes for error messages.
- **Database B-tree visualization**: same layout algorithm applies to wider trees.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable counter through recursion | `ref` counter or return tuple | `&mut usize` parameter |
| Nested helper function | `let rec aux ...` inside function | `fn aux<T>(...) { }` inside function body |
| Result struct | Record type `{ val; x; y }` | `struct LayoutNode<T> { val, x, y }` |
| In-order threading | Same left-root-right order | Same |
