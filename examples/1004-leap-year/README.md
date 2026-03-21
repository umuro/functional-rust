[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 1004 — Leap Year
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Determine whether a year is a leap year using the Gregorian calendar rules: divisible by 400, OR divisible by 4 but not 100. Implement the single boolean expression `(year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)` and verify it on edge cases (2000, 1900, 2004, 2100). Compare with OCaml's equivalent.

## Learning Outcomes

- Express multi-condition boolean logic with `||` and `&&` and correct precedence
- Understand the three-case leap year rule: 400 overrides 100, which overrides 4
- Use Rust's `u32` type for years — no negative years needed
- Write thorough test cases covering each branch: divisible by 400, by 100, by 4, and none
- Map Rust's boolean expression directly to OCaml's `mod` operator syntax
- Recognise that clean logic > branching: one expression beats if/else chain

## Rust Application

`is_leap_year(year: u32) -> bool` is the entire implementation. The expression `(year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)` correctly handles all cases. `&&` binds tighter than `||`, so the second term is correctly read as "divisible by 4 AND NOT divisible by 100". Tests cover: 2000/1600 (400-divisible), 1900/1800 (100-divisible), 2004/2024 (4-divisible), 2001/2003/2100 (non-leap).

## OCaml Approach

OCaml's `let leap_year year = (year mod 400 = 0) || (year mod 4 = 0 && year mod 100 <> 0)` is structurally identical. The `mod` operator corresponds to `%`; `<>` corresponds to `!=`. The logic and precedence rules are the same. This is one of the simplest cross-language comparisons: the two implementations differ only in surface syntax.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Modulo | `%` | `mod` |
| Not equal | `!=` | `<>` |
| Year type | `u32` | `int` |
| Boolean ops | `&&`, `\|\|` | `&&`, `\|\|` |
| Return type | `-> bool` | Inferred `bool` |
| Complexity | O(1) | O(1) |

This example demonstrates that some algorithms are language-independent at the semantic level — only the syntax changes. The key skill is recognising the three-tier hierarchy in the leap year rule and expressing it as a single minimal boolean formula.

## Exercises

1. Implement a version using a `match` on `(year % 400, year % 100, year % 4)` and compare readability.
2. Write a function `leap_years_in_range(start: u32, end: u32) -> Vec<u32>` using `.filter(|&y| is_leap_year(y))`.
3. Count how many leap years occur in the 20th century (1901–2000).
4. Implement an `is_leap_year_julian(year: u32) -> bool` using only the 4-year rule (no century exception).
5. In OCaml, implement `next_leap_year : int -> int` that finds the first leap year after the given year.
