# 494: Number ↔ String Conversion

**Difficulty:** 1  **Level:** Beginner

Convert between numbers and strings in every direction — including hex, binary, and scientific notation.

## The Problem This Solves

Number-to-string and string-to-number conversions are so common they're practically boilerplate. Python has `str(42)`, `int("42")`, `float("3.14")`, `hex(255)`. JavaScript has `String(42)`, `parseInt("42")`, `parseFloat("3.14")`, `(255).toString(16)`. Both let you do radix conversions.

Rust covers all of this without external crates. `42.to_string()` and `format!("{}", 42)` convert to string. `.parse::<i32>()` converts back. Radix formatting is built into `format!` — `{:x}` for hex, `{:b}` for binary, `{:o}` for octal. Parsing from non-decimal bases uses `i64::from_str_radix("ff", 16)`.

The key advantage over Python/JavaScript: all parse operations return `Result`, not runtime exceptions. `"abc".parse::<i32>()` returns `Err(...)` — not a crash. You handle it with `match`, `?`, or `.unwrap_or(0)`.

## The Intuition

Number → string: use `n.to_string()` for the simple case, or `format!("{:format_spec}", n)` for formatted output. The `format!` approach gives you padding, alignment, precision, and radix control.

String → number: use `.parse::<T>()` for standard decimal parsing. For other bases, use `T::from_str_radix(s, base)`.

Float formatting: `{:.2}` gives 2 decimal places. `{:e}` gives scientific notation. `{:.0}` rounds to nearest integer in display (doesn't change the actual value).

The chain pattern — `"42".parse::<i32>().map(|n| n * 2)` — is idiomatic for "parse, transform, use" workflows.

## How It Works in Rust

```rust
// Int → String
42i32.to_string()          // "42"
format!("{:08}", 42i32)    // "00000042" — zero-padded
format!("{:+}", 42i32)     // "+42" — explicit sign
format!("{:x}", 255u32)    // "ff"
format!("{:X}", 255u32)    // "FF"
format!("{:b}", 42u32)     // "101010"
format!("{:o}", 8u32)      // "10"

// Float → String
format!("{:.2}", 3.14159)  // "3.14"
format!("{:e}", 1_234_567.89f64)  // "1.234568e6"
format!("{:.0}", 3.7f64)   // "4" — rounded for display

// String → Int (Result, not panic)
"42".parse::<i32>()        // Ok(42)
"-7".parse::<i32>()        // Ok(-7)
"abc".parse::<i32>()       // Err(ParseIntError)

// String → Int from other bases
i64::from_str_radix("ff", 16)   // Ok(255)
i64::from_str_radix("101", 2)   // Ok(5)
i64::from_str_radix("17", 8)    // Ok(15)

// String → Float
"3.14".parse::<f64>()      // Ok(3.14)
"inf".parse::<f64>()       // Ok(f64::INFINITY)
"1e5".parse::<f64>()       // Ok(100000.0)

// Chain: parse → transform → to string
let result = "42"
    .parse::<i32>()
    .map(|n| (n * 2).to_string())
    .unwrap_or_else(|_| "invalid".to_string());
// "84"

// Handle parse error gracefully
let n: i32 = "123abc".parse().unwrap_or(0);  // 0 on failure
```

## What This Unlocks

- **Config parsing** — read port numbers, timeouts, flags from string config values with type safety.
- **Report formatting** — zero-padded IDs (`{:06}`), percentages (`{:.1}%`), currency (`{:.2}`).
- **Binary/hex tools** — convert between decimal, hex, binary with format specifiers and `from_str_radix`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Int to string | `string_of_int 42` | `42.to_string()` or `format!("{}", 42)` |
| Float to string | `string_of_float 3.14` | `3.14.to_string()` or `format!("{:.2}", 3.14)` |
| String to int | `int_of_string "42"` (raises on error) | `"42".parse::<i32>()` → `Result` |
| Safe string to int | `int_of_string_opt "42"` → `option` | `"42".parse::<i32>()` → `Result` |
| Hex output | `Printf.printf "%x" n` | `format!("{:x}", n)` |
| Parse from hex | `int_of_string "0xff"` | `i64::from_str_radix("ff", 16)` |
| Float precision | `Printf.printf "%.2f" f` | `format!("{:.2}", f)` |
