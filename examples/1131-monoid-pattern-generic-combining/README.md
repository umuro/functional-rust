# Example 1131: Monoid Pattern — Generic Combining

**Difficulty:** ⭐⭐
**Category:** Type Classes & Abstractions
**OCaml Source:** Real World OCaml — first-class modules as type-class witnesses

## Problem Statement

Implement a generic `concat_all` function that folds any list of values into one, parameterised by a Monoid: a type with an identity element (`empty`) and an associative binary operation (`combine`). Demonstrate with sum, product, string concatenation, and boolean-AND instances.

## Learning Outcomes

- How OCaml first-class modules translate to Rust traits
- Using newtype wrappers to provide multiple monoid instances for the same underlying type
- How `fold` over an iterator replaces explicit recursive descent
- The identity law as a property-based test pattern

## OCaml Approach

OCaml represents the monoid interface as a module type (`MONOID`) and passes module values as first-class arguments via `(module Sum)` syntax. `concat_all` receives a module witness at the call site and dispatches `M.empty` and `M.combine` through it.

## Rust Approach

Rust uses a trait `Monoid` with associated functions `empty()` and `combine()`. Because Rust cannot have multiple `impl Monoid for i32` blocks, each instance (sum, product) lives in its own newtype wrapper. `concat_all` is generic over any `M: Monoid`, and the iterator `.fold` does the work.

## Key Differences

1. **Interface mechanism:** OCaml uses module types + first-class modules; Rust uses traits + newtypes.
2. **Multiple instances:** OCaml separates instances at the call site with `(module Sum)`; Rust separates them via distinct newtype structs.
3. **Identity element:** OCaml `M.empty` is a value; Rust `M::empty()` is an associated function (called once per fold, matching the value semantics).
4. **Fold:** Both use left fold — OCaml `List.fold_left`, Rust `Iterator::fold` — producing identical evaluation order.

## Exercises

1. Implement a `HashMap` monoid whose binary operation merges two maps by summing numeric values for shared keys.
2. Write a `fold_map` function that applies a function `f: T -> M` to each element before combining with the monoid, and use it to compute the sum of squares in one pass.
3. Demonstrate the associativity law of your monoid implementation with a property-based test: for any three values `a`, `b`, `c`, verify `combine(combine(a,b),c) == combine(a,combine(b,c))`.
