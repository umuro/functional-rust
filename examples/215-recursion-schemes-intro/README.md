📖 **[View on hightechmind.io →](https://hightechmind.io/rust/215-recursion-schemes-intro)**

---

# Example 215: Recursion Schemes — Separating What From How

**Difficulty:** ⭐⭐⭐
**Category:** Recursion Schemes
**OCaml Source:** Meijer, Fokkinga, Paterson — "Functional Programming with Bananas, Lenses, Envelopes and Barbed Wire" (1991)

## Problem Statement

When processing a recursive data structure (AST, tree, config), every function (`eval`, `show`, `depth`, `count_nodes`) rewrites the same recursive traversal with only the leaf/combine logic differing. Recursion schemes factor the traversal out into one place — `cata` — so each new function becomes a plain, non-recursive algebra.

## Learning Outcomes

- How to construct a *base functor* (`ExprF<A>`) by replacing recursive positions with a type variable
- How `map` on the functor enables the catamorphism to recurse automatically
- Why Rust uses owned values (`Expr`) in `cata` while direct recursion prefers borrows (`&Expr`)
- How closures as algebras compile to zero-cost abstractions via monomorphisation

## OCaml Approach

OCaml defines `type 'a expr_f` as the base functor, implements `fmap` to transform recursive positions, and writes `cata alg e = alg (fmap (cata alg) (project e))`. Algebras are ordinary functions like `eval_alg : int expr_f -> int`. OCaml's GC makes value passing frictionless.

## Rust Approach

Rust defines `enum ExprF<A>` as the base functor with a `map` method. The catamorphism `cata<A, F>(e: Expr, alg: &F) -> A` consumes the tree (ownership, no GC) and passes `alg` by reference so it can be called repeatedly. Closures serve as algebras; monomorphisation ensures no runtime overhead.

## Key Differences

1. **Ownership model:** OCaml passes by GC reference freely; Rust `cata` consumes `Expr` (owned), while direct helpers borrow `&Expr` — the type system enforces the distinction.
2. **Functor map:** OCaml `fmap` is a standalone function; Rust `ExprF::map` is a method, matching idiomatic Rust style.
3. **Algebra representation:** OCaml uses named functions; Rust uses `Fn(ExprF<A>) -> A` closures or function pointers — same flexibility, different syntax.
4. **Monomorphisation:** Rust generates a specialised `cata` for each algebra at compile time, so there is no dynamic dispatch or boxing penalty.
