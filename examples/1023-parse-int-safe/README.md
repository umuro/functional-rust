📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1023-parse-int-safe)**

---

# 1023-parse-int-safe — Safe Integer Parsing

## Problem Statement

Parsing integers from strings is the entry point for untrusted data in almost every application: reading configuration files, processing HTTP query parameters, deserializing CSV rows. C's `atoi` silently returns 0 on failure. C++'s `std::stoi` throws an exception. Python's `int()` raises a `ValueError`. Rust's `str::parse::<i64>()` returns `Result<i64, ParseIntError>`, forcing the caller to handle the failure case.

This example explores all the variants: raw parsing, custom error messages, range validation, and default fallbacks — covering the full spectrum of real-world needs.

## Learning Outcomes

- Use `str::parse::<i64>()` and handle `ParseIntError`
- Add descriptive error context with `map_err`
- Chain parsing with domain validation (positive, in-range)
- Use `unwrap_or` and `unwrap_or_else` for safe defaults
- Know the difference between `ParseIntError` kinds (empty, invalid digit, overflow)

## Rust Application

`src/lib.rs` provides four escalating functions. `parse_int` delegates directly to the standard library. `parse_int_msg` adds a human-readable error with `map_err`. `parse_positive` chains `parse` with a positivity check using `?`. `parse_in_range` adds both lower and upper bound checks. `parse_or_default` collapses the result to a default value using `.unwrap_or`.

`ParseIntError` has a `kind()` method that returns `IntErrorKind::Empty`, `InvalidDigit`, `PosOverflow`, or `NegOverflow`, enabling precise error handling.

## OCaml Approach

OCaml's `int_of_string_opt` returns `option int`:

```ocaml
let parse_int s =
  match int_of_string_opt s with
  | None -> Error (Printf.sprintf "cannot parse '%s' as integer" s)
  | Some n -> Ok n

let parse_positive s =
  let* n = parse_int s in
  if n > 0 then Ok n
  else Error (Printf.sprintf "not positive: %d" n)
```

The `int_of_string` function (without `_opt`) raises `Failure` on invalid input, which is the exception-based alternative.

## Key Differences

1. **Error type richness**: Rust's `ParseIntError` has a `kind()` method for precise failure categorisation; OCaml returns `None` with no error detail.
2. **Overflow detection**: Rust detects overflow as a distinct `ParseIntError::PosOverflow` variant; OCaml's `int_of_string` raises on overflow.
3. **Composition with `?`**: Rust composes parse + validation with `?` in a linear style; OCaml uses `let*` or manual `match`.
4. **Default handling**: Rust's `unwrap_or_else` is a method on `Result`; OCaml uses `Option.value ~default:` for the `option` equivalent.

## Exercises

1. Write a `parse_hex(s: &str) -> Result<i64, String>` function that parses a hexadecimal string like `"0xFF"` or `"ff"`.
2. Implement `parse_list(s: &str) -> Result<Vec<i64>, String>` that parses a comma-separated string of integers and collects errors.
3. Write a function that attempts to parse a string as `i64`, then `f64`, then `bool`, returning the first successful parse as a boxed value.
