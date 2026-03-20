📖 **[View on hightechmind.io →](https://hightechmind.io/rust/419-macro-cfg-attr)**

---

# 419: `cfg!` and Conditional Compilation

## Problem Statement

Cross-platform libraries must support Linux, macOS, Windows, and embedded targets — each with different APIs, file paths, and system calls. Feature flags enable shipping a core library with optional capabilities that users opt into. Compiling debug-only code into release builds wastes binary space and performance. Conditional compilation solves all of these: `#[cfg(target_os = "linux")]` includes code only on Linux, `#[cfg(feature = "advanced")]` only when the feature is enabled, `#[cfg(debug_assertions)]` only in debug builds. The compiler eliminates excluded branches entirely — zero runtime cost.

`#[cfg(...)]` powers `tokio`'s platform backends, `std`'s OS-specific implementations, `serde`'s feature-gated formats, and any multi-platform Rust library.

## Learning Outcomes

- Understand `#[cfg(condition)]` for item-level conditional compilation
- Learn `cfg!()` macro for inline conditional expressions
- See how `target_os`, `target_arch`, `feature`, `debug_assertions` are used
- Understand how `#[cfg_attr(condition, attribute)]` conditionally applies attributes
- Learn how cargo features enable/disable code paths via `#[cfg(feature = "name")]`

## Rust Application

In `src/lib.rs`, `path_separator()` uses `if cfg!(windows)` for runtime-branch-based conditional (the compiler still eliminates the dead branch). `debug_log` uses two separate `#[cfg(debug_assertions)]` and `#[cfg(not(debug_assertions))]` items — one exists in debug, the other in release. `advanced_feature()` is similarly gated by `#[cfg(feature = "advanced")]`. `os_name()` uses nested `#[cfg(target_os = "...")]` return statements.

## OCaml Approach

OCaml achieves conditional compilation through the `dune` build system. `(libraries (select lib.ml from (linux -> linux_impl.ml) (windows -> windows_impl.ml)))` selects platform-specific files. The `Sys.os_type` variable provides runtime OS detection. Feature flags are handled through dune's `(flags ...)` and C preprocessor `#ifdef` for C stubs. OCaml has no built-in `cfg!` equivalent — all conditional compilation is at the file/module level.

## Key Differences

1. **Granularity**: Rust conditions can wrap individual items, expressions, or attributes; OCaml's conditional compilation is file-level.
2. **Cargo integration**: Rust `#[cfg(feature = "x")]` integrates directly with `Cargo.toml`'s `[features]`; OCaml requires dune configuration.
3. **Dead code elimination**: Rust's compiler guarantees that `#[cfg(not(...))]` items are completely absent from the binary; OCaml's `if Sys.os_type = "Unix"` branches exist in the binary.
4. **Attribute conditionals**: Rust's `#[cfg_attr(condition, derive(Serialize))]` conditionally applies attributes; OCaml has no equivalent.

## Exercises

1. **Platform utilities**: Write a `platform_temp_dir() -> &'static str` function returning `"/tmp"` on Unix and `"C:\\Temp"` on Windows using `#[cfg(target_family = "unix")]` and `#[cfg(target_family = "windows")]`.
2. **Feature-gated struct**: Define a `MetricsCollector` struct that is only compiled when feature `"metrics"` is enabled. For the non-metrics build, provide a zero-size stub with the same API that compiles away entirely.
3. **Debug assertions**: Implement a `safe_divide(a: i32, b: i32) -> i32` that uses `debug_assert!(b != 0)` to catch division by zero in debug builds, panicking with a message. In release, skip the check for performance.
