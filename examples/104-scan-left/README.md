# Example 104: Scan Left — Running Accumulation

**Difficulty:** ⭐
**Category:** Higher-Order Functions
**OCaml Source:** [OCaml Docs](https://ocaml.org/docs/)

## Problem Statement

Implement `scan_left` — like `fold_left` but returns all intermediate accumulator values, not just the final result.

## Learning Outcomes

- Map OCaml's custom `scan_left` to Rust's `Iterator::scan`
- Understand scan as "fold with history"
- Compare eager collection vs lazy scan

## Key Insight

OCaml has no built-in `scan_left` — you build it with `fold_left` and a result list. Rust has `Iterator::scan` built in, which lazily produces intermediate results. The relationship: `fold` returns the last element of what `scan` returns.
