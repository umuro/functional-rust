# Panic vs Result — Comparison

## Core Insight
Both languages have two error channels: one for bugs (panic/exception) and one for expected failures (Result). The key is knowing which to use when.

## OCaml Approach
- `failwith`, `invalid_arg`, `assert false` — for bugs
- `Result` type — for expected failures
- Exceptions are lightweight (no stack trace by default)
- Culture: exceptions used more liberally than Rust panics

## Rust Approach
- `panic!`, `unwrap()`, `expect()`, `unreachable!()` — for bugs
- `Result<T, E>` — for expected failures
- Panics unwind the stack (or abort, configurable)
- Culture: strong preference for Result; panic = something went very wrong

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Bug/invariant | `failwith` / `assert false` | `panic!` / `unreachable!` |
| Quick unwrap | `Option.get` (unsafe) | `unwrap()` / `expect()` |
| Expected failure | `Result` / `Option` | `Result` / `Option` |
| Debug-only check | N/A | `debug_assert!` |
| Custom message | `invalid_arg "msg"` | `expect("msg")` |
| Cultural norm | Exceptions common | Result strongly preferred |
