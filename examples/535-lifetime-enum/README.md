📖 **[View on hightechmind.io →](https://hightechmind.io/rust/535-lifetime-enum)**

---

# Lifetimes in Enums
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Enums can hold references just like structs, and the same lifetime annotation rules apply. Token types in parsers, parse result types, and zero-copy JSON/YAML values are all classic examples: they borrow slices of the input string rather than copying, making parsing dramatically faster. The `Token<'a>` pattern is foundational in parser combinators (nom, winnow, pest) — a lexer tokenizes a string slice and yields tokens that are lightweight views into the original input, requiring no allocation.

## Learning Outcomes

- How enum variants with reference fields require a lifetime parameter on the enum
- How `Token<'a>` with `Word(&'a str)` variants enables zero-copy tokenization
- How `ParseResult<'a, T>` models a remaining-input alongside a parsed value
- How `JsonValue<'a>` builds a zero-copy JSON tree borrowing strings from the source
- Where lifetime-annotated enums appear: nom tokens, serde zero-copy deserialization, ASTs

## Rust Application

`Token<'a>` has variants `Word(&'a str)`, `Number(i64)`, `Punctuation(char)`, and `End`. Only `Word` borrows from the input; the others are owned values. `ParseResult<'a, T>` carries either `Ok(T, &'a str)` (value plus remaining input) or `Err(&'a str, String)` (failure site plus message). `JsonValue<'a>` is a recursive enum with `String(&'a str)`, `Array(Vec<JsonValue<'a>>)`, and `Object(Vec<(&'a str, JsonValue<'a>)>)` — all string values borrow directly from the source JSON bytes.

Key patterns:
- `enum Token<'a> { Word(&'a str), ... }` — only borrowing variants need the lifetime
- `ParseResult<'a, T>` — threading the remaining input through parser combinator results
- Recursive `JsonValue<'a>` — lifetime propagates through `Vec` elements

## OCaml Approach

OCaml variant types for tokens use `string` (owned) since strings are immutable and GC-managed. Zero-copy parsing requires explicit `Bigarray` or `Bytes` slices:

```ocaml
type token = Word of string | Number of int | Punct of char | End
type 'a parse_result = Ok of 'a * string | Err of string * string
```

All `string` values in OCaml are GC-managed, so there is no dangling reference concern — but they are copied by default.

## Key Differences

1. **Zero-copy tokens**: Rust `Word(&'a str)` is a zero-copy view into the input; OCaml `Word of string` copies the substring unless explicit slice types are used.
2. **Enum lifetime propagation**: Rust propagates `'a` automatically through recursive enum arms like `Array(Vec<JsonValue<'a>>)`; OCaml records and variants need no lifetime propagation.
3. **Mixed owned/borrowed variants**: Rust enums can mix owned variants (`Number(i64)`) and borrowed variants (`Word(&'a str)`) in the same type; OCaml has no such distinction — all values are uniformly GC-managed.
4. **Parser result threading**: Rust `ParseResult<'a, T>` threads the remaining `&'a str` through the type system; OCaml parser combinators return `(value, rest: string)` tuples with no lifetime tracking.

## Exercises

1. **Full tokenizer**: Extend `tokenize` to handle multi-digit numbers and punctuation sequences, returning a `Vec<Token<'_>>` that borrows entirely from the input string.
2. **Parser combinator**: Implement `fn parse_word<'a>(input: &'a str) -> ParseResult<'a, &'a str>` that consumes and returns the first whitespace-delimited word.
3. **Zero-copy JSON strings**: Write a function that takes a JSON-like string and returns a `Vec<(&str, &str)>` of key-value pairs where both key and value are slices of the original input.
