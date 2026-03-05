## Core Insight

Railway-oriented programming models computation as two tracks: success and failure. Each function either continues on the success track or diverts to the error track. Result's bind/and_then is the switch.

## OCaml Approach
- `Result.bind` chains fallible operations
- `let*` binding operators for monadic syntax
- Error track accumulates through the pipeline

## Rust Approach
- `?` operator is railway switching — returns Err early
- `.and_then()` for explicit chaining
- `.map_err()` to convert between error types on the error track

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Switch to error | `Error e` | `Err(e)` / `?` returns |
| Stay on success | `Ok x` | `Ok(x)` |
| Chain | `Result.bind` | `.and_then()` / `?` |
| Transform error | `Result.map_error` | `.map_err()` |
