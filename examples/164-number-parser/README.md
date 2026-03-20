📖 **[View on hightechmind.io →](https://hightechmind.io/rust/164-number-parser)**

---

# Number Parser

## Problem Statement

Floating-point numbers in text formats (JSON, CSV, scientific data) require parsing optional sign, integer digits, optional decimal point and fractional digits, and optional exponent notation (`1.5e-10`). Each component is optional or required in a specific combination. This example builds a full floating-point parser using combinators, demonstrating how complex lexical rules reduce to composed simple rules with clear, testable components.

## Learning Outcomes

- Build a complete floating-point parser with sign, integral, fractional, and exponent parts
- Learn how `opt` and `many1` combine to handle optional and required components
- Understand the string-then-convert pattern: collect the number string, then call `str::parse`
- See how combinator parsers map directly to BNF grammar rules

## Rust Application

The grammar: `number := sign? digit+ ('.' digit+)? (('e'|'E') sign? digit+)?`. The parser collects all characters into a `String` buffer, then calls `.parse::<f64>()`. An imperative scanner in `parse_float` scans byte-by-byte for performance, avoiding the `Vec<char>` + join overhead of combinator-based collection. The signed integer version uses `opt(char_parser('-'))` + `many1(satisfy(is_digit))` + `map` to assemble the pieces.

## OCaml Approach

Angstrom provides a direct approach:
```ocaml
let number =
  take_while1 (fun c -> Char.is_digit c || c = '.' || c = 'e' || c = 'E'
                        || c = '+' || c = '-')
  >>| float_of_string
```
This is a common shortcut, though it accepts invalid strings like `"1.2.3"` that `float_of_string` rejects with an exception. A stricter combinator parser follows the BNF more closely.

## Key Differences

1. **Precision shortcut**: OCaml's `take_while1` + `float_of_string` is concise but permissive; Rust's combinator parser is strict but verbose.
2. **Exception vs. Result**: OCaml's `float_of_string` raises `Failure` on invalid input; Rust's `str::parse::<f64>()` returns `Result`, propagated via `?`.
3. **Buffer efficiency**: OCaml's `take_while1` works directly on the buffer; Rust's combinator version collects `Vec<char>` before converting.
4. **Locale**: Both use the C locale for decimal parsing (`.` as decimal separator); locale-aware parsing requires additional handling.

## Exercises

1. Add exponent parsing: `"1.5e-10"`, `"2.0E+3"` should parse correctly.
2. Implement a strict JSON number parser that rejects leading zeros (`"01"` is invalid in JSON).
3. Write a parser for rational numbers in the form `"3/4"` → `(3, 4)` as a pair of integers.
