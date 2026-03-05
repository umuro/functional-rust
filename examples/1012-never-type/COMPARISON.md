# The Never Type — Comparison

## Core Insight
Both languages have bottom types for diverging expressions. Rust makes it explicit with `!` and `Infallible`; OCaml uses the polymorphic type variable `'a` and empty variant types.

## OCaml Approach
- `exit`, `raise`, `failwith` have type `'a` — universally quantified acts as bottom
- Empty variant types `type never = |` are uninhabitable (OCaml 4.07+)
- `'a` in return position means "can unify with anything" — similar to `!`

## Rust Approach
- `!` is the never type — functions returning `!` diverge
- `std::convert::Infallible` is the stable uninhabited enum (0 variants)
- `match e {}` on uninhabited types requires no arms
- `unreachable!()` macro expands to `panic!` but returns `!`

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Bottom type | `'a` (polymorphic) | `!` (explicit) |
| Uninhabited type | `type never = \|` | `Infallible` / `!` |
| Diverging function | Returns `'a` | Returns `!` |
| Empty match | Implicit (no constructors) | `match e {}` |
| Infallible Result | Not idiomatic | `Result<T, Infallible>` |
| Unreachable | `assert false` | `unreachable!()` |
