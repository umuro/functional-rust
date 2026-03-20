📖 **[View on hightechmind.io →](https://hightechmind.io/rust/208-traversal)**

---

# Traversal — Focus on Zero or More Targets
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

A lens focuses on exactly one field; a prism focuses on at most one. A traversal focuses on zero or more values simultaneously. `over_all(traversal, f, structure)` applies `f` to every focused element and returns the updated structure. `collect_all(traversal, structure)` gathers all focused elements into a `Vec`. Traversals generalize `map` and `fold` to any structure: the same traversal that maps over `Vec` elements can map over tree nodes or nested struct fields.

## Learning Outcomes

- Understand traversals as optics that focus on multiple values at once
- Implement `over` and `collect` as the two primitive traversal operations
- Derive higher-level operations (`length_of`, `sum_of`, `all_of`) from primitives
- See how traversals compose with lenses to focus deep inside complex structures

## Rust Application

`Traversal<S, A>` has `over: (Fn(&A) -> A, &S) -> S` (apply function to all focused elements) and `to_list: &S -> Vec<A>` (collect all focused elements). `length_of` is `to_list(&s).len()`. `sum_of` (for numeric `A`) is `to_list(&s).iter().sum()`. `all_of(pred)` is `to_list(&s).iter().all(pred)`. `any_of(pred)` is `to_list(&s).iter().any(pred)`. A traversal for `Vec<f64>` elements, a traversal for tree leaves, and a traversal for struct fields all share this interface.

## OCaml Approach

OCaml's traversals use the Haskell-inspired `Traversable` typeclass:
```ocaml
module type TRAVERSAL = sig
  type s
  type a
  val over : (a -> a) -> s -> s
  val to_list : s -> a list
end
```
OCaml's `List.map`, `Tree.map`, and custom `map` implementations are all traversals. Haskell's `traverse : Applicative f => (a -> f b) -> t a -> f (t b)` is the general version — OCaml simulates this with functors.

## Key Differences

1. **Over and collect**: Both fundamental operations (`over` and `to_list`/`collect`) are identical in structure across languages.
2. **Composition**: Composing a lens with a traversal gives a traversal focused on the lens's target, then traversed; both languages handle this via function composition.
3. **Applicative vs. explicit**: Haskell/OCaml traversals use the `Applicative` functor for general `traverse`; Rust's version uses explicit `over` and `collect` — less general but clearer.
4. **Derived operations**: All aggregate operations (sum, count, any, all) are derived from `collect` — the traversal is a "lens for many targets."

## Exercises

1. Implement a traversal for tree leaves and verify `sum_of` and `length_of` work correctly.
2. Write `modify_at(traversal, index, f, s)` that applies `f` only to the element at a specific index.
3. Compose a lens and a traversal: a lens that focuses on a `Vec<i32>` field, composed with a traversal for the vec's elements.
