📖 **[View on hightechmind.io →](https://hightechmind.io/rust/585-pattern-box-deref)**

---

# 585: Box Deref Patterns

**Difficulty:** 2  **Level:** Beginner

Match through `Box<T>` in pattern expressions — Rust auto-derefs through the box so recursive data structures feel natural.

## The Problem This Solves

Recursive data structures (trees, linked lists, expression ASTs) must use `Box<T>` in Rust because the compiler needs to know the size of every type at compile time. A `Tree::Node` can't contain a `Tree` directly — it would be infinitely sized. `Box<Tree>` is a fixed-size pointer to a heap-allocated `Tree`.

The question is then: how do you `match` on these structures? Without automatic deref, you'd write `match *t { ... }` at every level or wrestle with reference vs ownership. Rust's deref coercion handles this transparently: when you match on `&Box<Tree>` in a function taking `&Tree`, the compiler inserts the necessary derefs. For consuming matches (taking ownership through a `Box`), you use `match *t { ... }` explicitly.

This is the example every Rust learner needs when implementing their first BST or AST.

## The Intuition

`Box<T>` is just a heap pointer. When you match on a `&Tree` where the `Node` variant holds `Box<Tree>` children, Rust automatically dereferences through the box in `match` arms. The `left` and `right` bindings in `Tree::Node { val, left, right }` have type `&Box<Tree>`, which further coerces to `&Tree` when passed recursively — all transparent.

For consuming patterns (`match *t`), you dereference the box explicitly to move out the contents. This is the ownership-consuming form: the `Box` is consumed, its content is moved into the match arm.

## How It Works in Rust

**The tree type** — `Box` because recursive:
```rust
enum Tree {
    Leaf,
    Node { val: i32, left: Box<Tree>, right: Box<Tree> }
}
```

**Borrowed match — auto-deref through Box:**
```rust
fn depth(t: &Tree) -> usize {
    match t {
        Tree::Leaf => 0,
        Tree::Node { left, right, .. } =>
            1 + depth(left).max(depth(right)),
        //          ^^^^ left: &Box<Tree>, auto-coerces to &Tree for recursion
    }
}
```
Matching on `&Tree` when `left` is `Box<Tree>` — Rust dereferences automatically, so `depth(left)` compiles as `depth(&**left)`.

**Consuming match — explicit `*t`:**
```rust
fn insert(t: Box<Tree>, v: i32) -> Box<Tree> {
    match *t {  // dereference: move Tree out of Box
        Tree::Leaf => Tree::node(v, Tree::leaf(), Tree::leaf()),
        Tree::Node { val, left, right } => {
            if v < val { Tree::node(val, insert(left, v), right) }
            else if v > val { Tree::node(val, left, insert(right, v)) }
            else { Tree::node(val, left, right) }
        }
    }
}
```
`match *t` moves the `Tree` out of the `Box`. The old `Box` is consumed; `insert` returns a new `Box`.

**Inorder traversal with borrowed match:**
```rust
fn contains(t: &Tree, v: i32) -> bool {
    match t {
        Tree::Leaf => false,
        Tree::Node { val, left, right } => match v.cmp(val) {
            Ordering::Equal   => true,
            Ordering::Less    => contains(left, v),
            Ordering::Greater => contains(right, v),
        },
    }
}
```

## What This Unlocks

- **Recursive data structures** — the standard Rust pattern for trees, lists, and expression ASTs.
- **Pattern matching on nested heap data** — no manual unwrapping or pointer chasing in match arms.
- **Ownership-consuming transformations** — `match *t` lets you destructure a `Box` and rebuild a new tree without cloning.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Recursive type | Direct recursion (GC) | Must use `Box<T>` for indirection |
| Pattern match through ref | Natural | `match t` on `&Tree`; Box auto-derefs |
| Consuming destructure | Pattern binding (GC frees old) | `match *box_val` — moves out of heap |
| Algebraic tree | `type tree = Leaf \| Node of int * tree * tree` | `enum Tree { Leaf, Node { val: i32, left: Box<Tree>, right: Box<Tree> } }` |
