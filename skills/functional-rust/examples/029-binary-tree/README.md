# 029: Binary Tree

**Difficulty:** ⭐  **Level:** Foundations

Define a binary tree type in Rust and implement basic operations: size and height.

## The Problem This Solves

Almost every non-trivial program eventually needs a tree. File systems are trees. HTML is a tree. Abstract syntax trees power every compiler. If you can model and traverse a binary tree, you have the foundation for all of them.

In Python or Java you'd write a class with `left` and `right` fields that can be `None` or `null`. You'd construct it step by step, and nothing stops you from building a half-broken tree — a node with a left child but no right field set. You have to be disciplined to keep things consistent.

In Rust, we use an `enum`: a tree node *is* either a `Leaf` (empty, no data) or a `Node(left, value, right)`. The type system enforces the shape. You can't have a half-constructed tree — every `Node` always has both subtrees, and every `Leaf` has none. This is algebraic data types (ADTs) in action.

## The Intuition

Think of a binary tree like a Russian nesting doll. A `Leaf` is the smallest doll — nothing inside. A `Node` is a bigger doll containing a value and exactly two more dolls (which can themselves be big or small).

In Python you might write:
```python
class Tree:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right
```

In Rust we express this as an enum:
```rust
enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}
```

The `<T>` makes it generic — a `Tree<char>` holds characters, a `Tree<i32>` holds integers.

**Why `Box<T>`?** Rust needs to know the size of every type at compile time. A `Tree<T>` that contains another `Tree<T>` would be infinitely large on the stack. `Box<Tree<T>>` stores a pointer to the subtree on the heap — known size (one pointer), any depth tree. That's all it is.

## How It Works in Rust

```rust
#[derive(Debug, PartialEq, Clone)]
enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

impl<T: Clone> Tree<T> {
    // Convenience constructors — wrap Box::new for you
    fn leaf() -> Self { Tree::Leaf }
    fn node(left: Tree<T>, val: T, right: Tree<T>) -> Self {
        Tree::Node(Box::new(left), val, Box::new(right))
    }

    // Recursion + pattern matching: each arm handles one case
    fn size(&self) -> usize {
        match self {
            Tree::Leaf => 0,
            Tree::Node(l, _, r) => 1 + l.size() + r.size(),
        }
    }

    fn height(&self) -> usize {
        match self {
            Tree::Leaf => 0,
            Tree::Node(l, _, r) => 1 + l.height().max(r.height()),
        }
    }
}

// Build a tree:
//        a
//       / \
//      b   c
let t = Tree::node(
    Tree::node(Tree::leaf(), 'b', Tree::leaf()),
    'a',
    Tree::node(Tree::leaf(), 'c', Tree::leaf()),
);
println!("{}", t.size());   // 3
println!("{}", t.height()); // 2
```

The `_` in `Node(l, _, r)` ignores the value — we only need left and right for size/height. Rust will warn you if you forget to use a bound variable, so `_` signals "intentionally unused."

## What This Unlocks

- **Every tree algorithm** builds on this foundation — search, traversal, balancing.
- **Compilers and interpreters** use exactly this structure for ASTs (abstract syntax trees).
- **Generic data structures** — the `<T>` parameter means this same code works for any value type.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Tree type | `type 'a tree = Leaf \| Node of 'a tree * 'a * 'a tree` | `enum Tree<T> { Leaf, Node(Box<Tree<T>>, T, Box<Tree<T>>) }` |
| Recursive heap allocation | Automatic (GC) | Explicit `Box::new(...)` |
| Pattern matching | `match t with \| Leaf -> ... \| Node(l,v,r) -> ...` | `match t { Tree::Leaf => ..., Tree::Node(l,v,r) => ... }` |
| Generics | Type parameter `'a` | Type parameter `T` with trait bounds |
