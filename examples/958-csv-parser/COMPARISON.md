# CSV Parser — Comparison

## Core Insight
CSV parsing requires a state machine to handle quoted fields. The algorithm is identical in both languages. OCaml expresses mutable state via `ref` cells; Rust uses `let mut` variables. Both use `Buffer`/`String` for accumulating the current field. The Rust `enum` for state is more idiomatic than OCaml's `type state`.

## OCaml Approach
- `type state = Normal | InQuote | AfterQuote` — custom variant type
- `state := InQuote` — mutable reference cell update
- `Buffer.create`, `Buffer.add_char`, `Buffer.contents` — mutable character accumulation
- `for i = 0 to n - 1 do ... done` — imperative iteration over string indices
- `String.split_on_char '\n'` for line splitting
- `List.filter_map` to skip empty lines

## Rust Approach
- `enum State { Normal, InQuote, AfterQuote }` — same concept, idiomatic Rust
- `state = State::InQuote` — direct assignment of enum variant
- `String::new()`, `push(c)`, `clear()` — mutable String accumulation
- `for c in line.chars()` — iterator over chars (Unicode-safe)
- `match (&state, c)` — tuple pattern matching on (state, char) pair
- `text.lines().filter(...).map(...).collect()` — functional pipeline for rows

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| State type | `type state = ...` | `enum State { ... }` |
| State mutation | `state := InQuote` | `state = State::InQuote` |
| Char accumulation | `Buffer.add_char current c` | `current.push(c)` |
| String from buffer | `Buffer.contents current` | `current.clone()` |
| Loop style | `for i = 0 to n-1` | `for c in line.chars()` |
| Pattern on pair | `match !state, c with` | `match (&state, c)` |
| Line iteration | `String.split_on_char '\n'` | `text.lines()` |
| Skip empty | `List.filter_map` | `.filter(\|l\| !l.is_empty())` |
