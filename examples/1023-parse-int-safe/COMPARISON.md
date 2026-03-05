# Safe Integer Parsing — Comparison

## Core Insight
Both languages evolved from exception-based parsing to Result/Option-based. The safe versions are now idiomatic in both.

## OCaml Approach
- `int_of_string` raises `Failure` — old style, avoid
- `int_of_string_opt` returns `option` — safe, preferred
- No built-in range validation — wrap manually

## Rust Approach
- `str::parse::<i64>()` returns `Result<i64, ParseIntError>`
- `ParseIntError` has descriptive messages
- Chain with `.map_err()` for custom errors
- `unwrap_or(default)` for quick defaults

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Safe parse | `int_of_string_opt` | `str::parse::<i64>()` |
| Unsafe parse | `int_of_string` (exception) | No equivalent (always safe) |
| Error type | `None` (option) | `ParseIntError` (descriptive) |
| Default value | `Option.value ~default` | `.unwrap_or(default)` |
| Whitespace | Trimmed automatically | NOT trimmed — explicit `.trim()` |
| Overflow | Platform-dependent | Returns `Err` |
