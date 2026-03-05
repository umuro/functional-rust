# Error Conversion — Comparison

## Core Insight
Rust's `From` trait + `?` operator automates what OCaml forces you to do manually: wrapping sub-errors into a unified error type.

## OCaml Approach
- Must manually wrap each sub-error: `Error (IoError e)` at every call site
- Can write `lift_*` helper functions but they're boilerplate
- No language-level support for automatic error conversion
- Each new error source means another wrapper call

## Rust Approach
- Implement `From<SubError> for UnifiedError` once per sub-error type
- The `?` operator automatically calls `.into()` which uses `From`
- Adding a new error source = one new `From` impl, zero call-site changes
- The `source()` method preserves the error chain

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Conversion mechanism | Manual wrapping | `From` trait + `?` |
| Boilerplate per call site | One wrapper per call | Zero (automatic) |
| Adding new error source | Touch every call site | One `From` impl |
| Error chain | Manual nesting | `source()` method |
| Type safety | Variant pattern match | Same + compiler enforced |
