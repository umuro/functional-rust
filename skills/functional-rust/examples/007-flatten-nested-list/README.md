# 007: Flatten Nested List

**Difficulty:** 2  **Level:** Beginner

Flatten an arbitrarily nested list structure into a flat list using a recursive enum.

## The Problem This Solves

Real data is rarely flat. A file system has folders inside folders. A JSON document has arrays inside arrays. When you need to process every leaf value, you want a flat list — not a tree you have to recurse through manually every time.

Without a proper recursive type and a `flatten` function, you end up writing the same recursive traversal everywhere, scattered across your codebase. The traversal logic leaks into business logic.

Defining a `Node<T>` enum that can hold either a single value (`One`) or a list of further nodes (`Many`) lets you model the nesting once, and `flatten` handles the traversal cleanly.

## The Intuition

Think of a Russian nesting doll. Each doll either contains a solid core (a `One`) or more dolls (a `Many`). Flattening means opening every doll until you've collected all the cores.

In plain English: walk the list. When you see a `One(x)`, yield `x`. When you see a `Many(xs)`, recurse into `xs` and yield everything inside. Collect all yields into a flat `Vec`.

This is a classic **structural recursion** — the function's shape mirrors the data's shape.

## How It Works in Rust

```rust
#[derive(Debug, PartialEq, Clone)]
enum Node<T> {
    One(T),            // leaf: a single value
    Many(Vec<Node<T>>) // branch: more nodes inside
}

fn flatten<T: Clone>(list: &[Node<T>]) -> Vec<T> {
    list.iter()
        .flat_map(|node| match node {
            Node::One(x)   => vec![x.clone()],  // base case: wrap value
            Node::Many(xs) => flatten(xs),       // recursive case: descend
        })
        .collect()
}
```

`flat_map` does two things at once: transform each element AND flatten one level. The `T: Clone` bound lets us extract owned values from borrowed nodes without consuming the input.

## What This Unlocks

- **Recursive data processing** — any tree-shaped data (ASTs, file trees, JSON) follows this same pattern.
- **Custom iterators** — the recursive `flat_map` pattern is the foundation for writing tree-walking iterators.
- **Ownership discipline** — choosing between `.clone()` here vs a consuming version (`into_iter`) teaches when copying pays off vs when ownership transfer is better.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive type | `type 'a node = One of 'a \| Many of 'a node list` | `enum Node<T> { One(T), Many(Vec<Node<T>>) }` |
| Heap allocation | Implicit (GC) | Explicit (`Vec` allocates on heap) |
| Value extraction | Free under GC | Requires `T: Clone` or ownership transfer |
| Tail-call safety | TCO guaranteed | No TCO — deep nesting can stack overflow |
| Traversal style | Tail-recursive helper with accumulator | `flat_map` + recursive call |
