# Example 102: Map Module — Frequency Counter

**Difficulty:** ⭐
**Category:** Data Structures
**OCaml Source:** [OCaml Docs](https://ocaml.org/docs/)

## Problem Statement

Count word frequencies in a text string using a map data structure.

## Learning Outcomes

- Map OCaml's `Map.Make(String)` functor to Rust's `HashMap`/`BTreeMap`
- Master the `entry` API for in-place map updates
- Compare immutable (OCaml) vs mutable (Rust) map patterns

## Key Insight

OCaml's `Map.Make(String)` produces an immutable balanced tree map — every "update" creates a new map. Rust's `HashMap` is mutable by default, and the `entry().or_insert()` API provides elegant in-place updates without double lookups.
