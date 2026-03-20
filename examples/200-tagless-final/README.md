📖 **[View on hightechmind.io →](https://hightechmind.io/rust/200-tagless-final)**

---

# Example 200: Tagless Final

**Difficulty:** ⭐⭐⭐
**Category:** DSLs | Type System
**OCaml Source:** Kiselyov — "Finally Tagless, Partially Evaluated"

## Problem Statement

Embed a typed DSL directly as trait/module methods so that multiple interpreters (evaluate, pretty-print, type-check…) can share one program definition without building an intermediate AST.

## Learning Outcomes

- How OCaml module types with abstract type constructors (`type 'a repr`) map to Rust traits with Generic Associated Types (`type Repr<T>`)
- Why Generic Associated Types (GATs) are the key enabler for tagless final in Rust
- The contrast between *initial encoding* (explicit AST enum) and *final encoding* (trait methods): the former centralises interpretation, the latter distributes it
- How `Repr<T> = T` (evaluate) and `Repr<T> = String` (pretty-print) express two completely different semantics through the same interface

## OCaml Approach

OCaml uses a first-class module type `EXPR` with an abstract type constructor `'a repr`. Each interpreter is a module that instantiates `repr` differently — `Eval` sets `type 'a repr = 'a` (the identity) and `Pretty` sets `type 'a repr = string`. The program is a polymorphic function parameterised over any module satisfying `EXPR`, so calling it with different modules yields different results.

## Rust Approach

Rust replaces the OCaml module type with a trait `Expr` whose associated type `Repr<T>` is a Generic Associated Type (GAT). Each interpreter is a zero-sized struct (`Eval`, `Pretty`) implementing `Expr`. The program is a generic function `fn program<E: Expr>() -> E::Repr<i64>` — calling it with `program::<Eval>()` evaluates; `program::<Pretty>()` pretty-prints. No runtime dispatch, no heap allocation.

## Key Differences

1. **Type abstraction:** OCaml uses `type 'a repr` (higher-kinded); Rust uses `type Repr<T>` (Generic Associated Type, stable since 1.65)
2. **Module vs. trait:** OCaml passes interpreters as first-class modules; Rust uses monomorphisation over a type parameter
3. **`bool` keyword:** OCaml names the constructor `bool`; Rust must rename to `bool_val` since `bool` is a built-in type
4. **Zero-cost:** Both encodings are zero-cost — no boxing or dynamic dispatch in the final encoding

## Exercises

1. Add a new `Console` interpreter for the tagless-final algebra that prints each expression to stdout instead of evaluating it, demonstrating the open extension property.
2. Implement a `Pretty` interpreter that renders arithmetic expressions back to a human-readable string, reusing the same algebra trait without modifying existing code.
3. Define a larger algebra that includes both arithmetic and boolean operations, combine the two interpreters using trait composition, and implement a short-circuit evaluator that skips the right operand of `And` when the left is `false`.
