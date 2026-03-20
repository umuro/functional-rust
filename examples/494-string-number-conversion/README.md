📖 **[View on hightechmind.io →](https://hightechmind.io/rust/494-string-number-conversion)**

---

# String-Number Conversion

Rust provides `.to_string()` (via `Display`), `format!("{:x}", n)` for radix formatting, `.parse::<T>()` (via `FromStr`) for parsing, and `i64::from_str_radix(s, base)` for non-decimal parsing — covering all directions of number-string conversion.

## Problem Statement

Every program at its boundaries converts between numbers and text: reading config values, displaying metrics, parsing user input, serialising protocols. These conversions must handle: invalid input gracefully (`Result` not panic), radix selection (hex, binary, octal), floating-point precision, and negative numbers. Rust's standard library handles all these cases with a consistent `Result`-based API, avoiding the silent failure modes of C's `atoi` or Python's implicit type coercions.

## Learning Outcomes

- Convert integers to strings with `.to_string()` and `format!("{}", n)`
- Format in hexadecimal with `format!("{:x}", n)` / `{:X}` / `{:b}` / `{:o}`
- Parse a string to a number with `.parse::<T>()` returning `Result`
- Parse a non-decimal number with `i64::from_str_radix(s, base)`
- Format floats with precision `format!("{:.N}", f)` and parse with `.parse::<f64>()`

## Rust Application

Integer to string:

```rust
42i32.to_string()    // "42"
(-7i32).to_string()  // "-7"
```

Radix formatting:

```rust
format!("{:x}", 255u32)    // "ff"
format!("{:X}", 255u32)    // "FF"
format!("{:b}", 8u32)      // "1000"
format!("{:o}", 8u32)      // "10"
```

Parsing:

```rust
"42".parse::<i32>().unwrap()          // 42
"abc".parse::<i32>().is_err()         // true
i64::from_str_radix("ff", 16).unwrap() // 255
"3.14".parse::<f64>().is_ok()         // true
```

## OCaml Approach

```ocaml
(* int to string *)
string_of_int 42         (* "42" *)
Printf.sprintf "%d" 42   (* "42" *)

(* hex *)
Printf.sprintf "%x" 255  (* "ff" *)

(* string to int *)
int_of_string "42"        (* 42 — raises Failure on error *)
int_of_string_opt "42"    (* Some 42 — OCaml 4.05+ *)
int_of_string "0xff"      (* 255 — handles 0x prefix *)

(* float *)
string_of_float 3.14      (* "3.14" *)
float_of_string "3.14"    (* 3.14 *)
```

## Key Differences

1. **Error handling**: Rust's `parse` returns `Result`, requiring explicit error handling; OCaml's `int_of_string` raises `Failure` (exception) — use `int_of_string_opt` for the `option`-based safe version.
2. **Radix parsing**: Rust's `from_str_radix(s, base)` takes an explicit base; OCaml's `int_of_string` handles `0x`/`0o`/`0b` prefixes automatically.
3. **Float formatting**: Rust's `format!("{:.2}", f)` is compile-time checked; OCaml's `Printf.sprintf "%.2f" f` is not.
4. **Generic `parse`**: Rust's `.parse::<T>()` is generic over any `T: FromStr`; OCaml requires type-specific functions (`int_of_string`, `float_of_string`).

## Exercises

1. **Radix converter**: Write `fn convert_base(s: &str, from: u32, to: u32) -> Result<String, _>` that parses `s` in base `from` and formats in base `to`.
2. **Human-readable bytes**: Write `fn format_bytes(n: u64) -> String` that formats a byte count as `"1.23 MB"`, `"456 KB"`, or `"789 B"` with appropriate precision.
3. **Bulk parse**: Write `fn parse_csv_ints(s: &str) -> Result<Vec<i64>, _>` that splits on commas, trims whitespace, and parses each field, returning the first error encountered.
