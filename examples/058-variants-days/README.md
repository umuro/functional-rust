# Example 058: Variants — Days of the Week

**Difficulty:** ⭐
**Category:** Algebraic Data Types
**OCaml Source:** [CS3110 — Variants](https://cs3110.github.io/textbook/chapters/data/variants.html)

## Problem Statement

Model the days of the week as an algebraic data type. Implement `day_name`, `is_weekend`, and `next_day` using exhaustive pattern matching.

## Learning Outcomes

- Map OCaml variants to Rust enums (fieldless / C-like)
- See how both languages enforce exhaustive pattern matching
- Compare OCaml's free functions with Rust's `impl` methods
- Use `#[derive]` to get traits OCaml variants have implicitly
- Explore numeric discriminants as an alternative to match

## OCaml Approach

OCaml variants are lightweight — declare the type, write functions with `match`/`function`. Equality, printing, and copying come free. All functions are standalone (no method syntax).

## Rust Approach

1. **Idiomatic:** Enum with `impl` block methods (`self.name()`, `self.is_weekend()`, `self.next()`)
2. **Functional:** Free functions mirroring OCaml's style
3. **Numeric:** Using discriminant values and arithmetic for `next_day`

## Key Differences

1. **Derive macros:** Rust requires explicit `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`; OCaml gets these implicitly
2. **Methods vs functions:** Rust idiom puts functions in `impl` blocks; OCaml uses standalone functions
3. **Display:** Rust needs explicit `Display` impl; OCaml can derive `show` via ppx or manually print
4. **Copy semantics:** `Copy` must be opted into in Rust; OCaml values are always copyable
5. **Exhaustiveness:** Both compilers warn on non-exhaustive matches — this is a shared strength
