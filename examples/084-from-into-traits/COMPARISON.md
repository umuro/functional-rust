## Core Insight

`From<T>` defines how to create a type from T. `Into<T>` is the reverse view. Implementing `From` auto-provides `Into`. This replaces ad-hoc conversion functions with a unified protocol.

## OCaml Approach
- Manual conversion functions: `int_of_float`, `string_of_int`
- No unified conversion trait
- Module-level `of_*` / `to_*` conventions

## Rust Approach
- `impl From<Source> for Target`
- `Into` comes free via blanket impl
- `.into()` for ergonomic conversion
- `TryFrom`/`TryInto` for fallible conversions

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Conversion | `of_string`, `to_int` functions | `From`/`Into` traits |
| Infallible | Manual function | `From`/`Into` |
| Fallible | Return `option`/`result` | `TryFrom`/`TryInto` |
| Auto | No | `Into` from `From` |
