# Example 1081: Lenses

**Difficulty:** ⭐⭐⭐
**Category:** Higher-order functions
**OCaml Source:** Real World OCaml

## Problem Statement

Implement lenses — composable functional getters and setters for nested record updates. A lens focuses on a specific field of a structure, allowing you to get, set, and transform it without mutation.

## Learning Outcomes

- How OCaml's record-of-closures pattern translates to Rust structs with boxed closures
- Composing lenses via `Arc`-shared closures to focus through multiple levels of nesting
- The three lens laws (get-set, set-get, set-set) as property-based test foundations
- Why Rust needs explicit `Clone` bounds where OCaml's GC handles structural sharing implicitly

## OCaml Approach

OCaml defines a lens as a record with `get` and `set` closures. Composition is trivial: thread the inner lens through the outer lens's get/set. The `{ p with addr = a }` syntax creates a new record cheaply. Garbage collection handles all intermediate values.

## Rust Approach

Rust uses a struct with `Box<dyn Fn>` closures for the same pattern. Composition requires `Arc` to share the inner lens and outer closures between the composed getter and setter. The setter path needs `Clone` to build new values since Rust has no `{ ..p }` for arbitrary cloning. All updates are pure — the original value is never mutated.

## Key Differences

1. **Record update syntax:** OCaml `{ p with field = v }` is built-in; Rust requires manually constructing a new struct and cloning unchanged fields.
2. **Closure sharing:** OCaml closures are GC-managed values; Rust needs `Arc` to share closures between composed getter and setter.
3. **Type bounds:** OCaml's parametric polymorphism just works; Rust requires `'static + Clone` bounds to store closures in boxes and rebuild values.
4. **Lifetime threading:** OCaml's GC makes composed getters trivial; Rust's composed getter must carefully thread lifetimes through two layers of boxed closures.
