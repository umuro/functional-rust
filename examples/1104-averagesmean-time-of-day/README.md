# Example 1104: Averages/Mean time of day

**Difficulty:** ⭐⭐
**Category:** Math
**OCaml Source:** [Rosetta Code — Averages/Mean time of day](https://rosettacode.org/wiki/Averages/Mean_time_of_day)

## Problem Statement

Compute the mean (average) time of day from a list of "HH:MM:SS" strings.
Because time wraps at midnight, a plain arithmetic average produces nonsensical
results for times that straddle midnight (e.g. the naive mean of 23:00 and
01:00 would be 12:00 instead of 00:00).

## Learning Outcomes

- Why circular/angular averaging is necessary for cyclic quantities (time,
  compass bearings, seasons).
- Using `atan2` and iterator adapters (`map`, `sum`) for numeric reductions.
- Rust's `f64::atan2` call syntax: `y.atan2(x)` vs OCaml's `atan2 y x`.
- Parsing structured strings with `split` + `Option`-chaining (`?` operator).

## OCaml Approach

OCaml converts each time to a radian angle proportional to its fraction of the
day, accumulates the sum of sines and cosines with `List.fold_left`, calls
`atan2` to recover the mean direction, and maps back to seconds.  A negative
result is shifted by one full day.

## Rust Approach

Rust mirrors the OCaml algorithm step for step.  The idiomatic additions are:
iterator chains with `.map().sum()` in place of `fold_left`, the method-call
form `sum_sin.atan2(sum_cos)`, and `Option`-returning parse helpers that use
the `?` operator for early-return on malformed input.

## Key Differences

1. **`atan2` call syntax:** OCaml `atan2 y x` becomes Rust `y.atan2(x)` —
   same math, different notation.
2. **Error handling:** OCaml's `Scanf.sscanf` panics on bad input; the Rust
   version returns `Option<f64>` and propagates errors with `?`.
3. **Iteration:** OCaml uses `List.fold_left` and `List.map`; Rust uses
   `.iter().map(…).sum()` and `.collect()` — both purely functional.
4. **Rounding:** OCaml defines an explicit `round` helper; Rust uses
   `(t + 0.5).floor() as u64` inline.
