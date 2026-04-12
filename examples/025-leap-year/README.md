[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 025 — Leap Year
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Determine whether a year is a leap year using the Gregorian calendar rule: a year is a leap year if it is divisible by 400, or divisible by 4 but not by 100. Express this as a single boolean expression and verify on representative examples.

## Learning Outcomes

- Express multi-condition boolean logic with `||` and `&&` and correct operator precedence
- Understand the three-tier hierarchy: 400 overrides 100, which overrides 4
- Use Rust's `u32` type for years with `%` modulo operator
- Compare directly with OCaml's `mod` and `<>` operators
- Write test cases covering each branch of the rule
- Recognise that a single-expression implementation is clearer than nested if/else

## Rust Application

The implementation is `(year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)`. Because `&&` binds tighter than `||`, the second operand of `||` is correctly interpreted as "divisible by 4 AND NOT divisible by 100". The expression is branchless and O(1). Test cases should verify: 400-divisible years (2000, 1600) are leap; 100-divisible non-400 (1900, 1800) are not; 4-divisible non-100 (2004, 2024) are leap; and non-4-divisible (2001, 2003) are not.

## OCaml Approach

`let leap_year year = (year mod 400 = 0) || (year mod 4 = 0 && year mod 100 <> 0)` is identical in structure. The differences are purely syntactic: `mod` vs `%`, `=` vs `==`, `<>` vs `!=`. Both implementations are equivalent, correct, and O(1).

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Modulo | `%` | `mod` |
| Equality | `== 0` | `= 0` |
| Not equal | `!= 0` | `<> 0` |
| Type | `u32` | `int` |
| Boolean ops | `&&`, `\|\|` | `&&`, `\|\|` |
| Expression length | Same | Same |

The leap year problem is one of the simplest demonstrations of translating between Rust and OCaml — the algorithm is identical, only the surface syntax changes. The canonical single-expression form is preferred over branching because it maps directly to the mathematical definition.

## Exercises

1. Implement `days_in_february(year: u32) -> u32` returning 29 for leap years and 28 otherwise.
2. Write `is_leap_year_julian(year: u32) -> bool` using only the 4-year rule (used before 1582).
3. Count how many leap years are in the range 1900..=2100 using `filter` and `count`.
4. Find the next leap year after a given year using `(year+1..).find(|&y| is_leap_year(y))`.
5. In OCaml, write `years_until_next_leap : int -> int` that returns how many years until the next leap year.
