# Example 985: String.split_on_char — Tokenize a String

**Difficulty:** ⭐
**Category:** stdlib-string
**OCaml Source:** OCaml standard library — `String.split_on_char`

## Problem Statement

Split a string into a list of substrings using a single delimiter character,
then optionally filter out empty segments that arise when delimiters appear
consecutively or at the edges of the input.

## Learning Outcomes

- How `str::split(char)` maps directly onto OCaml's `String.split_on_char`
- Why Rust returns an iterator (`Split<char>`) while OCaml returns a `string list`
- How `filter` on an iterator replaces `List.filter` for empty-token removal
- How `str::split_whitespace` provides a batteries-included alternative for word tokenisation
- How recursive string slicing in Rust mirrors OCaml's recursive list destructuring

## OCaml Approach

OCaml's `String.split_on_char delimiter s` returns a `string list`.  Empty
strings appear in the result wherever the delimiter is adjacent to itself or to
the string boundaries.  Filtering them requires a separate `List.filter` pass.
The recursive formulation uses pattern matching on the index of the first
delimiter found and rebuilds the list tail-recursively.

## Rust Approach

`str::split(delimiter)` is a lazy iterator — no allocation until `.collect()`.
The result type is `Vec<&str>`, borrowing slices of the original string rather
than allocating new `String` values, which is more efficient.  `.filter()` is
chained on the iterator before collection, replacing `List.filter`.
`split_whitespace()` handles consecutive-whitespace collapsing in one call.

## Key Differences

1. **Return type:** OCaml returns `string list` (owned); Rust returns `Vec<&str>` (borrowed slices — zero copy).
2. **Empty tokens:** Both include empty strings by default; both need an explicit filter step to remove them.
3. **Whitespace:** Rust provides `split_whitespace()` which collapses runs of whitespace; OCaml requires manual filtering.
4. **Lazy vs eager:** Rust's `split` is a lazy iterator; OCaml's function produces the full list immediately.
