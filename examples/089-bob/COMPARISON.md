# Bob — Comparison

## Core Insight
Bob demonstrates tuple pattern matching on computed boolean conditions. The approach is identical in both languages — compute predicates, match on the tuple. String operations differ slightly in ergonomics.

## OCaml Approach
- `String.trim`, `String.uppercase_ascii` for string ops
- `String.to_seq |> Seq.exists` for character testing
- Manual last-character check: `String.get s (String.length s - 1)`
- Tuple match: `match a, b, c with | true, _, _ -> ...`

## Rust Approach
- `.trim()`, `.to_uppercase()` — method syntax
- `.chars().any(|c| c.is_alphabetic())` for character testing
- `.ends_with('?')` — dedicated method
- Tuple match: `match (a, b, c) { (true, _, _) => ... }`

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Trim | `String.trim` | `.trim()` |
| Uppercase | `String.uppercase_ascii` | `.to_uppercase()` |
| Ends with | Manual char check | `.ends_with()` |
| Has letter | `Seq.exists` | `.chars().any()` |
| Tuple match | `match a, b, c with` | `match (a, b, c)` |
| Result type | `string` | `&'static str` |

## Learner Notes
- Rust's `.ends_with()` is more ergonomic than OCaml's manual index check
- `&'static str` for constant strings avoids allocation
- Both languages handle tuple matching the same way — very elegant pattern
- Rust's `.is_alphabetic()` handles Unicode; OCaml's manual check doesn't
