# Result Combinators — Comparison

## Core Insight
Both OCaml and Rust treat Result as a monad with `map` (functor) and `bind`/`and_then` (monadic bind). Rust adds more built-in combinators.

## OCaml Approach
- `Result.map`, `Result.bind` in stdlib (OCaml 4.08+)
- Custom `map_error`, `or_else` typically hand-written
- Pipeline via `|>` operator
- `Option.value ~default` for unwrap-with-default

## Rust Approach
- Rich built-in: `map`, `map_err`, `and_then`, `or_else`, `unwrap_or_else`, `unwrap_or_default`
- Method chaining with `.` notation
- `?` operator as syntactic sugar for `and_then` + early return
- `ok()`, `err()` to convert between Result and Option

## Comparison Table

| Combinator | OCaml | Rust |
|-----------|-------|------|
| map | `Result.map f r` | `r.map(f)` |
| flatmap/bind | `Result.bind r f` | `r.and_then(f)` |
| map error | custom `map_error` | `r.map_err(f)` |
| fallback | custom `or_else` | `r.or_else(f)` |
| default | `Result.value r ~default` | `r.unwrap_or(v)` |
| lazy default | custom | `r.unwrap_or_else(f)` |
