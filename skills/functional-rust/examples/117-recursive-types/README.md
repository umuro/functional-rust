# 117: Recursive Types with Box

**Difficulty:** 2  **Level:** Intermediate

Build trees, lists, and expression ASTs in Rust using `Box` for heap indirection.

## The Problem This Solves

Every type in Rust must have a size known at compile time. When you write `enum Tree<T> { Leaf, Node(Tree<T>, T, Tree<T>) }`, the compiler tries to compute the size: `sizeof(Node) = sizeof(Tree) + sizeof(T) + sizeof(Tree)`. But that's circular — `sizeof(Tree)` depends on itself. The compiler gives up with "recursive type has infinite size."

In OCaml this just works because the GC heap-allocates everything and the compiler only stores a pointer. Rust's answer is: make the indirection explicit. By writing `Node(Box<Tree<T>>, T, Box<Tree<T>>)`, you tell the compiler: "the children are heap pointers, not inline structs." Now the size is fixed: two pointers plus `sizeof(T)`.

This isn't a workaround — it's intentional. Rust's type system forces you to be explicit about where heap allocation happens, which means you can reason precisely about memory layout, copying costs, and where data lives.

## The Intuition

Wrap recursive enum variants in `Box` to give the compiler a fixed-size pointer instead of an infinitely recursive type.

## How It Works in Rust

```rust
// Without Box: "error[E0072]: recursive type `Tree` has infinite size"
// enum Tree<T> { Leaf, Node(Tree<T>, T, Tree<T>) }

// With Box: the Node variant holds pointers, which are pointer-sized
#[derive(Debug)]
enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

impl<T: Ord> Tree<T> {
    fn insert(self, x: T) -> Self {
        match self {
            Tree::Leaf => Tree::Node(
                Box::new(Tree::Leaf), x, Box::new(Tree::Leaf)
            ),
            Tree::Node(l, v, r) => {
                if x < v { Tree::Node(Box::new(l.insert(x)), v, r) }
                else      { Tree::Node(l, v, Box::new(r.insert(x))) }
            }
        }
    }
}

// Build a BST, traverse in-order
let tree = [5, 3, 7, 1].iter().fold(Tree::Leaf, |t, &x| t.insert(x));
// sorted: [1, 3, 5, 7]
```

## What This Unlocks

- **Binary trees and BSTs** — the foundational data structure for sorted sets and maps.
- **Linked lists** — `Cons(T, Box<List<T>>)` vs. the more idiomatic `Vec<T>`, useful for understanding ownership chains.
- **Expression ASTs** — interpreters, compilers, and calculators all model syntax as recursive trees.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive types | Direct — GC handles layout | Require `Box` for heap indirection |
| Size at compile time | Not required (GC pointer) | Required — `Box` provides fixed size |
| Allocation control | Implicit (GC decides) | Explicit (`Box::new` allocates) |
| Memory overhead | GC header per value | Zero overhead beyond the pointer |
