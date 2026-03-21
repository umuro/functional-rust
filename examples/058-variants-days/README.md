📖 **[View on hightechmind.io →](https://hightechmind.io/rust/058-variants-days)**

---

# Example 058: Variants — Days of the Week
**Difficulty:** ⭐  
**Category:** Functional Programming  



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

## Exercises

1. Add a method `is_weekend` to the `Day` enum using `match`, and implement `working_days_until` that counts weekdays between two days of the week.
2. Extend the `Day` enum to a `WorkDay { day: Day, hours: f32 }` struct and implement `total_hours` for a slice of `WorkDay` values using an iterator fold.
3. Define a `Month` enum with all 12 months and implement `days_in_month` that accounts for leap years, then write a `calendar_days` iterator that yields every `(Month, u8)` day pair for a given year.
