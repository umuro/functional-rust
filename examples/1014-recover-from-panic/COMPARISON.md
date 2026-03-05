# Recover from Panic — Comparison

## Core Insight
OCaml's `try/with` is everyday error handling; Rust's `catch_unwind` is an escape hatch. This reflects their different philosophies on exceptions vs typed errors.

## OCaml Approach
- `try expr with pattern -> handler` — standard, idiomatic
- Can catch specific exceptions or all with wildcard
- `Fun.protect ~finally` for cleanup (like try/finally)
- Exceptions are cheap and common in OCaml

## Rust Approach
- `std::panic::catch_unwind` converts panic to `Result<T, Box<dyn Any>>`
- Requires `UnwindSafe` bound (or `AssertUnwindSafe` wrapper)
- Only catches unwinding panics (not `abort` mode)
- Intended for FFI boundaries, thread pools, not normal flow

## Comparison Table

| Aspect | OCaml `try/with` | Rust `catch_unwind` |
|--------|-----------------|-------------------|
| Idiomacy | Standard practice | Last resort |
| Overhead | Near zero | Stack unwinding |
| Type safety | Pattern matching | `Box<dyn Any>` downcast |
| Cleanup | `Fun.protect ~finally` | `Drop` trait (RAII) |
| Abort mode | N/A | Panics can't be caught |
| Use case | Normal error handling | FFI / thread isolation |
