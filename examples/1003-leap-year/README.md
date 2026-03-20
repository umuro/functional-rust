[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 1003 — Leap Year

## Problem Statement

Determine whether a year is a leap year using the Gregorian calendar rules: a year is a leap year if it is divisible by 400, or divisible by 4 but not divisible by 100. Implement `is_leap_year(year: u32) -> bool` as a single boolean expression and verify on 2000 (leap), 1900 (not), 2004 (leap), 2001 (not).

## Learning Outcomes

- Express the Gregorian leap year rule as a compact boolean expression
- Use `%` modulo and `&&`/`||` with correct precedence (no extra parentheses needed)
- Write tests covering all four rule branches: 400-divisible, 100-divisible, 4-divisible, none
- Understand that `(year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)` is the canonical form
- Map Rust's `u32` year and `%` operator to OCaml's `int` and `mod`
- Recognise that clean single-expression logic eliminates branching error

## Rust Application

`is_leap_year(year: u32) -> bool` returns `(year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)`. The `&&` operator binds tighter than `||`, making the parentheses around each operand of `||` clear but technically optional. The `u32` type prevents negative year values, which simplifies the domain. Tests cover all four cases: year 2000 is divisible by 400 (leap), 1900 is divisible by 100 but not 400 (not leap), 2004 is divisible by 4 but not 100 (leap), and 2001 is not divisible by 4 (not leap).

## OCaml Approach

`let leap_year year = (year mod 400 = 0) || (year mod 4 = 0 && year mod 100 <> 0)` is the direct translation. The logic is identical; only the syntax differs: `mod` instead of `%`, `=` instead of `==`, `<>` instead of `!=`. Both implementations are O(1) and branchless.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Modulo | `year % 4` | `year mod 4` |
| Equality | `== 0` | `= 0` |
| Not equal | `!= 0` | `<> 0` |
| Year type | `u32` (unsigned) | `int` (signed) |
| Boolean ops | `&&`, `\|\|` | `&&`, `\|\|` |
| Branchless | Yes | Yes |

This is one of the cleanest cross-language demonstrations: the algorithm is identical, the syntax differs only in operator spelling. Use it to show that problem-solving translates directly between languages.

## Exercises

1. Write a version using a `match` on `(year % 400, year % 100, year % 4)` tuples and compare readability.
2. Implement `days_in_year(year: u32) -> u32` that returns 366 for leap years and 365 for non-leap years.
3. Write `next_leap_year(year: u32) -> u32` that finds the first leap year strictly after the given year.
4. Count leap years in a range using `(1..=2100).filter(|&y| is_leap_year(y)).count()`.
5. In OCaml, implement `leap_years_between : int -> int -> int list` that returns all leap years in the inclusive range.
