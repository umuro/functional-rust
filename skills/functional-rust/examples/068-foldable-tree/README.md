# 068: Foldable Tree

**Difficulty:** 2  **Level:** Intermediate

Implement in-order, pre-order, and post-order folds for a binary tree, then derive aggregations from a single fold primitive.

## The Problem This Solves

A binary tree can be traversed in three meaningful ways: in-order (left, root, right — produces sorted order for a BST), pre-order (root, left, right — useful for serialisation), post-order (left, right, root — useful for deletion or evaluation). Each order produces a different sequence from the same structure.

Without parameterised folds, you'd write separate recursive functions for every aggregation: `sum_inorder`, `max_inorder`, `to_vec_preorder`, etc. With a fold that takes traversal order as a parameter, you write the traversal once and derive all aggregations by varying the combining function.

This is the practical meaning of "fold is universal": every aggregation over a tree — sum, max, any, all, count, to_vec — is a fold with a different initial value and combining function.

## The Intuition

Think of a fold as threading a needle through the tree. The traversal order determines the path of the needle (which nodes it visits first). The combining function determines what happens at each node — whether you sum, compare, collect, or test.

Change the path (in-order vs pre-order): you get elements in a different sequence.  
Keep the path, change the combining function: you get a different aggregation.

That separation is the power.

## How It Works in Rust

```rust
impl<T> Tree<T> {
    // In-order: left → root → right (sorted order for BST)
    fn fold_inorder<B>(&self, init: B, f: &mut impl FnMut(B, &T) -> B) -> B {
        match self {
            Tree::Leaf => init,
            Tree::Node(l, v, r) => {
                let acc = l.fold_inorder(init, f);  // visit left subtree
                let acc = f(acc, v);                // visit root
                r.fold_inorder(acc, f)              // visit right subtree
            }
        }
    }
    // Pre-order: root → left → right (swap first two lines above)
    // Post-order: left → right → root (root last)
}

impl Tree<i32> {
    // Everything derived from fold_inorder:
    fn sum(&self)    -> i32         { self.fold_inorder(0,     &mut |acc, x| acc + x) }
    fn max_val(&self) -> Option<i32> { self.fold_inorder(None,  &mut |acc, x| Some(acc.map_or(*x, |a| a.max(*x)))) }
    fn all(&self, pred: impl Fn(&i32) -> bool) -> bool { self.fold_inorder(true, &mut |acc, x| acc && pred(x)) }
    fn count(&self, pred: impl Fn(&i32) -> bool) -> usize { self.fold_inorder(0, &mut |acc, x| if pred(x) { acc + 1 } else { acc }) }
    fn to_vec_inorder(&self) -> Vec<i32> {
        let mut v = Vec::new();
        self.fold_inorder((), &mut |(), x| { v.push(*x); });
        v
    }
}
```

The `&mut impl FnMut` signature lets the closure be mutated through recursive calls — necessary because the closure captures mutable state (like `&mut Vec`) across the recursion. This is where Rust's borrow checker shapes the API differently from OCaml.

## What This Unlocks

- **BST operations** — in-order fold on a sorted binary tree gives you sorted order, enabling range queries and sorted output.
- **Expression tree evaluation** — post-order fold evaluates an AST naturally (children before parent).
- **Serialisation** — pre-order fold produces a prefix notation sequence for compact tree encoding.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Closure through recursion | Plain function argument | `&mut impl FnMut` (mutable borrow through recursion) |
| Side-effecting fold | Functional (returns value) | `to_vec_inorder` captures `&mut Vec` in closure |
| Empty-tree max | `min_int` sentinel | `Option<i32>` — no sentinel needed |
| Traversal order | Separate functions or parameter | Separate methods (`fold_inorder`, `fold_preorder`, `fold_postorder`) |
| Tree node allocation | Implicit heap | `Box<Tree<T>>` for recursive fields |
