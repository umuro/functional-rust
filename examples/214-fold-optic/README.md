📖 **[View on hightechmind.io →](https://hightechmind.io/rust/214-fold-optic)**

---

# Fold Optic — Read-Only Multi-Focus Aggregation
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

A Fold is a read-only optic that focuses on multiple values and aggregates them. Unlike a Traversal (which can modify), a Fold only reads — providing `sum`, `count`, `max`, `any`, `all` operations derived from a single `to_list` primitive. Folds are useful when you need to aggregate across deeply nested or graph-structured data without modifying it. They compose like traversals but with only read capabilities.

## Learning Outcomes

- Understand Folds as the read-only counterpart of Traversals
- Derive `sum`, `count`, `max`, `min`, `any`, `all` from the `to_list` primitive
- See how Folds compose via flat-map: focusing into sub-structures
- Understand Folds in the context of functional reactive programming (FRP) and data pipelines

## Rust Application

`Fold<S, A>` wraps `to_list: Box<dyn Fn(&S) -> Vec<A>>`. Operations: `sum_of(s)` = `to_list(s).iter().sum()` (for numeric `A`). `count_of(s)` = `to_list(s).len()`. `max_of(s)` = `to_list(s).iter().max().cloned()`. Fold composition via flat-map: if `fold_b_in_a: Fold<A, B>` and `fold_a_in_s: Fold<S, A>`, then `composed: Fold<S, B>` = `s => fold_a_in_s.to_list(s).flat_map(|a| fold_b_in_a.to_list(a))`.

## OCaml Approach

OCaml's fold optic mirrors the Haskell `Fold` typeclass. A fold is characterized by:
```ocaml
type ('s, 'a) fold = { to_list : 's -> 'a list }
let sum_of fold s = List.fold_left (+) 0 (fold.to_list s)
let compose f1 f2 = { to_list = fun s -> List.concat_map f2.to_list (f1.to_list s) }
```
`List.concat_map` is the flat-map that composes folds — focusing through all levels.

## Key Differences

1. **Read-only**: Folds have no `over` or `set` — they are strictly for aggregation; traversals are a superset that adds write capability.
2. **Composition via flat-map**: Fold composition uses flat-map (concat_map); lens composition uses function composition — different mechanisms reflecting the different structure.
3. **Lazy evaluation**: A lazy Fold can focus on infinite structures without materializing all elements; `to_list` makes them strict; both languages can implement lazy folds.
4. **FRP connection**: Reactive streams and observables are essentially infinite folds — focusing on events over time.

## Exercises

1. Implement a fold over all leaves of a tree and verify `sum_of` produces the correct total.
2. Write `filter_fold(fold, pred) -> Fold<S, A>` that focuses only on elements satisfying `pred`.
3. Implement `map_fold(fold, f) -> Fold<S, B>` that transforms focused elements — making `Fold` a `Functor`.
