# Example 1106: Abbreviations, Easy

**Difficulty:** ⭐⭐
**Category:** String Processing | Pattern Matching
**OCaml Source:** Rosetta Code — Abbreviations, easy

## Problem Statement

Given a command table where each entry encodes its minimum abbreviation via mixed case
(e.g. `"ALTer"` means the minimum is `"ALT"`), resolve a list of user-supplied words
to their canonical uppercase command names, or `"*error*"` if no match exists.

## Learning Outcomes

- Using `char::is_uppercase` as a predicate in iterator chains to extract structural information from strings
- Lifetime annotations on functions returning references borrowed from slice elements
- Building lookup tables as `Vec<(String, String)>` and borrowing them as `&[(&str, &str)]`
- How `str::split_whitespace` cleanly handles irregular spacing in data files
- Translating OCaml's `List.find_opt` + `match` pattern to Rust's `Iterator::find` + `map/unwrap_or`

## OCaml Approach

OCaml builds the command table with `List.map`, using `Seq.filter char_is_uppercase` to
extract minimum abbreviations and `String.uppercase_ascii` for the canonical form. Lookup
uses `List.find_opt` and returns `Some (_, found)` or `None`, matched to produce either
the command or `"*error*"`.

## Rust Approach

Rust uses iterator chains throughout: `chars().filter(|c| c.is_uppercase()).collect()` for
abbreviation extraction, `split_whitespace().map(...).collect()` for table construction, and
`iter().find(...)` for lookup. Lifetime parameters (`'a`) thread the borrowed `&str` from the
table slice through to the return value, avoiding any allocation in the hot path.

## Key Differences

1. **String extraction:** OCaml uses `String.to_seq` + `Seq.filter` + `String.of_seq`; Rust uses `.chars().filter().collect()` — same pipeline, different names.
2. **Lifetimes:** OCaml's GC makes lifetime implicit; Rust requires `'a` to express that the returned `&str` borrows from the command table slice.
3. **Lookup:** OCaml's `List.find_opt` returns `option`; Rust's `Iterator::find` returns `Option<&T>`, then `.map()` + `.unwrap_or()` replaces the `match` expression.
4. **Case conversion:** Both use a standard library function (`String.uppercase_ascii` / `str::to_uppercase`); Rust's returns a new `String`, OCaml's returns a new string too.
