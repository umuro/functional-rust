# Example 1109: ABC Problem

**Difficulty:** ⭐⭐  
**Category:** General / Backtracking  
**OCaml Source:** Rosetta Code — ABC Problem

## Problem Statement

Given a set of 20 lettered blocks (each block shows exactly two letters), determine whether a given word can be spelled by selecting one block per letter, using each block at most once.

## Learning Outcomes

- Backtracking search implemented with slice patterns and `.any()` short-circuit evaluation
- How `partition` in OCaml maps directly to `Iterator::partition` in Rust
- Using index-based removal from a slice to represent "consume one item" without mutation
- Case-insensitive string handling with `.to_uppercase()` before processing

## OCaml Approach

OCaml uses `List.partition` to split available blocks into matching and non-matching, then greedily picks the first matching block and recurses. The approach is clean and declarative, relying on immutable list construction (`res @ remaining`) to model the remaining pool.

## Rust Approach

The idiomatic Rust version tracks block availability by index, using slice patterns (`[c, rest @ ..]`) to destructure the letter list. A call to `.any()` drives the backtracking: it short-circuits on the first successful branch, and the index-based removal avoids any mutation. The functional variant mirrors OCaml more closely with `partition`, but adds proper backtracking by trying all matching blocks, not just the first.

## Key Differences

1. **Greedy vs backtracking:** The OCaml `find_letter` greedily picks the first matching block; the Rust versions try all matching blocks, which is correct for adversarial block sets.
2. **Immutable lists vs index slicing:** OCaml rebuilds lists (`res @ remaining`); Rust builds a new `Vec<usize>` from two slice halves — same semantics, explicit allocation.
3. **Pattern matching on slices:** OCaml's `| [] -> ... | x :: rest -> ...` maps directly to Rust's `[] => ...` and `[c, rest @ ..] => ...` slice patterns.
4. **Case handling:** OCaml compares `Char.uppercase_ascii` lazily; Rust normalises the whole word upfront with `.to_uppercase().chars().collect()`.
