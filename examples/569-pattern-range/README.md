📖 **[View on hightechmind.io →](https://hightechmind.io/rust/569-pattern-range)**

---

# Range Patterns

## Problem Statement

Many real-world conditions are naturally expressed as ranges: grade thresholds, age categories, ASCII character classification, HTTP status code groups. Without range patterns, these require chains of `if/else if` with repeated comparisons. Range patterns (`lo..=hi`) in match arms express these conditions declaratively, with the added benefit of exhaustiveness checking across the covered domain. Range patterns are used in compilers, ASCII processors, game scoring systems, and any domain with numeric thresholds.

## Learning Outcomes

- How `90..=100 => 'A'` matches any value in the inclusive range
- How range patterns work for both integer and character values
- How ranges compose with or-patterns, guards, and bindings
- How the compiler checks for overlapping or missing ranges
- Where range patterns are more readable than equivalent if-else chains

## Rust Application

`grade(score: u32)` uses `90..=100`, `80..=89`, etc. — the compiler verifies the `_` fallthrough covers the remaining values. `char_type(c: char)` uses `'a'..='z'`, `'A'..='Z'`, `'0'..='9'` — character range patterns. `categorize(n: i32)` uses `i32::MIN..=-1` for all negative values. The `..=` operator means inclusive on both ends. Exclusive ranges (`..`) are not yet stable in patterns.

Key patterns:
- `lo..=hi => expr` — inclusive range in match arm
- `'a'..='z'` — character range pattern
- `i32::MIN..=0` — range to a sentinel value
- Combining: `0..=9 | 11..=19` — or with ranges

## OCaml Approach

OCaml does not have range patterns directly — numeric ranges use guards:

```ocaml
let grade score = match score with
  | s when s >= 90 -> 'A'
  | s when s >= 80 -> 'B'
  | s when s >= 70 -> 'C'
  | _ -> 'F'
```

Character ranges use the same guard approach or a `Char.code` comparison.

## Key Differences

1. **Pattern vs guard**: Rust has first-class range patterns; OCaml requires `when` guards for ranges — a fundamental syntactic difference.
2. **Char ranges**: Rust `'a'..='z'` is a pattern; OCaml handles this with `| c when Char.code c >= 97 && Char.code c <= 122`.
3. **Exhaustiveness**: Rust can verify that numeric range patterns cover the full domain; OCaml cannot do exhaustiveness checking for guarded arms.
4. **Exclusive ranges**: Rust's exclusive `..` is not yet stable in patterns (as of 2024); OCaml has no range patterns at all.

## Exercises

1. **Temperature zones**: Implement `fn climate_zone(temp_c: i32) -> &'static str` using range patterns to classify as "arctic" (below -20), "cold" (-20..=0), "temperate" (1..=20), "hot" (21..=40), "extreme" (above 40).
2. **ASCII classifier**: Write `fn ascii_category(c: u8) -> &'static str` using range patterns to classify bytes as control (0..=31), printable (32..=126), or extended (127..=255).
3. **Combined ranges**: Implement `fn is_weekend_hour(hour: u8) -> bool` using `0..=8 | 20..=23` or-with-range to identify typical non-business hours.
