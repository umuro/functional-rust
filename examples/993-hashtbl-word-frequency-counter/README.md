# Example 993: Hashtbl — Word Frequency Counter

**Difficulty:** ⭐
**Category:** stdlib-hashtbl
**OCaml Source:** Standard Library — Hashtbl module

## Problem Statement

Count how many times each word appears in a string of text, producing a
frequency map from word to occurrence count.

## Learning Outcomes

- How OCaml's `Hashtbl` maps to Rust's `HashMap` from `std::collections`
- Using `HashMap::entry` + `or_insert` for concise, idiomatic counter patterns
- Building an accumulator with `Iterator::fold` instead of a mutable loop
- Sorting a `Vec` of pairs by multiple keys with `sort_by` and `Ordering::then`

## OCaml Approach

OCaml uses the imperative `Hashtbl` module: `Hashtbl.create` allocates a hash
table, `Hashtbl.find` looks up a key (raising `Not_found` on miss), and
`Hashtbl.replace` updates the binding. The pattern `try find … with Not_found -> 0`
is idiomatic for a "get or default" operation.

## Rust Approach

Rust's `HashMap` in `std::collections` provides the same functionality with a
safer API. The `entry` API combines lookup and insertion into a single operation
— `map.entry(key).or_insert(0)` returns a mutable reference to the existing
value or inserts `0` and returns a reference to that. Dereferencing and
incrementing in one line (`*ref += 1`) is idiomatic and avoids a second lookup.

## Key Differences

1. **Error handling on miss:** OCaml raises `Not_found`; Rust's `entry` API
   returns an `OccupiedEntry` or `VacantEntry` — no exceptions involved.
2. **Mutability:** OCaml's `Hashtbl` is always mutable; Rust requires `mut` to
   be declared explicitly, making mutation visible at the call site.
3. **Whitespace splitting:** OCaml's `String.split_on_char ' '` splits on
   a single space and produces empty strings for runs; Rust's
   `split_whitespace` handles all Unicode whitespace and skips empty tokens.
4. **Functional fold:** Rust's iterator `fold` lets you build the map without
   any explicit mutation in the outer scope, mirroring a functional accumulator
   pattern.
