📖 **[View on hightechmind.io →](https://hightechmind.io/rust/161-digit-parser)**

---

# Digit Parser
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Numbers are ubiquitous in data formats — configuration files, JSON, CSV, protocol messages. Parsing integers and floats correctly requires handling signs, leading zeros (allowed for floats, disallowed in JSON for integers), and overflow. Building a number parser from primitives demonstrates the full combinator pipeline: match sign, match digits, collect and convert, handle errors. This is the most universally used parser in real-world applications.

## Learning Outcomes

- Build a complete integer parser from digit primitives: sign → digits → conversion
- Handle edge cases: empty input, sign with no digits, overflow
- See how `many1` + `map` + `flat_map` combine for number parsing
- Understand the difference between parsing (string recognition) and interpretation (numeric value)

## Rust Application

`single_digit() -> Parser<char>` uses `satisfy(|c| c.is_ascii_digit())`. `unsigned_int() -> Parser<u64>` applies `many1(single_digit())`, collects the chars into a string, and parses with `str::parse`. `signed_int() -> Parser<i64>` prefixes with `opt(char_parser('-'))` and combines sign + magnitude. Overflow is handled by propagating `parse::<i64>().map_err(|e| e.to_string())` as a parser error.

## OCaml Approach

OCaml's standard library provides `int_of_string` and `float_of_string`. In angstrom:
```ocaml
let digit = satisfy (fun c -> c >= '0' && c <= '9')
let uint = many1 digit >>| (fun cs -> int_of_string (String.init (List.length cs) (List.nth cs)))
```
OCaml's arbitrary-precision integers (`Zarith`) handle overflow naturally where Rust must explicitly check bounds.

## Key Differences

1. **Overflow handling**: Rust's `i64::from_str` returns `Err` on overflow; OCaml's `int_of_string` raises `Failure` (exception); `Zarith` in OCaml never overflows.
2. **Digit range**: Rust's `is_ascii_digit()` handles ASCII `0-9`; OCaml's `c >= '0' && c <= '9'` is equivalent; Unicode digit handling requires additional work in both.
3. **Intermediate representation**: Rust collects `Vec<char>`, joins to `String`, then parses — three steps; OCaml similarly needs `String.init` or `Buffer.t` for the intermediate.
4. **Float parsing**: Both delegate to system-level float parsers; the main challenge is recognizing the float format (sign, integer part, fraction, exponent) with combinators.

## Exercises

1. Add parsing for hexadecimal integers: `"0x1F"` → `31`.
2. Implement `bounded_int<const MIN: i64, const MAX: i64>() -> Parser<i64>` that fails if the parsed value is out of range.
3. Write a binary number parser: `"0b1010"` → `10`.
