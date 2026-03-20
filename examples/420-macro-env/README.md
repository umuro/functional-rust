📖 **[View on hightechmind.io →](https://hightechmind.io/rust/420-macro-env)**

---

# 420: `env!` and `option_env!` — Compile-time Environment Variables
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Version numbers, API keys for build-time compilation, build metadata, and package names should be embedded in binaries without being hardcoded as string literals that can drift from the actual package metadata. `env!("CARGO_PKG_VERSION")` reads the version from `Cargo.toml` at compile time, producing a `&'static str` with zero runtime cost. `option_env!` handles optional variables that may not be present, returning `Option<&'static str>`. This is how `--version` flags get their version strings, and how build-time configuration is embedded into binaries.

`env!` is used by virtually every CLI tool for `--version` output, embedded firmware for build metadata, and any binary that needs to know its own version at runtime.

## Learning Outcomes

- Understand how `env!` reads environment variables at compile time, not runtime
- Learn the standard Cargo-provided env vars: `CARGO_PKG_VERSION`, `CARGO_PKG_NAME`, `CARGO_PKG_AUTHORS`
- See how `option_env!` handles optional variables with `Option<&'static str>`
- Understand why compile-time embedding is preferable to runtime `std::env::var` for version info
- Learn how CI systems can inject build metadata via environment variables

## Rust Application

In `src/lib.rs`, `VERSION`, `PKG_NAME`, and `AUTHORS` are `const &str` values read from Cargo-provided environment variables. These are set automatically by `cargo build`. `full_version()` concatenates them at runtime but both strings are `'static`. `build_profile()` uses `option_env!("PROFILE")` with `.unwrap_or("unknown")` for an optional variable. `optional_api_key()` returns `Option<&'static str>` for a key that may be absent.

## OCaml Approach

OCaml dune build system generates `Version.ml` files from opam metadata: `(library (name mylib) (inline_tests) (preprocessor_deps ../CHANGES.md))`. The `%%VERSION%%` substitution in `dune` files inserts the package version. OCaml's `Sys.argv.(0)` provides the binary name but not version. Libraries like `build_info` generate OCaml modules from build metadata. There is no direct equivalent of `env!` — all approaches require build system configuration.

## Key Differences

1. **Built-in Cargo vars**: Rust has documented, guaranteed `CARGO_PKG_*` variables; OCaml requires explicit `dune` configuration to expose package metadata.
2. **Zero config**: `env!("CARGO_PKG_VERSION")` requires no setup beyond having a `Cargo.toml`; OCaml's equivalent requires dune rules.
3. **Optional**: `option_env!` returns `Option<&'static str>` cleanly; OCaml's equivalent requires conditional compilation or runtime checks.
4. **Build reproducibility**: Compile-time env vars baked into the binary make builds non-reproducible if the env changes; both languages face this trade-off.

## Exercises

1. **Version command**: Create a `print_version()` function that prints `"{PKG_NAME} {VERSION}\nBuilt by: {AUTHORS}\nProfile: {PROFILE}"` using all available Cargo env vars. Call it from a `main.rs` with `--version` flag handling.
2. **Build-time API key**: Use `option_env!("SENTRY_DSN")` to conditionally initialize error reporting. If the key is present at compile time, initialize a mock Sentry client; otherwise print a warning that error reporting is disabled.
3. **Custom build info**: Write a `build.rs` script that sets `cargo:rustc-env=BUILD_TIMESTAMP={timestamp}` using `SystemTime::now()`. Access it with `env!("BUILD_TIMESTAMP")` in the library and verify the timestamp is baked in.
