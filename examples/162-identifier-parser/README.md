📖 **[View on hightechmind.io →](https://hightechmind.io/rust/162-identifier-parser)**

---

# Identifier Parser
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Identifiers appear in every programming language, configuration format, and data schema: variable names, function names, JSON keys, INI section names. An identifier is typically a letter or underscore followed by any number of letters, digits, or underscores. Parsing identifiers correctly — distinguishing them from keywords, handling Unicode identifiers, producing owned `String` output — is a foundational skill for building any language-level parser.

## Learning Outcomes

- Build a complete identifier parser: leading letter/underscore, followed by letters/digits/underscores
- Understand why identifiers start with a letter (not digit) in most languages
- Learn how to produce owned `String` output from a parser returning `Vec<char>`
- See how identifier parsing combines with keyword exclusion for language lexers

## Rust Application

`identifier() -> Parser<String>` combines: `satisfy(|c| c.is_ascii_alphabetic() || c == '_', "letter or underscore")` for the first character, then `many0(satisfy(|c| c.is_alphanumeric() || c == '_', "alphanumeric or underscore"))` for the rest. The first and rest are joined with `pair` and `map`. The result is collected into a `String` via `iter().collect()`. Keyword exclusion (`"if"`, `"let"`) can be added by checking the result against a keyword set.

## OCaml Approach

In angstrom:
```ocaml
let ident_start = satisfy (fun c -> Char.is_alpha c || c = '_')
let ident_rest = take_while (fun c -> Char.is_alphanum c || c = '_')
let identifier = lift2 (fun c s -> String.make 1 c ^ s) ident_start ident_rest
```
OCaml's `take_while` is more efficient than `many0(satisfy(...))` because it works on the raw buffer without constructing intermediate character values.

## Key Differences

1. **Efficiency**: OCaml's `take_while` scans bytes directly; Rust's `many0(satisfy(...))` decodes each UTF-8 character individually — more correct for Unicode but slower for ASCII.
2. **Owned output**: Both produce owned strings (`String` in Rust, `string` in OCaml) — identifiers are typically stored and compared, not just sliced.
3. **Unicode identifiers**: Rust's `is_ascii_alphabetic()` restricts to ASCII; `is_alphabetic()` allows Unicode letters; OCaml's `Uchar` handles Unicode similarly.
4. **Keyword exclusion**: Both add keyword exclusion as a post-map check; some parsers define keywords as distinct parser rules tried before identifiers.

## Exercises

1. Add keyword exclusion: the identifier parser should fail if the result is in `["if", "else", "while", "let", "fn"]`.
2. Implement `scoped_identifier() -> Parser<Vec<String>>` that parses `"std::collections::HashMap"` as `["std", "collections", "HashMap"]`.
3. Write a test verifying that `"_private"`, `"CamelCase"`, `"snake_case_123"` all parse successfully.
