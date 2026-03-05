# Custom Error Display — Comparison

## Core Insight
Error messages need layers: "what went wrong" (Display), "why" (source chain), and "where" (Debug). Rust's `Error` trait standardizes all three.

## OCaml Approach
- Manual `string_of_*` functions at each level
- Nested record types with explicit `inner`/`source` fields
- Chain display by recursive function
- No standard trait — each project defines its own convention

## Rust Approach
- `Display` trait: human-readable message for THIS error only
- `Error::source()`: returns reference to underlying cause
- Walk chain with `while let Some(e) = current.source()`
- `Debug` trait: programmer-readable with full structure

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Display | `string_of_*` function | `impl Display` |
| Source chain | Manual field access | `Error::source()` method |
| Chain walking | Recursive function | While-let loop |
| Standard trait | No | `std::error::Error` |
| Debug output | `[@@deriving show]` (ppx) | `#[derive(Debug)]` |
| Inline format | Manual concatenation | `display_inline` via `join(" -> ")` |
