# Validation Errors — Comparison

## Core Insight
Standard `Result` + `?` short-circuits at the first error. Validation needs to report ALL errors. Both languages solve this by collecting errors into a list.

## OCaml Approach
- Each validator returns `field_error list` (empty = valid)
- Concatenate with `@` operator
- Applicative-style: validate independently, merge error lists

## Rust Approach
- Each validator returns `Vec<FieldError>` (empty = valid)
- `extend()` to accumulate across validators
- Functional: `filter` + `map` + `collect` for rule-based checks

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Error accumulator | `error list` | `Vec<Error>` |
| Concatenation | `@` (list append) | `extend()` / `concat()` |
| Per-field validators | Return `[] \| [err]` | Return `Vec::new() \| vec![err]` |
| Short-circuit option | `Result.bind` / `let*` | `?` operator |
| Non-short-circuit | Collect lists | Collect Vecs |
| Libraries | Custom | `validator` crate |
