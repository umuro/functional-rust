📖 **[View on hightechmind.io →](https://hightechmind.io/rust/164-number-parser)**

---

# 164: Number Parser

**Difficulty:** 3  **Level:** Advanced

Parse integers, floats, and scientific notation — the token every expression parser needs first.

## The Problem This Solves

Numbers show up everywhere: config files, JSON, CSV, math expressions. Parsing them correctly is surprisingly tricky. `-3.14` has an optional sign. `1.5e-3` has a decimal and an exponent. `.5` starts with a dot, not a digit. Any number parser you write will encounter all three.

The standard library's `str::parse::<f64>()` can parse the final string — but it can't tell you *where* the number ends in a larger input. If your input is `"3.14 + rest"`, you need to scan forward to find the number boundary first, then parse it.

This example shows both an imperative scanning approach (fast, zero-copy) and a combinator approach (composable). The key insight: scan forward at the byte level, then hand the slice to the stdlib — best of both worlds.

## The Intuition

Walk the input character by character, collecting bytes that could be part of a number: optional sign, digits, optional dot + more digits, optional `e`/`E` + sign + digits. When you hit something that can't be part of a number, stop. Hand the collected slice to `str::parse::<f64>()`.

```
input: "-3.14rest"
scan:   ^ optional sign
        ^ ^ ^ ^ digits and dot
             ^ stop — 'r' is not numeric
slice: "-3.14" → parse → -3.14f64
remaining: "rest"
```

## How It Works in Rust

```rust
fn parse_number(input: &str) -> ParseResult<f64> {
    let bytes = input.as_bytes();
    let mut pos = 0;

    // Optional sign
    if pos < bytes.len() && (bytes[pos] == b'+' || bytes[pos] == b'-') {
        pos += 1;
    }

    // Integer digits (or leading dot like ".5")
    let digits_start = pos;
    while pos < bytes.len() && bytes[pos].is_ascii_digit() {
        pos += 1;
    }

    // Decimal part
    if pos < bytes.len() && bytes[pos] == b'.' {
        pos += 1;
        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
            pos += 1;
        }
    }

    // Must have consumed at least one digit
    if pos == digits_start {
        return Err("expected number".to_string());
    }

    // Exponent part: e-3, E+10, etc.
    if pos < bytes.len() && (bytes[pos] == b'e' || bytes[pos] == b'E') {
        pos += 1;
        if pos < bytes.len() && (bytes[pos] == b'+' || bytes[pos] == b'-') {
            pos += 1;
        }
        while pos < bytes.len() && bytes[pos].is_ascii_digit() {
            pos += 1;
        }
    }

    // Zero-copy: slice into original string, no allocation
    let num_str = &input[..pos];
    let value = num_str.parse::<f64>()
        .map_err(|e| format!("invalid number: {}", e))?;

    Ok((value, &input[pos..]))
}
```

## What This Unlocks

- **Expression parsers** — every arithmetic parser needs to parse number literals.
- **Data formats** — JSON numbers, CSV columns, INI values — all need this.
- **Zero-copy parsing** — the `&str` slice approach avoids heap allocation for the number string.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Float conversion | `float_of_string` | `str::parse::<f64>()` |
| Byte access | `Bytes.get s i` or `s.[i]` | `s.as_bytes()[i]` |
| String buffer | `Buffer.t` (dynamic allocation) | `&str` slice of original input (zero-copy) |
| Scientific notation | Manual scan | Manual scan — both languages do the same work |
