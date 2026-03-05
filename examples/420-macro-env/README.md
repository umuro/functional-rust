📖 **[View on hightechmind.io →](https://hightechmind.io/rust/420-macro-env)**

---

# 420: env! and option_env! for Build-time Values

**Difficulty:** 2  **Level:** Intermediate

Embed environment variables directly into your binary at compile time — no runtime config file, no `std::env::var`, just constants baked in during the build.

## The Problem This Solves

Applications often need values that are fixed for a given build: version strings, API endpoints, feature flags, build timestamps. Reading them at runtime from environment variables or config files adds startup complexity, failure modes ("where's the config?"), and potential security exposure. Including them at compile time means they're constants — inlined, no parsing, no I/O, no missing values.

`env!("VAR")` reads an environment variable at compile time and produces a `&'static str`. If the variable isn't set, the *build* fails — not the runtime. This is a deliberate design: the problem is caught at the source. `option_env!("VAR")` returns `Option<&'static str>` for optional values, letting you handle absence gracefully at compile time.

Combined with Cargo's automatic variables (`CARGO_PKG_VERSION`, `CARGO_PKG_NAME`, `CARGO_MANIFEST_DIR`), this covers the most common case — version strings — without any extra setup.

## The Intuition

`env!("VAR")` reads an environment variable when `cargo build` runs and bakes it into the binary as a compile-time constant — the variable doesn't need to exist at runtime.

## How It Works in Rust

```rust
// Cargo automatically sets these — always available
const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

// Build fails if MY_API_KEY isn't set during cargo build
const API_KEY: &str = env!("MY_API_KEY");

// option_env! — None if not set, Some(&str) if set
const OPTIONAL_ENDPOINT: Option<&str> = option_env!("STAGING_URL");

fn main() {
    println!("{} v{}", PKG_NAME, VERSION);

    match OPTIONAL_ENDPOINT {
        Some(url) => println!("Using staging: {}", url),
        None      => println!("Using production endpoint"),
    }
}

// Common pattern: build info struct
pub struct BuildInfo {
    pub version: &'static str,
    pub pkg_name: &'static str,
    pub profile: &'static str,
}

pub const BUILD: BuildInfo = BuildInfo {
    version: env!("CARGO_PKG_VERSION"),
    pkg_name: env!("CARGO_PKG_NAME"),
    profile: if cfg!(debug_assertions) { "debug" } else { "release" },
};
```

1. `env!("NAME")` → `&'static str` constant. Build error if unset.
2. `option_env!("NAME")` → `Option<&'static str>`. `None` if unset, `Some(val)` if set.
3. Cargo pre-sets: `CARGO_PKG_VERSION`, `CARGO_PKG_NAME`, `CARGO_PKG_AUTHORS`, `CARGO_MANIFEST_DIR`.
4. For custom values, set the env var before `cargo build`, or use `build.rs` to emit `cargo:rustc-env=VAR=value`.

## What This Unlocks

- **Zero-overhead version strings**: `env!("CARGO_PKG_VERSION")` baked in as a static string — no parsing, no allocation.
- **CI/CD build tagging**: Inject git commit SHA, build timestamp, or CI pipeline ID at build time.
- **Fail-fast secrets**: Mandatory API keys cause build failure, not silent runtime misconfiguration.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Compile-time constants | `[%%getenv "VAR"]` (ppx_getenv) or custom PPX | `env!("VAR")` built-in |
| Package version | Manual or external tooling | `env!("CARGO_PKG_VERSION")` automatic |
| Optional build var | Custom handling required | `option_env!("VAR")` returns `Option` |
| Build script injection | Makefile / Dune env | `build.rs` emits `cargo:rustc-env=VAR=val` |
| Runtime env | `Sys.getenv` | `std::env::var("VAR")` (different from `env!`) |
