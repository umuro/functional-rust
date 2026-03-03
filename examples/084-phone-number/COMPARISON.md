# Phone Number Parser — Comparison

## Core Insight
Validation pipelines demonstrate monadic chaining with Result. Both OCaml (`Result.bind`) and Rust (`.and_then()`) thread a value through a series of checks, short-circuiting on the first error.

## OCaml Approach
- `Result.bind` chains validations: `Ok x |> Result.bind f1 |> Result.bind f2`
- `String.to_seq |> Seq.filter |> String.of_seq` for digit extraction
- `d.[0]` for character access
- `Error "message"` for error variants

## Rust Approach
- `.and_then(f)` chains validations (same as `Result.bind`)
- `.chars().filter().collect()` for digit extraction
- `d.as_bytes()[0]` for byte access, `d.starts_with()` for prefix
- `Err("message")` with static string slices

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Bind | `Result.bind` | `.and_then()` |
| Filter chars | `Seq.filter` | `.chars().filter()` |
| String access | `s.[i]` (char) | `s.as_bytes()[i]` (u8) |
| Error type | `string` | `&'static str` |
| Substring | `String.sub d 1 10` | `d[1..].to_string()` |

## Learner Notes
- Rust's `?` operator is syntactic sugar for and_then in many cases
- `and_then` passes ownership of the Ok value to the next function
- OCaml's `|>` pipe and Rust's method chaining serve the same purpose
- Both approaches avoid nested if/else by linearizing validation steps
