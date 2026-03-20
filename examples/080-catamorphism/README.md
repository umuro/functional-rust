[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 080 — Catamorphism

## Problem Statement

Implement a generalized fold (catamorphism) over a binary tree ADT. The `cata` function replaces each constructor with a provided function, enabling `size`, `sum`, `height`, `mirror`, and `to_vec` to be expressed as single `cata` calls — demonstrating the principle of replacing recursive case analysis with higher-order structure.

## Learning Outcomes

- Understand catamorphisms as the canonical way to eliminate a recursive data type
- Pass `&dyn Fn` closures to avoid monomorphizing `cata` for every use case
- Recognise that `mirror` cannot use the generic `cata` signature because it returns `Tree<T>` (same type)
- See how `leaf_val.clone()` is needed when the accumulator is split across two branches
- Map Rust's `cata` to OCaml's labeled-argument `~leaf ~node` style
- Identify when a direct recursive function is clearer than a catamorphism

## Rust Application

`Tree<T>` is a generic recursive enum with `Leaf` and `Node(Box<Tree<T>>, T, Box<Tree<T>>)`. The `cata` function takes a `leaf_val: R` and a `node_fn: &dyn Fn(R, &T, R) -> R`. Both sub-trees are folded first (`left`, `right`), then combined via `node_fn`. Because `leaf_val` is consumed separately for left and right branches, `R: Clone` is required. `size` and `height` pass integer literals and arithmetic closures. `mirror` is implemented as a direct recursive function because it constructs a new `Tree<T>`, which does not fit the `cata` return type cleanly.

## OCaml Approach

OCaml uses labeled arguments `~leaf` and `~node` for the catamorphism, making call sites self-documenting: `cata ~leaf:0 ~node:(fun l _ r -> 1 + l + r)`. Because OCaml uses structural sharing for lists and native recursive types, there is no need for `Clone`. `mirror` fits naturally as a catamorphism: `~node:(fun l v r -> Node(r, v, l))` simply swaps the already-folded children. The conciseness difference is mainly the absence of `Box` and `Clone`.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Signature | `leaf_val: R, node_fn: &dyn Fn(R, &T, R) -> R` | `~leaf ~node` labeled args |
| Constraint | `R: Clone` for branch splitting | Not needed (structural sharing) |
| Mirror | Separate direct recursion | Expressible as catamorphism |
| Tree node | `Box<Tree<T>>` | `'a tree` (native recursive) |
| Dispatch | `&dyn Fn` (dynamic) | Native closure (monomorphized) |
| Clarity | Verbose but explicit | Concise, labeled |

Catamorphisms unify all structural recursions over a type into a single combinator. When an operation can be expressed as a catamorphism, it gains free structural safety: the recursion is handled once, correctly, and the user only provides the algebra.

## Exercises

1. Add a `depth_at` function using `cata` that returns the depth (level) of the first node with a given value.
2. Implement `flatten_bfs: Tree<T> -> Vec<T>` using a queue rather than `cata`. Compare its readability with the catamorphism approach.
3. Extend `cata` to work on a ternary tree `TTree<T> = Leaf | Node(Box<TTree<T>>, T, Box<TTree<T>>, Box<TTree<T>>)`.
4. Write an `anamorphism` (unfold) dual: given a seed and a step function `S -> Option<(S, T, S)>`, build a `Tree<T>`.
5. In OCaml, use the catamorphism to implement a `map` function `cata ~leaf:Leaf ~node:(fun l v r -> Node(l, f v, r))`. Verify it satisfies the functor laws.
