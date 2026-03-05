# Example 986: String.sub and String.concat — Substring and Join

**Difficulty:** ⭐
**Category:** stdlib-string
**OCaml Source:** OCaml standard library — `String.sub`, `String.concat`

## Problem Statement

Extract a contiguous substring from a string given a start position and length,
and join a list of strings together with a separator between each element.

## Learning Outcomes

- How Rust's `&str` slicing (`s.get(pos..pos+len)`) maps to OCaml's `String.sub`
- Why Rust returns `Option<&str>` instead of raising an exception on bad ranges
- How `.join(sep)` on `&[&str]` replaces OCaml's `String.concat sep list`
- The difference between byte-index slicing and char-index iteration for Unicode safety

## OCaml Approach

OCaml's `String.sub s pos len` extracts `len` bytes starting at `pos`, raising
`Invalid_argument` if the range is out of bounds. `String.concat sep parts`
iterates a `string list`, inserting `sep` between every adjacent pair — equivalent
to a fold with string append.

## Rust Approach

Rust strings are UTF-8 byte sequences. `&s[pos..pos+len]` panics on invalid
byte ranges, so the safe alternative is `s.get(pos..pos+len)` which returns
`Option<&str>` — `None` if the range is out of bounds or splits a multibyte
character. For char-level indexing (matching OCaml's character semantics),
iterating with `.chars().skip(pos).take(len).collect()` is safe for all Unicode.
Joining uses the built-in `slice::join` method, while a manual fold with
`String::push_str` makes the sequential accumulation explicit.

## Key Differences

1. **Error handling:** OCaml raises `Invalid_argument`; Rust's `get` returns `Option<&str>` — no exceptions
2. **Byte vs char indexing:** OCaml `String.sub` works on bytes (same as ASCII chars); Rust byte slices panic on char boundaries, char iteration is Unicode-safe
3. **Join built-in:** OCaml `String.concat` takes `sep` first then the list; Rust `.join(sep)` is called on the slice — argument order is reversed
4. **Zero allocation for substring:** Rust `&str` slices borrow without allocating; OCaml `String.sub` always allocates a new string
