[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 089 — Bob

## Problem Statement

Implement a conversational responder named Bob. Bob replies based on the characteristics of the input: silence gets "Fine. Be that way!", yelling gets "Whoa, chill out!", a question gets "Sure.", a yelled question gets "Calm down, I know what I'm doing!", and anything else gets "Whatever." Compare both a tuple-match and an if/else implementation.

## Learning Outcomes

- Use `.trim()`, `.ends_with('?')`, and `.chars().any(…)` for string inspection
- Compute `is_yelling` by checking both letter presence and full uppercase equality
- Pattern match on a tuple of booleans `(is_silence, is_yelling, is_question)` for clear dispatch
- Return `&'static str` for response literals — no allocation needed
- Map Rust's method-based string operations to OCaml's `String` module functions
- Understand the priority ordering in the match arms (silence checked first)

## Rust Application

`is_question` trims whitespace and checks `ends_with('?')`. `is_yelling` uses `.chars().any(|c| c.is_alphabetic())` to confirm at least one letter, then compares the string to its `.to_uppercase()` form — if they are equal, every letter is uppercase. `is_silence` checks `trim().is_empty()`. The `response_for` function matches a tuple `(is_silence(s), is_yelling(s), is_question(s))` — the first arm uses `true, _, _` to short-circuit on silence. Responses are `&'static str` literals, requiring no heap allocation.

## OCaml Approach

OCaml checks `is_question` by indexing the last character of the trimmed string. `is_yelling` uses `Seq.exists` on `String.to_seq` for letter presence and `String.uppercase_ascii` for comparison. `response_for` pattern-matches on the same triple. The logic is identical; OCaml's string API is more functional (`String.to_seq`, `Seq.exists`) whereas Rust uses method calls on `&str`.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Uppercase check | `s == s.to_uppercase()` | `String.uppercase_ascii s = s` |
| Letter presence | `.chars().any(c.is_alphabetic())` | `Seq.exists (fun c -> …)` |
| Trim | `.trim()` | `String.trim` |
| Last char | `.ends_with('?')` | `String.get s (len - 1) = '?'` |
| Dispatch | `match (bool, bool, bool)` | `match …, …, …` tuple |
| Response type | `&'static str` | `string` |

The tuple match is a clean alternative to nested if/else chains. Exhaustive checking ensures no case is forgotten — adding a new boolean flag forces updating every match arm. The `&'static str` return type for constant responses is a best practice: avoid allocating when the string is already in the binary.

## Exercises

1. Add a `is_polite(s: &str) -> bool` predicate that checks for "please" and integrate it into the response logic.
2. Implement a version that returns `String` (allocated) to allow parameterised responses (e.g. "Sure, {name}.").
3. Handle multi-line input: split on newlines, classify each line, and return a composite response.
4. Add a counter to track how many times Bob has been asked a question without `is_yelling`.
5. In OCaml, extend `response_for` to also detect silence made of only punctuation (e.g. `"..."`) and treat it as silence.
