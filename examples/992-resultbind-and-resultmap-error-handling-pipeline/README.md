# Example 992: Result.bind and Result.map — Error Handling Pipeline

**Difficulty:** ⭐⭐
**Category:** stdlib-result
**OCaml Source:** Standard Library — Result module

## Problem Statement

Chain multiple fallible computations — parsing, validation, transformation — so that
the first failure short-circuits the rest and is propagated to the caller unchanged.

## Learning Outcomes

- How OCaml's `Result.bind` maps directly to Rust's `Result::and_then`
- How OCaml's `Result.map` maps directly to Rust's `Result::map`
- The `?` operator as syntactic sugar for the monadic bind pattern
- Why Rust represents errors as values (`Result<T, E>`) rather than exceptions

## OCaml Approach

OCaml uses `Result.bind` (the monadic bind) and the `|>` pipe operator to compose
fallible computations in a linear left-to-right chain. Each step receives the success
value of the previous step; if any step returns `Error`, the chain stops immediately.
`Result.map` applies a pure function to the success value without the ability to fail.

## Rust Approach

Rust provides the same monad structure via `and_then` (= `bind`) and `map` on `Result`.
The combinator style (`and_then` chain) is a direct transliteration of the OCaml pipeline.
Alternatively, the `?` operator desugars each fallible step into an early return on error,
reading like sequential imperative code while preserving the same semantics.

## Key Differences

1. **Error type:** OCaml `result` is `('a, 'b) result`; Rust `Result<T, E>` is generic but the
   error type must be uniform across the chain (or converted with `.map_err`).
2. **`bind` spelling:** OCaml spells it `Result.bind f r`; Rust spells it `r.and_then(f)`.
3. **`?` operator:** Rust provides `?` as first-class syntax for monadic bind in `fn` bodies;
   OCaml 5.x offers `let*` via `Result.bind` syntax extension, but it is less idiomatic.
4. **No exceptions:** Both languages use value-based error handling; there are no try/catch
   semantics — errors are plain data propagated explicitly through the return type.
