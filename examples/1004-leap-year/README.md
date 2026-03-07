# Leap Year — Boolean Logic & Divisibility

## Topic
Simple boolean logic using divisibility rules to determine leap years in the Gregorian calendar.

## Difficulty
Beginner

## Functional Insight
This trivial example demonstrates how **Rust's boolean operators directly translate OCaml's logic**:
- OCaml: `(year mod 400 = 0) || (year mod 4 = 0 && year mod 100 <> 0)`
- Rust: `(year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)`

There's no "clever" functional pattern here — just *pure* boolean evaluation. The leap year rule itself is:
- Leap if divisible by 400, **OR**
- Leap if divisible by 4 **AND NOT** divisible by 100

## Canonical Test Cases
- 2000: leap (divisible by 400)
- 1900: not leap (divisible by 100, not 400)
- 2004: leap (divisible by 4, not 100)
- 2001: not leap (not divisible by 4)
