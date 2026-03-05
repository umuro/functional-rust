📖 **[View on hightechmind.io →](https://hightechmind.io/rust/419-macro-cfg-attr)**

---

# 419: cfg! and cfg_attr for Conditional Code

**Difficulty:** 2  **Level:** Intermediate

Compile-time conditional compilation — include code only for specific targets, features, or test builds.

## The Problem This Solves

A single codebase needs to run on Linux and Windows, in debug and release, with and without optional features, and in test builds that need extra derives for mocking. Duplicating code per target is unmanageable. Runtime `if cfg!(target_os = "windows")` works but wastes binary space compiling both branches. You need compile-time branching.

`#[cfg(...)]` annotations remove items from compilation entirely — the excluded code doesn't exist in the binary. `cfg_attr` goes further: conditionally apply *attributes* based on the same predicates. The common use is `#[cfg_attr(test, derive(Mock))]` — only derive the test-heavy mock implementation in test builds, keeping production binaries lean.

`cfg!` (with `!`) works inside expressions, returning a `bool` at compile time. It's useful for conditional logic within a function body. But prefer `#[cfg]` on whole items where possible — it eliminates dead code entirely instead of just branching over it.

## The Intuition

`#[cfg(condition)]` is a compile-time `if` that removes the entire item from the binary when the condition is false; `cfg_attr` conditionally applies attributes like `derive` based on the same conditions.

## How It Works in Rust

```rust
// #[cfg] — include item only when condition holds
#[cfg(target_os = "linux")]
fn platform_info() -> &'static str { "Running on Linux" }

#[cfg(target_os = "windows")]
fn platform_info() -> &'static str { "Running on Windows" }

// cfg_attr — apply derive only in test builds
#[derive(Debug, Clone)]
#[cfg_attr(test, derive(PartialEq))]  // PartialEq only for tests
struct Config { host: String, port: u16 }

// Feature flags (Cargo.toml: [features] async-support = [])
#[cfg(feature = "async-support")]
async fn fetch_data() -> String { todo!() }

// cfg! macro — inline boolean expression
fn describe_build() -> &'static str {
    if cfg!(debug_assertions) { "debug" } else { "release" }
}

// Combine conditions
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn optimized_path() { /* x86_64 Linux only */ }

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn apple_only() { /* macOS or iOS */ }
```

1. `#[cfg(condition)]` on an item — item vanishes from binary if false.
2. `#[cfg_attr(condition, attribute)]` — apply `attribute` only when condition holds.
3. `cfg!()` in expressions — compile-time `bool`, both branches still compiled.
4. Conditions: `target_os`, `target_arch`, `feature = "..."`, `test`, `debug_assertions`.

## What This Unlocks

- **Test-only derives**: `#[cfg_attr(test, derive(PartialEq, Mock))]` — zero test infrastructure in production binaries.
- **Platform-specific code**: Clean per-OS implementations without `if/else` runtime checks.
- **Feature flags**: Optional features in libraries that users opt into via `Cargo.toml`.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Conditional compilation | `#ifdef` via C preprocessor (FFI) or Dune conditions | `#[cfg(...)]` built into language |
| Feature flags | opam optional dependencies, Dune `(enabled_if ...)` | `Cargo.toml` `[features]`, `#[cfg(feature="...")]` |
| Platform detection | `Sys.os_type` at runtime | `cfg!(target_os)` at compile time |
| Conditional attributes | Not applicable | `#[cfg_attr(test, derive(...))]` |
| Test-only code | `let%test` or `(test (libraries ...))` | `#[cfg(test)]` module |
