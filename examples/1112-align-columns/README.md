# Example 1112: Align Columns

**Difficulty:** ⭐⭐  
**Category:** General / String Formatting  
**OCaml Source:** Rosetta Code — Align columns

## Problem Statement

Given text with fields separated by a single `$` character, format the output
so every column is aligned: each field padded to the width of the widest field
in its column, with words left-justified, right-justified, or center-justified.

## Learning Outcomes

- How Rust's format string mini-language (`{:<w$}`, `{:>w$}`, `{:^w$}`) replaces
  manual padding arithmetic from OCaml.
- Composing `lines()` + `split()` + `collect()` to parse 2D text structure.
- Computing column widths functionally with `(0..n).map(|col| rows.filter_map(...).max())`.
- Returning formatted `String` values from library functions instead of printing.

## OCaml Approach

OCaml splits on `\n` and `$` using the `Str` library's `split` / `regexp_string`.
Column widths are accumulated with `List.fold_left` and a mutable `pads` array.
A generic `print` higher-order function takes a `word -> pad -> unit` callback,
allowing left, right, and center variants to share the iteration logic.

## Rust Approach

Rust uses iterator chains — `text.lines().map(|l| l.split('$').collect())` —
avoiding mutable state during parsing. Column widths are computed with `(0..n).map(...)`
which naturally handles ragged rows via `row.get(col)` returning `Option`. 
The format specifiers `{:<w$}`, `{:>w$}`, `{:^w$}` replace all manual padding
arithmetic, while an `Alignment` enum replaces the OCaml callback pattern.

## Key Differences

1. **String formatting:** OCaml manually constructs `String.make pad ' '` and concatenates;
   Rust's format strings (`{word:<w$}`) express alignment directly in the format spec.
2. **Higher-order abstraction:** OCaml uses a first-class function `print(fun word pad -> ...)`;
   Rust uses an `Alignment` enum + `match` for the same open/closed extensibility.
3. **Mutable state:** OCaml uses a mutable `pads` array updated with `Array.iteri`;
   Rust's functional column-width computation has no mutable state.
4. **Parsing:** OCaml uses `Str` regex library even for literal delimiter splits;
   Rust's `str::split(char)` is built-in and allocation-free for the split itself.
