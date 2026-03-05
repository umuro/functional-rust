# BTreeSet: Union, Intersection, Difference — Comparison

## Core Insight
Set algebra maps naturally to both languages. OCaml's `Set` module provides functional (immutable) set operations; Rust's `BTreeSet` returns lazy iterators over sorted results, plus operator overloads (`|`, `&`) for ergonomic use.

## OCaml Approach
- `Set.Make(Ord)` functor creates typed set module
- `union`, `inter`, `diff` return new immutable sets
- `subset`, `disjoint` for relationship checks
- `elements` to convert to sorted list
- `filter` and `fold` for derived operations

## Rust Approach
- `BTreeSet<T: Ord>` — generic, no functor needed
- `union()`, `intersection()`, `difference()` return lazy iterators
- `symmetric_difference()` — elements in either but not both
- `is_subset()`, `is_superset()`, `is_disjoint()` for checks
- Operator overloads: `&a | &b` (union), `&a & &b` (intersection)
- `range()` for efficient sub-range queries

## Comparison Table

| Feature | OCaml (`Set`) | Rust (`BTreeSet`) |
|---|---|---|
| Union | `Set.union a b` | `a.union(&b)` / `&a \| &b` |
| Intersection | `Set.inter a b` | `a.intersection(&b)` / `&a & &b` |
| Difference | `Set.diff a b` | `a.difference(&b)` / `&a - &b` |
| Symmetric diff | Manual via union+diff | `a.symmetric_difference(&b)` / `&a ^ &b` |
| Returns | New set (allocated) | Lazy iterator (zero-alloc) |
| Subset check | `Set.subset a b` | `a.is_subset(&b)` |
| Mutability | Immutable | Mutable |
