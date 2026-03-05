# Example 991: Option.iter and Option.fold — Side Effects and Folding

**Difficulty:** ⭐
**Category:** stdlib-option
**OCaml Source:** Standard Library — `Option.iter`, `Option.fold`

## Problem Statement

Perform a side effect (e.g. printing) only when an optional value is `Some`, and
collapse an `Option` to a plain value by supplying a default for the `None` case and
a transformation for the `Some` case.

## Learning Outcomes

- How OCaml's `Option.iter` maps to `Option::iter().for_each(…)` or `if let Some(x) = opt { … }` in Rust
- How OCaml's `Option.fold ~none:d ~some:f` maps to `opt.map_or_else(|| d, f)` in Rust
- Why Rust's `Option` implements `IntoIterator` (yielding 0 or 1 elements), enabling iterator combinators
- When to prefer `map_or_else` over an explicit `match` for ergonomics

## OCaml Approach

OCaml's `Option` module provides first-class `iter` and `fold` functions.
`Option.iter f opt` calls `f v` when `opt = Some v` and does nothing otherwise —
it is the `for_each` of the option world.
`Option.fold ~none:d ~some:f opt` generalises this to value production: it returns
`d` for `None` and `f v` for `Some v`, which is exactly a right-fold over a
zero-or-one element container.

## Rust Approach

Rust's `Option<T>` implements `IntoIterator`, so `opt.iter()` yields a
`std::option::Iter<T>` with at most one element.  Calling `.for_each(…)` on it
mirrors `Option.iter` precisely.  For `Option.fold`, `map_or_else(|| none, f)` is
the idiomatic translation: it is lazy (closures are only called when needed) and
avoids unnecessary allocations.  An explicit `match` is always valid and sometimes
clearer for complex branches.

## Key Differences

1. **Side-effect combinator:** OCaml has `Option.iter`; Rust uses `option.iter().for_each(…)` or `if let`.
2. **Fold/collapse:** OCaml's `Option.fold ~none ~some`; Rust's `map_or` (eager) / `map_or_else` (lazy).
3. **Iterator protocol:** Rust's `Option` is itself an iterator — you can chain `.filter_map`, `.flat_map`, etc. directly on it.
4. **Ownership:** Rust must choose between `opt.iter()` (borrow) vs consuming `opt.into_iter()` (move).
