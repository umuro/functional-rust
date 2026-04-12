[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 084 — Phone Number Validation
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Parse and validate a North American phone number from a free-form string (`(223) 456-7890`, `+1 223-456-7890`, etc.). Strip non-digit characters, normalise 11-digit numbers with a leading `1`, then validate the area code and exchange codes. Implement both an imperative and an `and_then`-chain version, comparing with OCaml's `Result.bind` pipeline.

## Learning Outcomes

- Use `.chars().filter(…).collect()` to extract digits from a string
- Chain `and_then` on `Result` to build a validation pipeline without nested `match`
- Use byte indexing (`d.as_bytes()[0]`) for single-byte ASCII comparisons
- Understand `&'static str` as the error type for string literals
- Compose small single-purpose validation functions for testability
- Map Rust's `and_then` to OCaml's `Result.bind` operator

## Rust Application

`digits_only` filters non-digit characters via `.chars().filter(|c| c.is_ascii_digit()).collect::<String>()`. The imperative `validate` handles length normalisation with if/else and returns early on invalid codes. The functional `validate_chain` composes `normalize_length`, `check_area_code`, and `check_exchange` using `and_then` — each function returns `Result<String, &'static str>` and the chain stops at the first `Err`. Using `&'static str` for errors avoids allocation; `String` errors would require `map_err` at the call site.

## OCaml Approach

OCaml's `Result.bind` (`|> Result.bind (fun d -> ...)`) chains validation steps. `String.to_seq`, `Seq.filter`, and `String.of_seq` extract digits. Individual checks use `if … then Error "…" else Ok d` — identical logic to the Rust version. The `|>` pipe makes the chain read left-to-right, equivalent to Rust's method chain. Both versions express: "start with the string, normalise, check area code, check exchange — fail at the first error."

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Error type | `&'static str` | `string` |
| Chaining | `.and_then(f)` | `|> Result.bind (fun d -> ...)` |
| Digit filter | `.chars().filter(…).collect()` | `String.to_seq |> Seq.filter |> String.of_seq` |
| Byte indexing | `d.as_bytes()[0]` | `d.[0]` (char indexing) |
| Early return | `return Err(…)` | Pattern on prev Result |
| Pipeline style | Method chain | Pipe operator `|>` |

The validation pipeline pattern — normalise, then check each rule in sequence — is a common functional idiom. Using `and_then`/`Result.bind` keeps each check independent and composable, unlike nested `match` or `if/else` chains that tangle logic across rules.

## Exercises

1. Add a `format_number(s: &str) -> Result<String, &'static str>` function that returns the number formatted as `(NXX) NXX-XXXX`.
2. Extend validation to reject numbers with all the same digit (e.g. `111-111-1111`).
3. Collect all errors (not just the first) by returning `Result<String, Vec<&'static str>>` and accumulating failures.
4. Write a version using `?` in a helper function instead of `and_then` chaining, and compare readability.
5. In OCaml, use the `let*` syntax (monadic bind with `ppx_let` or `Result.bind` desugared) to write the validation in `do`-notation style.
