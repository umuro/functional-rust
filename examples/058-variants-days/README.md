📖 **[View on hightechmind.io →](https://hightechmind.io/rust/058-variants-days)**

---

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

1. **Enum as closed set:** Rust's `enum Day` and OCaml's `type day = Mon | Tue | ...` are closed sum types — the set of values is fixed at definition time. This enables exhaustiveness checking.
2. **Pattern matching vs `switch`:** Both languages check exhaustiveness at compile time. Java's `switch` on enums checks exhaustiveness only with `--enable-preview` in Java 17+. C's `switch` has no exhaustiveness checking.
3. **Deriving `PartialEq`, `Debug`:** Rust's `#[derive]` automatically generates equality and debug printing for enums. OCaml's structural equality `(=)` works on variant types without any annotation.
4. **Methods on enums:** Rust's `impl Day` block adds methods directly to the enum. OCaml uses standalone functions (no method syntax). This is a major ergonomic difference.

## Exercises

1. Add a method `is_weekend` to the `Day` enum using `match`, and implement `working_days_until` that counts weekdays between two days of the week.
2. Extend the `Day` enum to a `WorkDay { day: Day, hours: f32 }` struct and implement `total_hours` for a slice of `WorkDay` values using an iterator fold.
3. Define a `Month` enum with all 12 months and implement `days_in_month` that accounts for leap years, then write a `calendar_days` iterator that yields every `(Month, u8)` day pair for a given year.

4. **Day arithmetic**: Implement `days_until(from: Day, to: Day) -> usize` that returns the number of days from `from` to `to`, wrapping around (Monday to Friday = 4, Friday to Monday = 3).
5. **Business days**: Implement `is_business_day(day: Day) -> bool` and `next_business_day(day: Day) -> Day` using pattern matching. Then implement `business_days_between(from: Day, to: Day) -> usize`.
