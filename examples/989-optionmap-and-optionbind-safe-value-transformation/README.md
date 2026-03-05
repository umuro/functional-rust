# Example 989: Option.map and Option.bind — Safe Value Transformation

**Difficulty:** ⭐⭐
**Category:** stdlib-option
**OCaml Source:** OCaml standard library — `Option.map`, `Option.bind`

## Problem Statement

Chain a sequence of fallible operations on optional values without explicit null checks: parse a string to an integer, double it, then divide — propagating `None` automatically at any failure point.

## Learning Outcomes

- `Option::map` transforms the inner value when `Some`, passes `None` unchanged
- `Option::and_then` (OCaml's `Option.bind`) chains operations that themselves return `Option`, flattening nested `Some(Some(x))` into `Some(x)`
- The `?` operator desugars exactly to `and_then`-style early return in `Option`-returning functions
- Early-return match style is an alternative to combinators for readers who prefer imperative control flow

## OCaml Approach

OCaml uses the pipe operator `|>` with `Option.map` and `Option.bind` from the standard library. The pipeline reads left-to-right, and any `None` at an intermediate step short-circuits the rest automatically. OCaml's `Option.bind` takes `'a option -> ('a -> 'b option) -> 'b option`, identical in spirit to Rust's `and_then`.

## Rust Approach

Rust provides `Option::map` and `Option::and_then` as inherent methods, enabling the same pipeline style with method chaining. The `?` operator offers an even more concise alternative: it returns `None` early from the enclosing function without ceremony. Pattern matching with early `return` is also idiomatic when the logic is more complex than a simple closure.

## Key Differences

1. **Syntax:** OCaml uses `|>` (pipe) to thread values; Rust uses `.method()` chaining or `?`
2. **bind vs and_then:** `Option.bind` in OCaml ≡ `Option::and_then` in Rust — same semantics, different name
3. **`?` operator:** Rust has dedicated syntax for Option/Result short-circuit; OCaml requires explicit `Option.bind` or manual match
4. **Type inference:** Both languages infer the inner type; Rust needs a type annotation on `s.parse::<i32>()` or can infer from context
