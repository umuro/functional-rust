# 066: Tree Map and Fold

**Difficulty:** 2  **Level:** Intermediate

Lift `map` and `fold` from lists to binary trees — once you have `fold_tree`, everything else is a one-liner.

## The Problem This Solves

You've used `map` and `fold` on lists. Trees are everywhere in real programs — ASTs, file systems, expression trees, decision trees — and they need the same operations. The naive approach is to write explicit recursive functions for each thing you want to compute: one for size, one for depth, one for sum, one for in-order traversal. After the third one you realize they're all the same structural pattern.

`fold_tree` captures that pattern once. You pass a function that says "given the current node's value and the results from the left and right subtrees, combine them." Then size, depth, sum, and all traversals become one-liners that pass different combining functions. No more explicit recursion.

This is the catamorphism pattern applied to binary trees — the same idea as catamorphism (example 080), but made concrete here on a familiar data structure before the abstraction is generalized.

## The Intuition

A binary tree is either a `Leaf` (empty) or a `Node` containing a value, a left subtree, and a right subtree.

**`map_tree`** transforms every node's value while keeping the tree structure. The tree shape is preserved — only the labels change. If you double every number in the tree, you get a tree with the same branching structure, every number doubled.

**`fold_tree`** collapses the tree into a single value. It processes bottom-up: fold the left subtree (getting some accumulated value), fold the right subtree (getting another), then combine those results with the current node's value. The combining function `f(value, left_result, right_result)` is called once per node.

For **size**: `f(_, l, r) = 1 + l + r` — count this node (1) plus left count plus right count.
For **depth**: `f(_, l, r) = 1 + max(l, r)` — current level (1) plus the deeper of the two sides.
For **sum**: `f(v, l, r) = v + l + r` — current value plus sums of both sides.

Notice that `Leaf` returns the *initial accumulator* — for size that's 0, for sum it's also 0. This is analogous to the initial value in a list fold.

## How It Works in Rust

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Tree<T> {
    Leaf,
    Node(T, Box<Tree<T>>, Box<Tree<T>>),
}

pub fn map_tree<T, U>(tree: &Tree<T>, f: &impl Fn(&T) -> U) -> Tree<U> {
    match tree {
        Tree::Leaf => Tree::Leaf,
        Tree::Node(v, l, r) => Tree::node(f(v), map_tree(l, f), map_tree(r, f)),
    }
}

pub fn fold_tree<T, A: Clone>(
    tree: &Tree<T>,
    acc: A,                          // base value for Leaf
    f: &impl Fn(&T, A, A) -> A,     // combine: (node_value, left_result, right_result)
) -> A {
    match tree {
        Tree::Leaf => acc,
        Tree::Node(v, l, r) => {
            let left  = fold_tree(l, acc.clone(), f);
            let right = fold_tree(r, acc, f);
            f(v, left, right)
        }
    }
}

// Every derived operation is a fold one-liner:
pub fn size<T>(t: &Tree<T>) -> usize  { fold_tree(t, 0, &|_, l, r| 1 + l + r) }
pub fn depth<T>(t: &Tree<T>) -> usize { fold_tree(t, 0, &|_, l, r| 1 + l.max(r)) }
pub fn sum(t: &Tree<i32>) -> i32      { fold_tree(t, 0, &|v, l, r| v + l + r) }
```

The `acc.clone()` in `fold_tree` is necessary because `acc` is passed to *both* subtrees. The left call consumes it, so we clone for the right. This is the main Rust-vs-OCaml difference: OCaml's GC handles sharing implicitly.

## What This Unlocks

- **One pattern, many computations**: define `fold_tree` once, express size, depth, sum, min, max, flattening, and serialization without any more explicit recursion.
- **Composable transformations**: `map_tree` followed by `fold_tree` is a common pattern — transform values, then aggregate. Pipeline-friendly.
- **Foundation for catamorphisms**: understanding fold on concrete trees prepares you for the fully abstract catamorphism in example 080.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `type 'a tree = Leaf \| Node of 'a * 'a tree * 'a tree` | `enum Tree<T> { Leaf, Node(T, Box<Tree<T>>, Box<Tree<T>>) }` |
| Box | Not needed — GC manages recursive types | Required: `Box<Tree<T>>` to give the enum a known size |
| fold accumulator sharing | GC shares the initial `acc` value freely | Must `.clone()` — one copy for left, one for right |
| Currying | `fold_tree ~leaf ~node tree` — natural 3-arg curried | Closure `&impl Fn(&T, A, A) -> A` — must pass explicitly |
