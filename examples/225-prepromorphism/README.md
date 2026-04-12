📖 **[View on hightechmind.io →](https://hightechmind.io/rust/225-prepromorphism)**

---

# Prepromorphism — Apply Natural Transformation at Each Step
**Difficulty:** ⭐⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A prepromorphism applies a natural transformation (a functor-to-functor mapping) to each layer of a structure before folding it. This enables optimizations or normalizations that happen during the fold: simplifying expression nodes before evaluating them, or applying a "compress" step before accumulating. The natural transformation must be a functor endomorphism: `F<A> -> F<A>` (same functor, same element type).

## Learning Outcomes

- Understand prepromorphisms as catamorphisms with an interleaved natural transformation
- Learn what a natural transformation is in the context of recursion schemes
- See expression simplification as a canonical prepromorphism: normalize nodes before folding
- Understand the dual: the postpromorphism (applies the transformation after unfolding in `ana`)

## Rust Application

`prepro<A>(nat: impl Fn(ExprF<Fix>) -> ExprF<Fix>, alg: impl Fn(ExprF<A>) -> A) -> impl Fn(Fix) -> A`. At each node: apply `nat` to normalize the `ExprF<Fix>` layer, then recurse on each child, then apply `alg`. A simplification `nat` might detect `AddF(LitF(0), child)` and simplify to `child` — the normalization happens at every layer before evaluation, enabling multi-level simplification in one pass.

## OCaml Approach

OCaml's prepromorphism:
```ocaml
let rec prepro nat alg (Fix ef) =
  alg (map_expr_f (prepro nat alg) (nat ef))
```
`nat ef` applies the natural transformation first, then `map_expr_f (prepro nat alg)` recurses on the (possibly simplified) children. The simplification `nat` can arbitrarily rewrite the top layer of the expression at each recursive step.

## Key Differences

1. **Transformation timing**: `prepro` applies `nat` before recursing into children — the transformation sees the original children as `Fix` values, not folded results.
2. **Natural transformation**: `nat: F<Fix<F>> -> F<Fix<F>>` maps one layer to another layer of the same type; this is the "natural transformation" in category theory.
3. **Multi-level simplification**: Because `nat` is applied at every level during the fold, simplifications cascade: simplifying the outer level exposes inner simplifications.
4. **Dual (postpro)**: The dual applies a natural transformation after each step of `ana` (unfolding); `prepro` is to `cata` as `postpro` is to `ana`.

## Exercises

1. Implement a constant-folding prepromorphism: `nat` simplifies `Add(Lit(a), Lit(b))` → `Lit(a+b)` before evaluation.
2. Write a `neg_elimination` nat that rewrites `Neg(Neg(x))` → `x` at each level.
3. Implement the dual `postpro<S>(nat, coalg)` and demonstrate it on a list-unfolding example.
