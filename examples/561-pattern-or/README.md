📖 **[View on hightechmind.io →](https://hightechmind.io/rust/561-pattern-or)**

---

# Or Patterns
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Matching the same action for multiple variants is a common need in language interpreters, state machines, and data validation. Before Rust 2021's or-pattern stabilization, developers had to either duplicate arms or use `if matches!(...)` guards. Or patterns (`|` in match arms and `if let`) allow matching several alternatives with one arm, keeping code DRY and exhaustiveness checking intact. OCaml has always had this with its `|` in `match`, making it a natural comparison point.

## Learning Outcomes

- How `|` in a single match arm matches multiple alternatives
- How `matches!(value, A | B | C)` provides a concise boolean check
- How or-patterns work in `if let` and `let` destructuring
- How or-patterns interact with enum variants, ranges, and literals
- Where or-patterns reduce duplication in state machine transitions and input validation

## Rust Application

`is_vowel(c: char) -> bool` uses `matches!(c, 'a' | 'e' | ... )` — a boolean check over multiple char alternatives. `describe_number(n: i32)` uses `1 | 2 | 3 => "small"` in a match arm. `is_primary(c: &Color)` uses `matches!(c, Color::Red | Color::Green | Color::Blue)`. Or-patterns in match arms are exhaustiveness-checked — adding a new variant causes a compile warning unless covered. The `|` separates alternatives anywhere a pattern is valid.

Key patterns:
- `1 | 2 | 3 => expr` — multiple literals in one arm
- `matches!(val, Pat1 | Pat2)` — boolean or-pattern check
- `Color::A | Color::B => ...` — enum variant or-pattern

## OCaml Approach

OCaml has had or-patterns since its earliest versions:

```ocaml
let is_vowel c = match c with
  | 'a' | 'e' | 'i' | 'o' | 'u' -> true
  | _ -> false

let describe_number n = match n with
  | 1 | 2 | 3 -> "small"
  | 4 | 5 | 6 -> "medium"
  | _ -> "other"
```

The syntax is identical in spirit to Rust's.

## Key Differences

1. **Stabilization history**: OCaml has had or-patterns since v1; Rust stabilized them fully in edition 2021 — earlier editions required separate arms.
2. **Scope of binding**: Rust requires all alternatives in an or-pattern to bind the same names with the same types; OCaml has the same restriction.
3. **`matches!` macro**: Rust's `matches!` is a convenient shorthand; OCaml achieves the same with a `function | Pat1 | Pat2 -> true | _ -> false`.
4. **Nested or-patterns**: Rust supports nested or-patterns inside destructuring (`Some(1 | 2)`); OCaml supports the same.

## Exercises

1. **Operator classifier**: Write `fn is_arithmetic_op(c: char) -> bool` using or-patterns to check for `+`, `-`, `*`, `/`, `%`.
2. **Status grouping**: Implement `fn http_category(code: u16) -> &'static str` using or-patterns and ranges to classify 200-299 as "success", 400-499 as "client error", 500-599 as "server error".
3. **Variant groups**: Create a `KeyEvent` enum with many variants and use or-patterns to group them into "printable", "control", and "navigation" in a single `categorize` function.
