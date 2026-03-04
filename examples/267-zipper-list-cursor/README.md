# Example 267: Zipper List Cursor

**Difficulty:** ⭐⭐
**Category:** Data Structures | Functional Patterns
**OCaml Source:** Classic functional programming — Huet (1997) "The Zipper"

## Problem Statement

Implement a *zipper* — a data structure that represents a list with a moveable focus point, supporting O(1) navigation left and right, and O(1) update of the focused element, while retaining the ability to reconstruct the full list at any time.

## Learning Outcomes

- How algebraic "context" structures (left/focus/right) translate from OCaml records to Rust structs
- Consuming `self` by value in navigation functions mirrors OCaml's persistent immutable style
- Rust's `Vec` as the backing store for the reversed left spine and the right tail
- Providing both a free-function API (close to OCaml) and a method API (idiomatic Rust)

## OCaml Approach

OCaml uses a record `{ left; focus; right }` where `left` is stored reversed (the head is the immediate left neighbour). Pattern matching on `go_right` and `go_left` is exhaustive and returns `option` for boundary cases. `update` uses the `{ z with focus = ... }` record update syntax.

## Rust Approach

Rust represents the same structure as a generic `struct Zipper<T>` with three `Vec<T>` fields. Navigation functions consume the zipper by value and return `Option<Zipper<T>>`, preserving immutability semantics. `update` uses struct spread `{ focus: f(z.focus), ..z }` — Rust's equivalent of OCaml's `{ z with focus = ... }`.

## Key Differences

1. **Record update:** OCaml `{ z with focus = f z.focus }` ↔ Rust `Zipper { focus: f(z.focus), ..z }` — nearly identical syntax
2. **Ownership:** OCaml's GC handles sharing transparently; Rust consumes the zipper by value, making it clear there is exactly one owner at each navigation step
3. **Boundary values:** Both use `Option` / `None` for out-of-bounds navigation — the types map directly
4. **API style:** Rust idiom adds a method API (`z.move_right()`) on top of the free-function API (`go_right(z)`) to suit different call-site styles
