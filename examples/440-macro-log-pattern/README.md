📖 **[View on hightechmind.io →](https://hightechmind.io/rust/440-macro-log-pattern)**

---

# 440: Log Pattern Macros
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Logging requires consistent metadata: timestamp, level, file, line number, and the actual message. Writing `eprintln!("[INFO] {}:{}: {}", file!(), line!(), msg)` at every log site is verbose and inconsistent. Log macros capture this metadata automatically at the call site using `file!()` and `line!()` built-in macros. The `log` crate standardizes this with `log::info!`, `log::warn!`, `log::error!` macros that also support multiple backends. Understanding the underlying macro pattern explains why log calls show the correct source location.

Log macros appear in virtually every serious Rust codebase via the `log` crate, `tracing` for structured logging, and custom logging macros for specific output formats.

## Learning Outcomes

- Understand how `file!()` and `line!()` capture the call site location in macros
- Learn how `format!($($arg:tt)*)` inside a macro passes format strings and arguments through
- See how convenience macros (`info!`, `warn!`, `error!`) wrap a general `log!` macro
- Understand `trace_fn!` as a structural logging pattern for function entry/exit tracking
- Learn how the `log` crate abstracts over logging backends using the same macro interface

## Rust Application

In `src/lib.rs`, `log!($level, $($arg:tt)*)` captures the level, then uses `file!()`, `line!()`, and `format!($($arg)*)` in the output. The `$($arg:tt)*` fragment passes any format string and arguments through to the inner `format!` call. `info!`, `warn!`, `error!` are thin wrappers over `log!` fixing the level string. `trace_fn!($name, $body)` wraps a block with entry/exit logging, returning the block's value.

## OCaml Approach

OCaml's `Logs` library provides level-based logging with `Logs.info`, `Logs.warn`, `Logs.err`. Source location is captured via `__LOC__` special value. `Ppx_log` provides `[%log.info ...]` syntax capturing locations automatically. The `tracing` ecosystem doesn't have a direct OCaml equivalent, though `Eio`'s traces come close for structured concurrent tracing.

## Key Differences

1. **Location capture**: Rust uses `file!()` and `line!()` macros in the expansion; OCaml uses `__LOC__` string (containing file:line:col).
2. **Format passthrough**: Rust's `$($arg:tt)*` passes format arguments through; OCaml's functions use `('a, Format.formatter, unit) format` types.
3. **Level-as-macro**: Rust's `info!(...)` and `warn!(...)` are macros, not functions; OCaml's `Logs.info` and `Logs.warn` are functions with callbacks.
4. **Structured logging**: `tracing` crate adds spans and structured fields to Rust logs; OCaml's `Logs` is simpler, lacking structured fields without PPX.

## Exercises

1. **Log to file**: Extend the `log!` macro to accept an optional `writer: impl Write` argument: `log_to!(writer, INFO, "message: {}", value)`. Default to `eprintln!` when no writer is specified.
2. **Structured log**: Create `slog!(level = "INFO", component = "db", "query {}", sql)` that outputs `{"level":"INFO","component":"db","file":"...","line":...,"msg":"..."}` as JSON. Use `serde_json` or manual formatting.
3. **Tracing span**: Implement `span!(name, { /* body */ })` that generates a unique span ID, logs entry with the span ID, executes the body, then logs exit with elapsed time and the same span ID — enabling correlation of nested spans in logs.
