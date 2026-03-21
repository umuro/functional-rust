📖 **[View on hightechmind.io →](https://hightechmind.io/rust/578-pattern-exhaustiveness)**

---

# Pattern Exhaustiveness
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Exhaustiveness checking is one of the most valuable compile-time guarantees in a language with algebraic data types. When you add a new variant to an enum, the compiler immediately points to every `match` expression that does not handle it — no runtime surprises, no silent fallthrough to wrong behavior. This makes refactoring safe: you cannot forget to handle new cases. It is why functional programmers value algebraic data types over class hierarchies with virtual dispatch, and why Rust and OCaml are preferred for compiler writing, protocol implementations, and state machines.

## Learning Outcomes

- How the Rust compiler verifies that `match` covers all cases
- How adding a new variant to an enum causes compile errors in unupdated matches
- How `_` wildcard provides a catch-all that satisfies exhaustiveness
- How `#[non_exhaustive]` allows library enums to add variants without breaking downstream code
- Where exhaustiveness checking prevents real bugs: state machines, command dispatch, protocol handling

## Rust Application

`describe(d: Dir)` matches `N`, `S`, `E`, `W` — exhaustive, no `_` needed. Adding `NE` to `Dir` causes a compile error in `describe` until all matches are updated. `match_with_wildcard` shows `_ => "other"` as a catch-all. `#[non_exhaustive]` on a public enum prevents external code from writing exhaustive matches — external code must use `_` to handle future variants. The source demonstrates how `match n { ... }` on integers requires `_` because integers are infinite.

Key patterns:
- Exhaustive enum match without `_`
- `_ =>` catch-all for extensible enums or infinite domains
- `#[non_exhaustive]` for public enums
- Compiler warning: "non-exhaustive patterns" when a variant is missing

## OCaml Approach

OCaml has the same exhaustiveness checking:

```ocaml
type dir = N | S | E | W
let describe = function
  | N -> "north" | S -> "south" | E -> "east" | W -> "west"
(* Adding NE causes: Warning 8: this pattern-matching is not exhaustive *)
```

Both compilers warn on missing variants when the `_` wildcard is absent.

## Key Differences

1. **Warning vs error**: Rust exhaustiveness failure is a compile error; OCaml's is a warning (treated as error with `-warn-error`).
2. **`#[non_exhaustive]`**: Rust has `#[non_exhaustive]` for library extensibility; OCaml achieves this with private constructors or abstract module signatures.
3. **Integer exhaustiveness**: Both languages require `_` for integer matches since integers have infinite cases; both compile without `_` for finite closed enums.
4. **Nested exhaustiveness**: Both check exhaustiveness recursively — a nested enum variant missing coverage causes an error at the appropriate depth.

## Exercises

1. **Add variant**: Add `NE, NW, SE, SW` to the `Dir` enum and update all match expressions — observe exactly which files and lines the compiler flags.
2. **Non-exhaustive library**: Implement a `#[non_exhaustive] pub enum ApiError` and show that external code using it must include a `_` arm — explain what future-proofing this provides.
3. **Nested exhaustiveness**: Create `enum Outer { A(Inner), B }; enum Inner { X, Y }` and write a match on `Outer` with nested `Inner` patterns — verify all four cases are covered.
