# Example 987: String — Trim, Uppercase, Contains

**Difficulty:** ⭐
**Category:** stdlib-string
**OCaml Source:** Real World OCaml / standard library exercises

## Problem Statement

Apply the three most common string transformations: strip leading and trailing
whitespace, convert case, and search for a substring. These are the bread-and-butter
operations of text processing in both OCaml and Rust.

## Learning Outcomes

- `str::trim()` returns a borrowed slice of the original — zero allocation
- `str::to_uppercase()` / `str::to_lowercase()` always allocate a new `String`
- `str::contains()` accepts any `Pattern`, including `&str`, `char`, and closures
- Recursive OCaml `find` translates naturally to a nested tail-recursive Rust function
- Iterator `.any()` + `starts_with` gives a declarative windowed search

## OCaml Approach

OCaml uses `String.trim`, `String.uppercase_ascii`, and `String.lowercase_ascii`
from the standard library. Substring search has no `String.contains` equivalent in
the base library, so the idiomatic OCaml approach is a hand-written recursive
`find` function that walks index positions — exactly what the example shows.

## Rust Approach

Rust's `&str` type has `.trim()`, `.to_uppercase()`, `.to_lowercase()`, and
`.contains()` built in. The idiomatic one-liner is a simple method chain with no
allocation until `to_uppercase()`. For the OCaml parallel, a recursive inner
function `find(s, needle, i)` matches the OCaml pattern structurally, and a third
variant using `(0..=n).any(|i| …)` shows how iterators replace explicit indexing.

## Key Differences

1. **Allocation:** OCaml strings are mutable heap values; Rust `&str` is a borrowed
   slice — `trim()` returns a view into the original with no copy.
2. **Case conversion:** Both languages produce a new string; Rust names the functions
   `to_uppercase` / `to_lowercase`, OCaml uses `uppercase_ascii` / `lowercase_ascii`.
3. **Substring search:** OCaml stdlib has no `String.contains`; Rust `str::contains`
   is built-in and accepts flexible patterns.
4. **Recursive search:** OCaml's idiomatic recursive `find` translates
   directly — Rust allows nested `fn` definitions for the same structure.
