# File Operation Errors — Comparison

## Core Insight
File operations always fail — the question is how richly you can classify and handle those failures.

## OCaml Approach
- `Sys_error of string` — one exception for all I/O failures
- Must parse the string to distinguish NotFound vs PermissionDenied
- `open_in`/`open_out` raise exceptions — need `try/with`
- Cleanup via `Fun.protect ~finally`

## Rust Approach
- `std::io::Error` with `ErrorKind` enum — structured classification
- `fs::read_to_string` returns `Result<String, io::Error>`
- Match on `err.kind()` for specific handling
- RAII handles cleanup (files close when dropped)

## Comparison Table

| Aspect | OCaml | Rust |
|--------|-------|------|
| Error type | `Sys_error of string` | `io::Error` with `ErrorKind` |
| Classification | Parse error string | Match `ErrorKind` enum |
| File read | `open_in` + `really_input_string` | `fs::read_to_string` |
| Cleanup | `Fun.protect ~finally` | RAII / `Drop` trait |
| Custom errors | Wrap in variant | `map_err` to app error |
| Error info | String message only | Kind + message + OS code |
