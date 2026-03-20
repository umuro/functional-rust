📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1122-leap-year)**

---

# 1122-leap-year — Leap Year
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

A leap year in the Gregorian calendar occurs every 4 years, except for centuries (divisible by 100), which are only leap years if also divisible by 400. The rule has three conditions arranged in a specific priority: `(divisible by 4) AND NOT (divisible by 100) OR (divisible by 400)`. This is a classic exercise in boolean logic and is the first date-calculation building block.

The Gregorian calendar reform in 1582 introduced this rule to keep the calendar aligned with the solar year (365.2425 days). Software that handles dates — from scheduling systems to financial calculations — must implement this rule correctly.

## Learning Outcomes

- Implement the three-part leap year rule correctly
- Express the rule as a boolean formula with correct operator precedence
- Write comprehensive tests covering all four cases (regular, century, leap century, non-leap)
- Understand why the rule exists (solar year alignment)
- Handle edge cases: year 0 (proleptic Gregorian), negative years

## Rust Application

The leap year rule in idiomatic Rust:

```rust
pub fn is_leap_year(year: i32) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}
```

The `src/lib.rs` is a stub, but the implementation is straightforward. Key test cases: 1900 (not a leap year — divisible by 100 but not 400), 2000 (leap year — divisible by 400), 1996 (leap year — divisible by 4, not 100), 1997 (not a leap year — not divisible by 4).

## OCaml Approach

```ocaml
let is_leap_year year =
  year mod 400 = 0 || (year mod 4 = 0 && year mod 100 <> 0)
```

Identical logic. OCaml uses `mod` instead of `%` and `<>` instead of `!=` for not-equal, but the boolean structure is the same.

## Key Differences

1. **Operator precedence**: Both `&&` binds tighter than `||` in Rust and OCaml, making the formula `a || (b && c)` without explicit parentheses — but adding them improves clarity.
2. **Integer division**: Both use remainder (`%` / `mod`) for the divisibility checks; negative years behave differently in each language's modulo semantics.
3. **`chrono` crate**: Production Rust uses the `chrono` crate for date calculations; OCaml uses `Calendar` or `Ptime`.
4. **Calendar systems**: The Gregorian rule applies to years > 1582; proleptic Gregorian extends it backward — `chrono` handles this, raw implementations often do not.

## Exercises

1. Write `days_in_month(year: i32, month: u32) -> u32` using `is_leap_year` for February.
2. Implement `day_of_year(year: i32, month: u32, day: u32) -> u32` that returns the ordinal day (1–365/366).
3. Write a property-based test using `quickcheck` that verifies `is_leap_year` matches the `chrono` crate's answer for all years from 1 to 9999.
