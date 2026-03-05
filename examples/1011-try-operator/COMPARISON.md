# The ? (Try) Operator — Comparison

## Core Insight
Rust's `?` is the ergonomic equivalent of OCaml's `let*` binding operator — both eliminate nested match/bind chains while keeping error handling explicit and typed.

## OCaml Approach
- Nested `match` expressions (verbose but clear)
- `>>=` bind operator (monadic, Haskell-style)
- `let*` binding operators (OCaml 4.08+, most ergonomic)
- All three require the same error type throughout

## Rust Approach
- `?` operator: `expr?` desugars to match + early return + `From::from(e)`
- Automatic `From` conversion means different error types can coexist
- Works in any function returning `Result` (or `Option`)
- Can be used in expression position: `parse(&read(key)?)?`

## Comparison Table

| Aspect | OCaml `let*` | Rust `?` |
|--------|-------------|----------|
| Syntax | `let* x = expr in` | `let x = expr?;` |
| Error conversion | None (same type required) | Automatic via `From` |
| Early return | Via continuation | Via `return Err(...)` |
| Nesting | Flat (monadic) | Flat (early return) |
| Works on | Result (custom) | Result, Option |
| Available since | OCaml 4.08 | Rust 1.13 (`try!` macro before) |
