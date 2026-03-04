# 080: Catamorphism — Generalized Fold

**Difficulty:** 4  **Level:** Advanced

Replace constructors with functions to fold any algebraic data type — the universal abstraction behind every fold.

## The Problem This Solves

You've written `fold_tree` in example 066. Now you're implementing a different tree shape — maybe an N-ary tree, or a tree with differently-typed leaves and nodes. Do you rewrite the same fold pattern again? No — the catamorphism abstracts it once.

A **catamorphism** (from Greek: *kata* = downward, *morphism* = transformation) is the formal name for "replace each constructor with a function and fold." Every algebraic data type has exactly one catamorphism — it's a structural property of the type, not an algorithm you design. For a list, the catamorphism is `fold_right`. For a binary tree, it's `fold_tree`. For an expression AST, it's an evaluator.

The practical payoff: if you write a catamorphism for your AST, you get interpreters, pretty-printers, type-checkers, optimizers, and serializers all from the same template. Change the functions you pass in, get a different transformation.

## The Intuition

Every algebraic data type is defined by its constructors. A binary tree has two:
- `Leaf` — no arguments
- `Node(left, value, right)` — three arguments

A catamorphism replaces *each constructor* with a *function*:
- `Leaf` → `leaf_val: R` (a base value, since Leaf takes no arguments)
- `Node(left, value, right)` → `node_fn(left_result, value, right_result)` (a function taking the same "shape" of arguments, but with subtree results instead of recursive subtrees)

Processing is bottom-up: process leaves first (return `leaf_val`), then combine using `node_fn` at each node. The result type `R` can be anything — a number, a string, a new tree, a list.

The key insight: once you define `cata`, you never write explicit recursion again for that data type. `size`, `sum`, `height`, `mirror`, `serialize` — all become "what functions do I pass to `cata`?"

For an **expression AST** (`Lit(n) | Add(l, r) | Mul(l, r)`), the catamorphism is:
- `lit_fn(n: i64) -> R`
- `add_fn(left_result: R, right_result: R) -> R`  
- `mul_fn(left_result: R, right_result: R) -> R`

Pass `lit_fn = |n| n, add_fn = |l, r| l + r, mul_fn = |l, r| l * r` and you get an evaluator. Pass `lit_fn = |n| n.to_string(), add_fn = |l, r| format!("({l}+{r})")` and you get a pretty-printer. Same structure, different functions.

## How It Works in Rust

```rust
#[derive(Debug, PartialEq, Clone)]
pub enum Tree<T> { Leaf, Node(Box<Tree<T>>, T, Box<Tree<T>>) }

/// The catamorphism for Tree<T>.
/// leaf_val: what Leaf maps to
/// node_fn: how to combine (left_result, node_value, right_result)
pub fn cata<T, R: Clone>(
    tree: &Tree<T>,
    leaf_val: R,
    node_fn: &dyn Fn(R, &T, R) -> R,
) -> R {
    match tree {
        Tree::Leaf => leaf_val,
        Tree::Node(l, v, r) => {
            let left  = cata(l, leaf_val.clone(), node_fn);  // process left subtree
            let right = cata(r, leaf_val.clone(), node_fn);  // process right subtree
            node_fn(left, v, right)                           // combine at this node
        }
    }
}

// All derived operations — no explicit recursion, just different functions
pub fn size<T>(tree: &Tree<T>) -> usize {
    cata(tree, 0, &|l, _, r| 1 + l + r)
}

pub fn sum(tree: &Tree<i64>) -> i64 {
    cata(tree, 0, &|l, v, r| l + v + r)
}

pub fn height<T>(tree: &Tree<T>) -> usize {
    cata(tree, 0, &|l, _, r| 1 + l.max(r))
}
```

An expression catamorphism giving both evaluator and pretty-printer from the same structure:
```rust
// Evaluator: pass arithmetic functions
let eval = |expr: &Expr| cata_expr(expr,
    |n| n,                     // Lit(n) → n
    |l, r| l + r,              // Add → +
    |l, r| l * r,              // Mul → *
);

// Pretty-printer: pass string functions — same shape, different types
let pretty = |expr: &Expr| cata_expr(expr,
    |n| n.to_string(),
    |l, r| format!("({l} + {r})"),
    |l, r| format!("({l} * {r})"),
);
```

## What This Unlocks

- **One abstraction, all transformations**: define `cata` once per data type, express all transformations (eval, print, optimize, serialize) by choosing different functions.
- **Compiler passes**: each compiler pass (constant folding, dead code elimination, code generation) is a catamorphism over the AST with different functions.
- **Schema migration**: if your data type changes, update `cata` once — all operations that use `cata` get the structural change for free.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Catamorphism API | `cata ~leaf ~node tree` — labeled arguments, natural | `cata(tree, leaf_val, &dyn Fn(...))` — positional, explicit `&dyn` |
| Labeled arguments | `~leaf:(unit -> 'r)` makes call sites readable | No labels; order matters — document carefully |
| Clone bound | Not needed — GC handles sharing `leaf_val` | `R: Clone` required — `leaf_val` passed to both subtrees |
| Mirror (builds new tree) | Returns `tree` type — catamorphism works | Must break out of `cata` — return type is `Tree<T>`, not R=usize |
| Higher-rank types | Can abstract over functor shape | Fixed to `Tree<T>`; generalizing needs GATs |
