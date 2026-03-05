# Network Error Classification — Comparison

## Core Insight
Network errors need classification (retryable? client error? transient?) for proper handling. Both languages use pattern matching, but Rust's methods on enums keep the logic co-located with the type.

## OCaml Approach
- Variant type with all error kinds
- Standalone `is_retryable` function matches on variants
- Retry logic uses recursive function with decrementing counter
- `string_of_*` functions for display

## Rust Approach
- Enum with methods: `impl NetError { fn is_retryable(&self) -> bool }`
- Pattern matching with guards: `Err(e) if e.is_retryable()`
- Retry loop with attempt counter
- `Display` trait for formatting

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Error type | Variant type | Enum |
| Classification | Standalone function | Method on enum |
| Retry guard | `when is_retryable e` | `if e.is_retryable()` |
| Structured data | `HttpError of int * string` | `HttpError { status, body }` |
| Display | `string_of_net_error` | `impl Display` |
| Methods on error | Not idiomatic | Very idiomatic |
