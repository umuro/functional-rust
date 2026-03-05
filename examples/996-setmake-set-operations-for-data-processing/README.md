# Example 996: Set.Make — Set Operations for Data Processing

**Difficulty:** ⭐⭐
**Category:** stdlib-set
**OCaml Source:** OCaml standard library — `Set.Make` functor

## Problem Statement

Use ordered sets for deduplication, membership testing, and set algebra
(union, intersection, difference) on word lists, mirroring OCaml's
`Set.Make(String)` pattern in idiomatic Rust.

## Learning Outcomes

- How `HashSet` in Rust replaces OCaml's `Set.Make` functor
- Deduplication via `.collect()` into a `HashSet`
- Set algebra methods: `.difference()`, `.union()`, `.intersection()`
- Lifetime annotations on functions that propagate string lifetimes through sets

## OCaml Approach

OCaml requires a functor application (`Set.Make(String)`) to produce a
module with a concrete element type before any set values can be created.
Operations like `of_list`, `diff`, `union`, `inter`, and `mem` are then
called as module-qualified functions.

## Rust Approach

Rust's `HashSet<T>` is generic from the start — no functor step needed.
Deduplication is expressed as `.iter().copied().collect::<HashSet<_>>()`,
and set algebra is provided by `.difference()`, `.union()`, and
`.intersection()`, each returning lazy iterators that must be `.collect()`ed
into a new `HashSet`.

## Key Differences

1. **Functor vs generics:** OCaml uses `Set.Make(String)` to produce a typed set module; Rust uses `HashSet<&str>` directly.
2. **Ordering:** OCaml's `Set` is a balanced BST and always sorted; Rust's `HashSet` is unordered (use `BTreeSet` for sorted output).
3. **Construction:** OCaml has `of_list`; Rust collects any iterator into a `HashSet`.
4. **Set algebra returns:** OCaml returns a new set value; Rust returns a lazy iterator that must be collected.
