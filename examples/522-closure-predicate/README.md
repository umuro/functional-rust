📖 **[View on hightechmind.io →](https://hightechmind.io/rust/522-closure-predicate)**

---

# 522: Predicate Functions Pattern

**Difficulty:** 2  **Level:** Beginner-Intermediate

Compose boolean tests from smaller predicates using `and`, `or`, and `not`.

## The Problem This Solves

Filtering logic in real code gets complex fast: "items that are positive, even, and less than 100" or "strings that start with 'h' and are longer than 3 characters." Writing these as inline closures works but doesn't scale. The boolean conditions can't be reused, tested independently, or composed in new combinations without copy-pasting.

The predicate combinator pattern solves this: each predicate is a `Fn(&T) -> bool`, and you build combinators like `pred_and`, `pred_or`, and `pred_not` that take two predicates and return a new one. The resulting predicates are closures that close over the originals, so they compose freely. You can build a library of named predicates and combine them at the call site.

This is the Rust equivalent of OCaml's function composition and Haskell's predicate combinators. It's a practical example of functional style in production Rust code.

## The Intuition

A predicate is just a function from a value to a boolean. Combining predicates is combining functions. `pred_and(p1, p2)` returns a new closure that calls both and `&&`s the results. Since closures can capture other closures, this composes indefinitely. `all_of(vec![p1, p2, p3])` returns a closure that iterates the list and short-circuits.

## How It Works in Rust

1. **Basic predicate** — `let is_even = |x: &i32| x % 2 == 0;` — a `Fn(&i32) -> bool`.
2. **`pred_and`** — `fn pred_and<T, P1: Fn(&T)->bool, P2: Fn(&T)->bool>(p1: P1, p2: P2) -> impl Fn(&T)->bool { move |x| p1(x) && p2(x) }`.
3. **`all_of`** — takes `Vec<Box<dyn Fn(&T)->bool>>`; returns closure that calls `.iter().all(|p| p(x))`.
4. **Use with `filter`** — `nums.iter().filter(|x| is_valid(x))` — the composed predicate slots directly into iterator adapters.
5. **Negation** — `pred_not(p)` returns `move |x| !p(x)`; negating any predicate produces a new predicate.

## What This Unlocks

- Build complex filter logic from named, testable, reusable predicates.
- Combine predicates at the call site without duplicating boolean logic.
- Use predicate combinators as building blocks for validation pipelines and rule engines.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Predicate composition | `let both f g x = f x && g x` — natural | Same; `pred_and(p1, p2)` returns `impl Fn` closure |
| Higher-order predicates | First-class; no boxing needed | `impl Fn` for generics; `Box<dyn Fn>` for heterogeneous lists |
| `all_of` / `any_of` | `List.for_all`, `List.exists` with predicate list | `all_of(Vec<Box<dyn Fn>>)` via `.iter().all(...)` |
| Use in filter | `List.filter pred` | `.iter().filter(|x| pred(x))` |
