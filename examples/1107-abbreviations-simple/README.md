# Example 1107: Abbreviations, Simple

**Difficulty:** ⭐⭐
**Category:** General / String Processing
**OCaml Source:** Rosetta Code — Abbreviations, simple

## Problem Statement

Given a command table where each entry is a name and an optional minimum abbreviation length,
determine whether a user-supplied word is a valid abbreviation of any command. Commands without
an explicit minimum require an exact match.

## Learning Outcomes

- Parsing structured text into typed data with iterators and `parse::<usize>()`
- Using `.find()` on an iterator of tuples with a multi-condition predicate
- Case-insensitive prefix matching with `.to_lowercase()` and `.starts_with()`
- The difference between "explicit minimum" (numbered) vs "exact only" (unnumbered) table entries

## OCaml Approach

OCaml defines the table as a string literal and then processes it imperatively using `String.split`
and `List.find_opt`. The lookup compares uppercased prefixes against uppercased command names, using
the numeric suffix as a minimum-length gate.

## Rust Approach

Rust parses the table once into a `Vec<(String, usize)>` of `(lowercase_command, min_length)` pairs,
then uses a single `.find()` call on that slice. The predicate checks both the length threshold and
the prefix condition. Collecting results over words uses `.map().collect()` on a split iterator.

## Key Differences

1. **Table representation:** OCaml keeps the raw string and re-parses on each lookup; Rust parses once into a typed structure.
2. **Optionality:** OCaml uses `match List.find_opt ... with Some/None`; Rust uses `.find(...).map(...).unwrap_or_else(...)`.
3. **Case handling:** Both lowercase the input for comparison; Rust then uppercases only the matched command name for output.
4. **No number = exact match:** In both languages, absence of a numeric suffix means the minimum equals the full command length.
