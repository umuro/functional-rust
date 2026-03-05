# Example 990: Option.value and Option.is_some — Default Values

**Difficulty:** ⭐
**Category:** stdlib-option
**OCaml Source:** OCaml standard library — `Option.value`, `Option.is_some`

## Problem Statement

Extract a value from an `Option`, supplying a fallback when it is `None`. Also
inspect whether an `Option` is populated without consuming it.

## Learning Outcomes

- `unwrap_or` is the direct equivalent of OCaml's `Option.value ~default:x`
- `unwrap_or_else` provides lazy evaluation of the default (closure called only on `None`)
- `.is_some()` / `.is_none()` are built-in methods — no wrapper needed in real code
- Chaining `.map().unwrap_or_else()` is the idiomatic way to transform + default in one step

## OCaml Approach

OCaml's `Option` module exposes named functions: `Option.value ~default:x opt`
extracts the payload or returns `x`. `Option.is_some` tests presence without
consuming the value. The labeled `~default` argument makes call sites read like
English.

## Rust Approach

Rust's `Option<T>` provides the same semantics as methods: `.unwrap_or(default)`
and `.unwrap_or_else(|| default_expr)`. The lazy variant avoids evaluating the
default when it is expensive. `.is_some()` / `.is_none()` replace OCaml's
predicate functions without any boilerplate.

## Key Differences

1. **Syntax:** OCaml uses module-qualified functions (`Option.value`); Rust uses
   method syntax (`opt.unwrap_or(...)`).
2. **Labeled arguments:** OCaml's `~default` is a labeled parameter; Rust passes
   the default as a positional argument.
3. **Lazy defaults:** OCaml has no built-in lazy variant; Rust has
   `unwrap_or_else` for closure-based lazy defaults.
4. **Consumption:** `unwrap_or` consumes the `Option`; `.is_some()` takes `&self`
   and leaves it intact — matching OCaml's non-destructive `Option.is_some`.
