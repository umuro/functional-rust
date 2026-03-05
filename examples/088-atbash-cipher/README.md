# Example 088: Atbash Cipher

**Difficulty:** ⭐⭐
**Category:** String Processing
**OCaml Source:** Classic cipher exercise

## Problem Statement

Encode a string using the Atbash cipher: each letter `a`–`z` maps to its mirror (`a↔z`, `b↔y`, …). Digits pass through unchanged; all other characters are dropped. The result is grouped into 5-character chunks separated by spaces. Decoding is the same operation since Atbash is its own inverse.

## Learning Outcomes

- Using `slice::chunks` for fixed-width grouping without manual index arithmetic
- Building a `filter_map` pipeline to simultaneously filter and transform characters
- Recognizing when a cipher is self-inverse and sharing encode/decode logic
- Translating OCaml's `Seq.filter_map` + manual `List.filteri` grouping into Rust's `chunks`

## OCaml Approach

OCaml converts the string to a `Seq`, applies `filter_map` to drop non-alphanumeric characters and apply the transposition, then uses recursive `List.filteri` calls to split the list into 5-element groups before joining with `String.concat`. The decode path reuses the same character map since the cipher is symmetric.

## Rust Approach

Rust collects the mapped characters into a `Vec<char>`, then calls `.chunks(5)` to slice it into fixed-width windows. Each chunk is collected into a `String` and the chunks are joined with `" "`. The recursive variant mirrors the OCaml grouping logic using `split_at`, making the structural parallel explicit.

## Key Differences

1. **Grouping:** OCaml uses `List.filteri` with index predicates to extract each chunk; Rust uses `slice::chunks` — the idiomatic, zero-cost slice primitive.
2. **Character mapping:** OCaml uses `Char.code`/`Char.chr` arithmetic on `char`; Rust uses the same arithmetic on `u8` bytes (`b'z' - (c as u8 - b'a')`), then casts back to `char`.
3. **Filtering:** OCaml uses `Seq.filter_map`; Rust uses `Iterator::filter_map` — essentially the same abstraction with identical semantics.
4. **Self-inverse property:** Both languages share one mapping function for encode and decode; Rust's `decode` adds a whitespace filter to ignore grouping spaces before re-applying the same transform.
