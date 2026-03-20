📖 **[View on hightechmind.io →](https://hightechmind.io/rust/217-catamorphism)**

---

# Catamorphism — The Universal Fold
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A catamorphism (`cata`) is the unique function that folds a recursive structure bottom-up. Instead of writing separate `sum`, `depth`, `pretty_print`, and `count_nodes` functions (each with their own recursion), you write one `cata` function and provide an "algebra" — a non-recursive function that handles one node. `cata` handles the recursion; the algebra handles the semantics. Adding new operations costs only a new algebra, never a new traversal.

## Learning Outcomes

- Understand catamorphisms as the universal fold for any recursive structure
- Learn the three-step recipe: base functor → Fix wrapper → cata + algebra
- See how multiple operations (eval, pretty-print, depth) share one traversal via different algebras
- Understand why catamorphisms generalize `fold_right` for lists and recursive descent for trees

## Rust Application

`cata<A>(f: impl Fn(ExprF<A>) -> A) -> impl Fn(Fix) -> A` implements the universal fold. Algebras: `eval_algebra: ExprF<i64> -> i64` evaluates the expression. `depth_algebra: ExprF<usize> -> usize` computes depth. `pretty_algebra: ExprF<String> -> String` formats as a string. Each algebra handles one node in isolation — no recursion. `cata(eval_algebra)` and `cata(pretty_algebra)` are distinct traversals from one infrastructure.

## OCaml Approach

OCaml's catamorphism:
```ocaml
let cata (alg : 'a expr_f -> 'a) (Fix e : fix_expr) : 'a =
  alg (map_expr_f (cata alg) e)
```
`map_expr_f` applies `cata alg` to each child, then `alg` processes the current node. OCaml's `let rec` makes the recursion in `cata` itself explicit and natural. Multiple algebras for the same structure are the standard functional approach to the expression problem.

## Key Differences

1. **Expression problem**: `cata` solves the "add new operations without modifying types" direction; adding new node types requires adding new base functor variants — the other direction.
2. **Performance**: `cata` processes bottom-up: leaf nodes are folded first, then their parents; this is equivalent to `fold_right` semantics for lists.
3. **Stack depth**: `cata` is still recursive and can overflow for deep structures; trampolining (example 197) or an iterative stack is needed for very deep trees.
4. **Generalization**: `cata` is one of many recursion schemes (`ana`, `hylo`, `para`, `histo`) — all sharing the same Fix infrastructure.

## Exercises

1. Write a `count_nodes` algebra that counts total nodes in the expression tree.
2. Implement `constant_fold` algebra: simplify `Add(Lit(2), Lit(3))` → `Lit(5)` during the fold.
3. Add a `NegF(A)` variant to `ExprF` and update all algebras — verify that adding a new variant requires only algebra updates, not `cata` changes.
