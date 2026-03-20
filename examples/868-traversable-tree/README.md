📖 **[View on hightechmind.io →](https://hightechmind.io/rust/868-traversable-tree)**

---

# 868-traversable-tree — Traversable Tree
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

A Traversable structure extends Foldable by allowing each element to produce an effect, then collecting the results into the same shape. The canonical examples are: validate every node (returning `Option<Tree<U>>` — None if any node fails), or parse every node (returning `Result<Tree<U>, E>` — Err on first failure). This is formally the "traverse" operation from the Haskell `Traversable` typeclass. It is used in compilers for type-checking expression trees, in configuration validators that walk nested structures, and in reactive systems that need all-or-nothing transformation of data trees.

## Learning Outcomes

- Understand the difference between map (no effects) and traverse (effectful map that can fail)
- Implement `traverse_option` and `traverse_result` for a binary tree
- Use the `?` operator inside recursive tree functions to propagate failures
- Compare Rust's explicit `Option`/`Result` chaining with OCaml's monadic bind
- Recognize traverse as a unification of "map" and "sequence"

## Rust Application

The code defines `Tree<T>` with three traversal methods. `traverse_option` returns `Option<Tree<U>>`, failing with `None` if any node's `f(v)` returns `None`. `traverse_result` returns `Result<Tree<U>, E>`, short-circuiting on the first `Err`. The pure `map` method is a degenerate traversal with no effects. The `?` operator threads failures cleanly through the recursive calls. All three methods preserve the original tree shape in the success case.

## OCaml Approach

OCaml lacks the `?` shorthand, so traversal is expressed as explicit pattern matches on `None`/`Some` and `Error`/`Ok` inside each recursive case. OCaml's `Traversable` typeclass equivalent requires a module functor parameterized over the applicative/monad. In practice, OCaml users write the specific `traverse_option` and `traverse_result` directly as shown in `example.ml`. With the `let*` syntax (OCaml 4.08+), monadic chains become readable without manual nesting.

## Key Differences

1. **Error propagation**: Rust uses `?` inside recursive functions; OCaml uses explicit `match` or `let*` monadic bind.
2. **No higher-kinded types**: Rust cannot abstract over `Option` and `Result` in a single generic `traverse`; OCaml can via module functors parameterized on an applicative.
3. **Tree shape preservation**: Both preserve the tree structure on success and produce a flat error/none on failure.
4. **Ownership**: Rust traversal borrows `&self` and must clone subtrees into `Some(Tree::node(...))` on success; OCaml reuses GC-managed nodes.

## Exercises

1. Implement `traverse_vec` that applies `f: &T -> Vec<U>` to each node, returning a `Vec<Tree<U>>` with all combinations (cartesian product across nodes).
2. Add a `sequence_option` function that converts `Tree<Option<T>>` into `Option<Tree<T>>` using `traverse_option` with the identity function.
3. Implement `validate_all` that collects all `Err` values into a `Vec<E>` rather than stopping at the first, using a custom accumulating result type.
