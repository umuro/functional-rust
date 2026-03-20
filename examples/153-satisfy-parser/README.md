📖 **[View on hightechmind.io →](https://hightechmind.io/rust/153-satisfy-parser)**

---

# Satisfy Parser

## Problem Statement

Rather than enumerate every specific character a parser might accept, `satisfy` generalizes character matching to any predicate `Fn(char) -> bool`. This single combinator replaces dozens of specific parsers: `is_digit`, `is_letter`, `is_whitespace`, `is_alphanumeric` all become one-liners built from `satisfy`. The predicate-based approach is more extensible, composable, and mirrors the mathematical notation for character classes used in formal grammar theory.

## Learning Outcomes

- Understand `satisfy` as the universal character matching primitive
- Learn how to build specific parsers (`digit`, `letter`, `alphanumeric`) from `satisfy`
- See how description strings in `satisfy` produce readable error messages
- Practice building a small parser vocabulary entirely from one primitive

## Rust Application

`satisfy<F: Fn(char) -> bool + 'a>(pred: F, desc: &str) -> Parser<'a, char>` captures the predicate as a closure and the description string for error messages. When the first character passes `pred`, it returns the character and advances the input. `is_digit()` is `satisfy(|c| c.is_ascii_digit(), "digit")`. Composing `satisfy` parsers with `many0` and `many1` gives complete lexers for numbers, identifiers, and whitespace.

## OCaml Approach

OCaml's `angstrom` provides `satisfy : (char -> bool) -> char t` directly. The idiomatic pattern:
```ocaml
let digit = satisfy Char.is_digit
let letter = satisfy Char.is_alpha
let alphanumeric = satisfy (fun c -> Char.is_alpha c || Char.is_digit c)
```
OCaml's lighter closure syntax (`Char.is_digit`) compared to Rust's (`|c| c.is_ascii_digit()`) makes these definitions more compact. Error messages in angstrom are produced separately via `<?> "description"`.

## Key Differences

1. **Description in signature**: Rust's `satisfy` takes `desc: &str` inline; OCaml's angstrom uses a `<?>` operator to attach descriptions separately.
2. **Predicate type**: Rust's `F: Fn(char) -> bool + 'a` captures the lifetime of the predicate; OCaml's predicates are plain function values managed by the GC.
3. **Unicode awareness**: Rust's `char` is always a Unicode scalar value; OCaml's `char` is a byte (0..255), requiring `Uchar` for full Unicode.
4. **Composability**: Both `satisfy` variants compose identically with `many0`, `many1`, `map`, and `choice`; the higher-level combinators are the same.

## Exercises

1. Build `hex_digit() -> Parser<char>` using `satisfy` and the predicate `|c| c.is_ascii_hexdigit()`.
2. Write `printable_char() -> Parser<char>` that accepts any non-control, non-whitespace character.
3. Implement `not_char(c: char) -> Parser<char>` that accepts any character except `c`, built from `satisfy`.
