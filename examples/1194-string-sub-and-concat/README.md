# Example 1194: String.sub and String.concat — Substring and Join

**Difficulty:** ⭐
**Category:** stdlib-string
**OCaml Source:** OCaml Standard Library

## Problem Statement

Extract substrings by position and length, and join a list of strings with a separator — the two most fundamental string assembly operations in OCaml's `String` module.

## Learning Outcomes

- How OCaml's `String.sub` maps to Rust's slice syntax `&s[start..end]`
- Why Rust slices are zero-cost borrows while OCaml `String.sub` allocates a new string
- How `String.concat sep list` becomes `parts.join(sep)` in Rust
- The `Option`-based safe variant vs OCaml's exception-based error handling

## OCaml Approach

OCaml's `String.sub s start len` extracts `len` characters starting at `start`, always returning a fresh allocated string. `String.concat sep list` folds a list into one string with a separator between each pair. Both raise `Invalid_argument` on bad inputs.

## Rust Approach

Rust's slice syntax `&s[start..start+len]` borrows a view into the original string — no allocation. `str::get` provides the same operation with an `Option` return for safe indexing. Joining is `parts.join(sep)`, a single-method call on slices, which allocates exactly once.

## Key Differences

1. **Allocation:** OCaml `String.sub` always allocates; Rust `&s[..]` is a zero-cost borrow.
2. **Error model:** OCaml raises `Invalid_argument` on bad bounds; Rust panics (unsafe) or returns `None` via `.get()` (safe).
3. **Join:** OCaml `String.concat sep list` takes a list; Rust `parts.join(sep)` works on any slice.
4. **Mutability:** OCaml strings are immutable; Rust has both `&str` (immutable borrow) and `String` (owned, mutable).
