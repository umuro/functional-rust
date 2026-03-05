# Error Propagation Depth — Comparison

## Core Insight
Deep call stacks need error propagation that scales. Both `let*` (OCaml) and `?` (Rust) keep the code flat regardless of depth.

## OCaml Approach
- `let*` chains keep code linear through any number of layers
- Single error type shared across layers
- Each `let*` is one potential early exit point
- Without `let*`: deeply nested match expressions

## Rust Approach
- `?` on each fallible call — one character per layer
- Shared error enum or `From` impls for automatic conversion
- Each `?` is an early-return point
- Without `?`: deeply nested match or try! macro

## Comparison Table

| Aspect | OCaml `let*` | Rust `?` |
|--------|-------------|----------|
| Syntax per layer | `let* x = f in` | `let x = f?;` |
| Depth scaling | Linear | Linear |
| Error type | Must match or wrap | `From` auto-converts |
| Without sugar | Nested match | Nested match |
| Readability at 5 levels | Good | Good |
| Performance | Zero-cost | Zero-cost |
