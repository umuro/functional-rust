# Example 1185: String.split_on_char — Tokenize a String
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Split a string on a delimiter character to produce a list of tokens, then filter out empty tokens that arise from consecutive delimiters or leading and trailing whitespace. This operation is fundamental to parsing CSV lines, tokenizing user input, and processing any character-delimited data format. The example covers the primary string splitting primitive in each language and shows how filtering empty strings integrates naturally into the functional pipeline.

## Learning Outcomes

- How OCaml's `String.split_on_char ',' s` maps to Rust's `s.split(',')` — both split on a single character delimiter and preserve empty strings between consecutive delimiters
- How OCaml's `List.filter (fun s -> s <> "") tokens` maps to Rust's `.filter(|s| !s.is_empty())` — the same concept expressed as a list operation vs. an iterator adapter
- Why Rust's `split` returns a lazy `Split` iterator rather than an allocated `Vec`, and how to materialize it with `.collect::<Vec<_>>()` when a concrete collection is needed
- The difference between `str::split(char)` for single-character delimiters and `str::split_whitespace()` for collapsing any run of whitespace — both useful, with different semantics
- How `List.iteri` maps to `.iter().enumerate()` in Rust for index-aware iteration, a pattern that appears repeatedly when processing tokenized data

## OCaml Approach

OCaml's `String.split_on_char : char -> string -> string list` (added in OCaml 4.04) takes the delimiter character first and the string second, returning a `string list`. It preserves empty strings between consecutive delimiters, so `String.split_on_char ',' "a,,b"` returns `["a"; ""; "b"]`. Empty tokens from surrounding whitespace are removed with `List.filter (fun s -> s <> "") tokens`. Index-aware printing uses `List.iteri (fun i f -> ...) fields`, which passes the zero-based index alongside each element. All operations produce new values; no mutation occurs.

## Rust Application

Rust's `str::split(pattern)` returns a lazy `Split<'_, char>` iterator over string slices (`&str`) that borrow from the original string — no heap allocation occurs until `.collect()` is called. Like the OCaml version, it preserves empty slices between consecutive delimiters. Filtering empty strings chains directly: `s.split(',').filter(|s| !s.is_empty()).collect::<Vec<_>>()`. For whitespace splitting with automatic empty-token removal, `str::split_whitespace()` handles any run of whitespace characters in one step. Index-aware iteration uses `.iter().enumerate()` on the collected `Vec`, or `.enumerate()` directly on the iterator chain before collection. Because the slices borrow from the original `&str`, no copying occurs during the split itself.

## Key Differences

1. **Eager vs. lazy evaluation:** OCaml's `String.split_on_char` immediately allocates and returns a `string list`; Rust's `str::split` returns a lazy iterator — allocation happens only when `.collect()` is called, and if you only need to iterate, you can avoid allocation entirely.
2. **Argument order:** OCaml: `String.split_on_char delimiter string` (delimiter first, string second — pipe-friendly); Rust: `string.split(delimiter)` (method on the string, delimiter as argument) — the string is the receiver in Rust's method call syntax.
3. **Empty token handling:** Both languages preserve empty strings between consecutive delimiters by default. OCaml removes them with `List.filter (fun s -> s <> "")`; Rust uses `.filter(|s| !s.is_empty())` as an iterator adapter, or uses the separate `split_whitespace()` method which collapses runs automatically.
4. **Ownership and borrowing:** OCaml returns owned `string` values in the list; Rust's `split` returns `&str` slices that borrow from the original string, which is more memory-efficient but means the resulting slices cannot outlive the source string without cloning.

## Exercises

1. Implement `parse_csv_record(line: &str) -> Vec<&str>` that splits on commas and trims leading and trailing whitespace from each field using `.map(str::trim)` in the iterator chain. Handle quoted fields containing commas as a stretch goal.
2. Implement `word_count(text: &str) -> std::collections::HashMap<&str, usize>` that splits on whitespace, filters empty tokens, and counts the occurrences of each word using `HashMap::entry(...).and_modify(...).or_insert(1)`.
3. Implement `split_first(s: &str, delim: char) -> Option<(&str, &str)>` that splits on the first occurrence of `delim` and returns `Some((before, after))`, or `None` if the delimiter is not present. Use `str::splitn(2, delim)` and pattern-match on the resulting iterator to extract both parts.
