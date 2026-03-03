# Allergies — Comparison

## Core Insight
Bitflag decoding maps cleanly between both languages. The pattern — enumerate variants, assign power-of-2 scores, use bitwise AND to test membership — is universal. The difference is syntactic, not conceptual.

## OCaml Approach
- `land` operator for bitwise AND
- Variant list `[Eggs; Peanuts; ...]` as the universe of allergens
- `List.filter` to find matching allergens
- `function` keyword for concise pattern match

## Rust Approach
- `&` operator for bitwise AND
- `const ALL` array on the enum for iteration
- `.filter().collect()` with iterator chain
- `1 << i` alternative using bit position

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Bitwise AND | `land` | `&` |
| Enum list | `let all = [...]` | `const ALL: [Allergen; 8]` |
| Filter | `List.filter` | `.filter().collect()` |
| Score type | `int` | `u32` |
| String name | Manual function | Method returning `&'static str` |

## Learner Notes
- Rust has no `land`/`lor` — uses C-style `&`, `|`, `^` operators
- `score & (1 << i)` is an alternative to explicit score matching
- Both languages guarantee exhaustive matches — adding an allergen forces updates
- Consider `bitflags` crate for production Rust bitflag patterns
