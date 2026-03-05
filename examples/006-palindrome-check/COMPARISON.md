## Core Insight

A palindrome reads the same forwards and backwards. The simplest check: reverse and compare. Both languages make this a one-liner, but the underlying data structures differ (OCaml string vs Rust UTF-8 String).

## OCaml Approach
- Convert string to char list, reverse, compare
- Or use index-based comparison from both ends
- `String.to_seq` + `List.of_seq` for char extraction

## Rust Approach
- `s.chars().rev().collect::<String>() == s`
- Or `s.chars().eq(s.chars().rev())` — no allocation
- `.chars()` handles UTF-8 correctly

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Reverse string | `String.to_seq \|> List.of_seq \|> List.rev` | `s.chars().rev()` |
| Compare | `=` structural equality | `==` or `.eq()` |
| UTF-8 | Byte-based strings | `.chars()` for Unicode |
| Zero-alloc check | Index loop | `.chars().eq(s.chars().rev())` |
