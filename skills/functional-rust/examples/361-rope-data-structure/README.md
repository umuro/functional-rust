# 361: Rope for Efficient String Operations

**Difficulty:** 4  **Level:** Expert

A tree-based string representation enabling O(1) concatenation and O(log n) splits — without copying.

## The Problem This Solves

Strings are immutable byte arrays in most languages. Concatenating two strings means allocating a new buffer and copying both inputs — `O(n)`. For small strings this is fine. For text editors handling megabytes of source code, this is catastrophic. Every keystroke, every undo, every paste would trigger a massive copy.

Real text editors (Vim, VS Code, Emacs) use specialized data structures to avoid this. The rope is the classic solution. A rope represents a string as a binary tree of string slices. Concatenation just creates a new root node pointing to both trees — no copying. Splitting walks the tree and restructures it — `O(log n)`. Index lookup traverses down the tree by accumulated length — also `O(log n)`.

The tradeoff: random character access is slower than a flat string (`O(log n)` vs `O(1)`), and memory overhead is higher. But for workloads dominated by insertions, deletions, and concatenations of large strings, ropes win decisively.

## The Intuition

Think of a rope as a linked list of string chunks, but organized as a balanced binary tree for efficient splitting and indexing. Each leaf holds a string slice. Each internal node holds the total length of its left subtree — enough information to navigate to any character position in `O(log n)` steps.

When you concatenate two ropes, you create a new root. When you split, you walk down and restructure. The key insight: you never modify existing nodes, so sharing subtrees between versions is safe — this is how persistent text buffers work.

## How It Works in Rust

```rust
#[derive(Clone)]
enum Rope {
    Leaf(String),
    Node {
        left: Box<Rope>,
        right: Box<Rope>,
        len: usize,  // total length for navigation
    },
}

impl Rope {
    fn concat(left: Rope, right: Rope) -> Rope {
        let len = left.len() + right.len();
        Rope::Node {
            left: Box::new(left),
            right: Box::new(right),
            len,
        }
    }

    fn len(&self) -> usize {
        match self {
            Rope::Leaf(s) => s.len(),
            Rope::Node { len, .. } => *len,
        }
    }
}
```

In practice, use the `ropey` crate for a production-quality implementation with Unicode support and rebalancing.

## What This Unlocks

- **Text editors** — efficient insert/delete at arbitrary positions without copying the whole buffer.
- **Persistent strings** — share subtrees between versions; structural sharing for undo/redo.
- **Diff engines** — split and merge large texts efficiently during patch application.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| String concat | `^` operator, `O(n)` copy | `+` operator (`O(n)`), or `Rope::concat` (`O(1)`) |
| Persistent string | Immutable strings, but still copies | `Rope` with `Arc` subtree sharing |
| Text buffer | `Buffer.t` (gap buffer in editors) | `ropey::Rope` (tree-based) |
| Split/slice | `String.sub`, `O(n)` copy | `Rope::split` `O(log n)`, no copy |
