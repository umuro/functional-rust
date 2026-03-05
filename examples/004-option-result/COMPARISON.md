## Core Insight

`Option` replaces null pointers. `Result` replaces exceptions. Both languages encode success/failure in the type system, forcing the caller to handle every case.

## OCaml Approach
- `option` type: `Some x | None`
- `result` type: `Ok x | Error e`
- `Option.map`, `Option.bind` for chaining
- `Result.map`, `Result.bind` for chaining
- Pattern matching is the primary way to unwrap

## Rust Approach
- `Option<T>`: `Some(x)` / `None`
- `Result<T, E>`: `Ok(x)` / `Err(e)`
- `.map()`, `.and_then()` for chaining
- `?` operator for early return on error
- `.unwrap_or()`, `.unwrap_or_else()` for defaults

## Comparison Table

| Feature | OCaml | Rust |
|---------|-------|------|
| Option type | `'a option` | `Option<T>` |
| Result type | `('a, 'e) result` | `Result<T, E>` |
| Map | `Option.map f o` | `o.map(f)` |
| Bind/FlatMap | `Option.bind o f` | `o.and_then(f)` |
| Default | `Option.value ~default o` | `o.unwrap_or(d)` |
| Error propagation | Pattern match | `?` operator |
