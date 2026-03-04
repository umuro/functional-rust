# 069: Traversable Tree

**Difficulty:** 3  **Level:** Advanced

Map a function with effects (Option/Result) over every tree node — fail fast if any node fails.

## The Problem This Solves

You have a binary tree of strings and you want to parse every node as an integer. If all parses succeed, you want a tree of integers. If *any* node fails, you want the whole operation to fail with an error — not a tree with `None` leaves mixed in.

This is the **traverse** operation: like `map`, but the function returns an effect (`Option<U>`, `Result<U, E>`) and the tree is only returned if *all* effects succeed. It's the difference between "transform and keep going" (map) and "transform but stop on first failure" (traverse).

In Python you'd wrap everything in a try/except and catch the first ValueError. In Rust, the `?` operator does this structurally — each recursive call uses `?` for early exit, and the code reads almost like pure code without the error handling noise.

## The Intuition

**Pure `map`**: apply `f` to each node value. `f` always succeeds. Returns a new tree.

**`traverse` with `Option`**: apply `f` to each node value. `f` might return `None`. If any node returns `None`, the whole traversal returns `None`. Otherwise returns `Some(new_tree)`. This is "all or nothing."

**`traverse` with `Result`**: same idea but with error values. The first `Err` short-circuits the traversal and is returned directly. `Ok(new_tree)` only if every node succeeds.

The key insight: Traversable separates *structure* (the tree shape) from *effects* (Option, Result, IO, etc.). You write one `traverse` per effect type, and the tree structure handling stays constant.

In OCaml, implementing `traverse` for a deeply nested tree requires explicit pattern matching at each level — matching the left subtree result, the node result, and the right subtree result produces deeply nested code. Rust's `?` operator eliminates this nesting entirely — the code looks almost exactly like the pure `map` function.

## How It Works in Rust

```rust
#[derive(Debug, PartialEq, Clone)]
enum Tree<T> {
    Leaf,
    Node(Box<Tree<T>>, T, Box<Tree<T>>),
}

impl<T> Tree<T> {
    // Traverse with Option: None on any failure → None for whole tree
    fn traverse_option<U>(&self, f: &impl Fn(&T) -> Option<U>) -> Option<Tree<U>> {
        match self {
            Tree::Leaf => Some(Tree::Leaf),
            Tree::Node(l, v, r) => {
                let l2 = l.traverse_option(f)?;   // ? returns None immediately on failure
                let v2 = f(v)?;                    // ? short-circuits here if f returns None
                let r2 = r.traverse_option(f)?;
                Some(Tree::node(l2, v2, r2))       // only reached if all succeeded
            }
        }
    }

    // Traverse with Result: Err propagates immediately
    fn traverse_result<U, E>(&self, f: &impl Fn(&T) -> Result<U, E>) -> Result<Tree<U>, E> {
        match self {
            Tree::Leaf => Ok(Tree::Leaf),
            Tree::Node(l, v, r) => {
                let l2 = l.traverse_result(f)?;   // ? returns Err immediately on failure
                let v2 = f(v)?;
                let r2 = r.traverse_result(f)?;
                Ok(Tree::node(l2, v2, r2))
            }
        }
    }
}
```

Usage — parsing a tree of strings to integers:
```rust
let string_tree = Tree::node(
    Tree::Leaf, "42".to_string(), Tree::node(Tree::Leaf, "7".to_string(), Tree::Leaf)
);

// All succeed → Some(integer tree)
let int_tree = string_tree.traverse_option(|s| s.parse::<i32>().ok());

// One bad node → None for the whole thing
let bad_tree = Tree::node(
    Tree::Leaf, "42".to_string(), Tree::node(Tree::Leaf, "bad".to_string(), Tree::Leaf)
);
let result = bad_tree.traverse_option(|s| s.parse::<i32>().ok());
assert!(result.is_none());  // "bad" failed → whole thing is None
```

## What This Unlocks

- **Validation trees**: traverse with `Result<T, Vec<Error>>` to collect all errors, or `?` style to fail-fast — two different traversal strategies for the same tree.
- **IO over tree structures**: traverse with `Result<T, io::Error>` to read files at every node, failing if any file is missing.
- **Configuration parsing**: a tree of raw config values traversed with a validator returns a typed config tree or the first validation error.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Pattern matching depth | Deeply nested `match`; each level matches the option/result | `?` operator linearizes: three `?` lines + return |
| Nesting count | `match (traverse_opt l) with Some l2 -> match (f v) ...` | `let l2 = ...?; let v2 = f(v)?; let r2 = ...?;` |
| Effect abstraction | Can write `traverse` generically over any monad via modules | Must implement separately per effect type (no HKT) |
| Tree ownership | OCaml reuses unchanged branches (GC sharing) | Rust builds a new owned tree — no sharing |
| Error collection | Same challenge — need Applicative not Monad for "collect all errors" | Same; use `Result<T, Vec<E>>` for accumulating |
