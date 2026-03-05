## Core Insight

`Display` controls how a type is printed with `{}`. Unlike `Debug` (derived), `Display` must be manually implemented — it's the user-facing representation.

## OCaml Approach
- Write `to_string` function manually
- Use `Printf.sprintf` with format strings
- No unified "display" protocol

## Rust Approach
- `impl fmt::Display for Type`
- Enables `format!("{}", x)`, `println!("{}", x)`
- Single `fmt` method returns `fmt::Result`

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Protocol | Manual `to_string` | `impl Display` |
| Format string | `%s` with `to_string` | `{}` automatic |
| Debug | `#show` (ppx) | `#[derive(Debug)]` |
| Derive | No | Display: no, Debug: yes |
