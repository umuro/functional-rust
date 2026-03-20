📖 **[View on hightechmind.io →](https://hightechmind.io/rust/154-string-parser)**

---

# String Parser

## Problem Statement

Many grammars require matching fixed string literals: keywords like `"true"` and `"false"`, operators like `"+="`, or delimiters like `"<!--"`. Matching character by character would work but is verbose and error-prone for multi-character strings. The `tag` combinator matches an entire expected string at once, using `str::starts_with` for efficiency. Case-insensitive variants handle HTTP methods, SQL keywords, and HTML tag names where casing is not significant.

## Learning Outcomes

- Implement `tag` (exact string match) and `tag_no_case` (case-insensitive match)
- Understand why string parsers return `&'a str` (a slice of the input) rather than `String` (an allocation)
- See how string parsers combine with other combinators to parse complex patterns
- Learn the lifetime relationships between the parser, input, and returned slice

## Rust Application

`tag(expected: &str) -> Parser<'a, &'a str>` checks `input.starts_with(expected)`. On match, it returns a slice `&input[..expected.len()]` — a zero-copy reference into the original input. Advancing the input is `&input[expected.len()..]`. The lifetime `'a` ties the returned slice to the input — the returned `&str` is valid as long as the original input string is alive. `tag_no_case` lowercases both strings before comparison, handling SQL keywords like `SELECT`/`select`.

## OCaml Approach

OCaml's angstrom provides `string : string -> string t` and `string_ci : string -> string t`. Because OCaml strings are immutable byte sequences, `string_ci` may allocate for the lowercased comparison. The returned value is a newly allocated string, not a slice of the input — allocation is less of a concern under GC.

## Key Differences

1. **Zero-copy returns**: Rust's `tag` returns a `&'a str` slice — no allocation; OCaml typically returns a new string (allocation under GC).
2. **Lifetime tracking**: Rust's `'a` lifetime annotation enforces the validity relationship between input and returned slice; OCaml has no lifetime concept.
3. **Case folding**: `tag_no_case` in Rust must be careful with Unicode case folding (`.to_lowercase()` allocates); OCaml's `String.lowercase_ascii` handles only ASCII.
4. **Error messages**: Both should include the expected string in the error; convention differs between libraries.

## Exercises

1. Implement `tag_byte_slice(expected: &[u8]) -> Parser<&[u8]>` that works on raw bytes instead of UTF-8 strings.
2. Write a `bool_parser() -> Parser<bool>` using `tag("true")` and `tag("false")` combined with `choice`.
3. Implement `keyword(kw: &str) -> Parser<&str>` that matches a string followed by a non-identifier character (word boundary).
