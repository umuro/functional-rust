# Example 983: List.assoc — Association Lists as Simple Maps

**Difficulty:** ⭐
**Category:** stdlib-list
**OCaml Source:** OCaml standard library — `List.assoc`, `List.mem_assoc`, `List.remove_assoc`

## Problem Statement

Association lists are ordered sequences of `(key, value)` pairs used as lightweight maps.
This example implements OCaml's `List.assoc`, `List.mem_assoc`, and `List.remove_assoc` in idiomatic Rust.

## Learning Outcomes

- How Rust slices of tuples `&[(K, V)]` map directly to OCaml association lists
- Using `.find()`, `.any()`, `.filter()` for linear key lookup with zero allocation
- Lifetime annotation patterns when returning references from within a slice
- The first-match-wins semantics that association lists share in both languages

## OCaml Approach

OCaml's `List.assoc` traverses a list of pairs using structural recursion, returning the value
for the first matching key or raising `Not_found`. `List.mem_assoc` returns a boolean,
and `List.remove_assoc` rebuilds the list without the first matching pair — all O(n).

## Rust Approach

Rust uses `&[(K, V)]` slices as the equivalent of OCaml association lists. Iterator combinators
`.find()`, `.any()`, and `.filter()` express the same traversal idioms without explicit
recursion. Lifetime annotations tie the output references back to the input slice, making
borrowing explicit where OCaml's GC would handle it silently.

## Key Differences

1. **Not_found vs Option:** OCaml raises an exception; Rust returns `Option<&V>` for safe, composable error handling.
2. **Lifetime annotations:** Rust requires `'a` to express that returned references live as long as the input slice — OCaml's GC makes this invisible.
3. **Iterator idioms:** `.find()` / `.any()` replace OCaml's explicit `match [] | (k,v)::rest` recursion with declarative chains.
4. **Owned vs borrowed result:** `remove_assoc` returns `Vec<&(K,V)>` — references into the original slice — avoiding the full allocation OCaml's list reconstruction requires.
