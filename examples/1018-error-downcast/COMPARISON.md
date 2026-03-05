# Error Downcast — Comparison

## Core Insight
Type erasure (`Box<dyn Error>`) is convenient but loses type information. Downcasting recovers it at runtime — OCaml's exception matching does this naturally, while Rust needs explicit downcasts.

## OCaml Approach
- Exceptions are pattern-matchable by default — no "downcast" needed
- Extensible variant types (`type t += ...`) support open matching
- GADTs can encode typed error containers
- Pattern matching is exhaustive (or has wildcard)

## Rust Approach
- `downcast_ref::<T>()` — borrow as concrete type (returns `Option`)
- `downcast::<T>()` — take ownership (returns `Result`)
- Uses `TypeId` internally (runtime reflection)
- Unavoidable when working with `Box<dyn Error>` from libraries

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Type recovery | Pattern matching | `downcast_ref` / `downcast` |
| Compile-time safe | Yes (match) | No (runtime check) |
| Cost | Zero | TypeId comparison |
| Ownership | N/A | `downcast` consumes Box |
| Preferred approach | Exceptions / variants | Typed enum (avoid downcast) |
