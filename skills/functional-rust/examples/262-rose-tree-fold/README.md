# 262: Rose Tree — Multi-Way Tree with Fold

**Difficulty:** 2  **Level:** Intermediate

An n-ary tree where every aggregation — size, depth, pretty-print — derives from one generic fold operation.

## The Problem This Solves

A binary tree has exactly two children per node. But many real structures are *n-ary*: a file system directory holds any number of subdirectories; a DOM element has any number of child elements; an AST node for a function call has any number of arguments. These are rose trees — each node has a value and an arbitrary-length list of children.

Once you have a rose tree, you need to traverse it repeatedly: count nodes for size, find the longest path for depth, render it as a string, collect all values, check a condition on every leaf. Without a generic fold, each operation is a separate recursive function with the same boilerplate.

A rose tree fold abstracts the recursion. You provide a combining function: "given a node's value and a list of the results from each child, what's the result for this node?" The fold handles all the traversal; your function handles only the logic. Size, depth, and render all follow this pattern — each is two lines.

## The Intuition

Fold on a rose tree works bottom-up: first fold all the children, then combine the results with the current node's value. It's like evaluating an expression tree: you evaluate the inner expressions first (the children), then apply the outer operation (the node's combining function).

For size: each leaf contributes 1; each internal node contributes 1 + sum of children's sizes. For depth: each leaf contributes 0; each internal node contributes 1 + maximum child depth. The combining function changes; the tree traversal stays the same. That's the point of fold.

In Rust, the combining function is a `&dyn Fn(&T, Vec<R>) -> R` — a reference to a function that takes the node value and a `Vec` of child results, and returns the result for this node. No boxing of the return value needed; the `R` type is generic.

## How It Works in Rust

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Rose<T> {
    pub value: T,
    pub children: Vec<Rose<T>>,  // Vec already heap-allocates; no Box needed
}

impl<T> Rose<T> {
    pub fn leaf(value: T) -> Self { Rose { value, children: vec![] } }
    pub fn node(value: T, children: Vec<Rose<T>>) -> Self { Rose { value, children } }

    // Bottom-up fold: fold children first, then combine with this node's value
    pub fn fold<R>(&self, f: &dyn Fn(&T, Vec<R>) -> R) -> R {
        let child_results: Vec<R> = self.children.iter().map(|c| c.fold(f)).collect();
        f(&self.value, child_results)  // apply combining function
    }
}

// Three aggregations, each derived from fold — no separate recursion
pub fn size<T>(tree: &Rose<T>) -> usize {
    tree.fold(&|_, sizes: Vec<usize>| 1 + sizes.iter().sum::<usize>())
}

pub fn depth<T>(tree: &Rose<T>) -> usize {
    tree.fold(&|_, depths: Vec<usize>| {
        1 + depths.into_iter().max().unwrap_or(0)
    })
}

pub fn render(tree: &Rose<String>) -> String {
    tree.fold(&|val, child_renders: Vec<String>| {
        if child_renders.is_empty() { val.clone() }
        else { format!("{}({})", val, child_renders.join(", ")) }
    })
}
```

Building a tree: `Rose::node("root", vec![Rose::leaf("a"), Rose::node("b", vec![Rose::leaf("c")])])`.

## What This Unlocks

- **AST evaluation** — compiler/interpreter ASTs are rose trees; fold evaluates or transforms them in one pass.
- **File system traversal** — directory trees are rose trees; fold computes total size, file count, or finds all `.rs` files.
- **DOM processing** — HTML elements with children are rose trees; fold renders, validates, or extracts data in a single recursive pass.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Data representation | `Rose of 'a * 'a rose list` (variant) | `struct Rose<T> { value, children: Vec<T> }` |
| Children storage | Linked list | `Vec<Rose<T>>` — heap-allocated, random access |
| Fold closure | First-class, partially applicable | `&dyn Fn(&T, Vec<R>) -> R` trait object |
| Box requirement | GC handles recursive type | `Vec` handles heap alloc; no extra `Box` needed |
| Partial application | `let size = fold (fun _ cs -> ...)` | Standalone function wrapping `.fold(...)` |
