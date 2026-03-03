# Catamorphism — Comparison

## Core Insight
A catamorphism generalizes `fold` to any algebraic data type by replacing each constructor with a function. OCaml's labeled arguments and polymorphic recursion make this pattern concise. Rust needs explicit Clone bounds and `&dyn Fn` for the closure parameters.

## OCaml Approach
- `let rec cata ~leaf ~node = function ...` — labeled args for clarity
- `let size = cata ~leaf:0 ~node:(fun l _ r -> 1 + l + r)` — partial application
- `mirror` returns a tree — same catamorphism, different result type
- Polymorphic: works for any `'a tree`

## Rust Approach
- `pub fn cata<T, R>(tree: &Tree<T>, leaf_val: R, node_fn: &dyn Fn(R, &T, R) -> R)`
- `R: Clone` bound needed because leaf_val is used in multiple branches
- `mirror` can't use the same cata signature (returns Tree, not a fold)
- Separate implementation for operations that build trees

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Constructor replacement | Labeled args | Closure parameters |
| Partial application | `let size = cata ~leaf:0 ~node:...` | Standalone function |
| Clone requirement | None (GC) | `R: Clone` |
| Mirror via cata | Yes (returns tree) | No (different signature) |
| Polymorphism | `'a tree` | `Tree<T>` with bounds |

## Learner Notes
- Catamorphisms are the "design pattern" of functional programming
- In Rust, `&dyn Fn` is needed for recursive closures (can't use generics easily)
- OCaml's labeled arguments (`~leaf`, `~node`) have no direct Rust equivalent
- Consider trait-based visitors as an alternative Rust pattern
