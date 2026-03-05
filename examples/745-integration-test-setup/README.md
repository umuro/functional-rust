📖 **[View on hightechmind.io →](https://hightechmind.io/rust/745-integration-test-setup)**

---

# 745: Integration Test Structure: tests/ Directory

**Difficulty:** 2  **Level:** Intermediate

Files in the `tests/` directory test your library's public API exactly as an external user would — each file is its own crate.

## The Problem This Solves

Unit tests in `#[cfg(test)]` blocks are great for individual functions, but they can access private internals. You need a separate layer that tests only what you publicly export — the way a downstream crate would use your library. If a refactor breaks the public API while keeping all unit tests green, you want to catch that.

Integration tests also let you test realistic workflows that span multiple functions. Validating a config object, connecting to a service, and making a request is a multi-step scenario that belongs in integration tests, not unit tests.

The `tests/` directory also solves the problem of shared test utilities. A `tests/common/mod.rs` file can export fixture builders and assertion helpers that multiple test files share — without that code appearing in any test file itself (Rust treats `common/mod.rs` as a helper, not a test file, because it has no `#[test]` functions at the top level).

## The Intuition

In Python, integration tests often live in a separate `tests/integration/` folder and import from your package's public API. In Jest, you'd import from the package entry point rather than internal modules.

Rust enforces this boundary structurally: files in `tests/` are compiled as separate crates. They can only use items you've marked `pub`. The compiler physically cannot see your private functions. There's no lint rule needed — the visibility rules do the work.

Each file in `tests/` is an independent test binary. Running `cargo test` compiles and runs all of them. You can run a specific integration test file with `cargo test --test config_test`.

## How It Works in Rust

Real project layout:

```
my_crate/
├── src/
│   └── lib.rs              ← your library (pub API)
├── tests/
│   ├── common/
│   │   └── mod.rs          ← shared helpers (NOT a test binary)
│   ├── config_test.rs      ← integration tests for config
│   └── api_test.rs         ← integration tests for the full API
└── Cargo.toml
```

`tests/common/mod.rs` — shared fixtures:
```rust
// This module is NOT auto-discovered as a test binary.
// Test files explicitly declare: mod common;
use my_crate::{Config, validate_config};

pub fn test_config() -> Config {
    Config::new("test-host", 9999, 10)
}

pub fn assert_valid(c: &Config) {
    assert!(validate_config(c).is_ok(), "config should be valid: {:?}", c);
}
```

`tests/config_test.rs` — integration tests:
```rust
mod common;  // pulls in shared helpers

use my_crate::{Config, validate_config, parse_port, ConfigError};

#[test]
fn default_config_is_valid() {
    let cfg = Config::default();
    common::assert_valid(&cfg);
}

#[test]
fn empty_host_is_invalid() {
    let cfg = Config::new("", 80, 10);
    assert_eq!(validate_config(&cfg), Err(ConfigError::EmptyHost));
}

#[test]
fn parse_port_rejects_zero_and_overflow() {
    assert!(parse_port("0").is_err());
    assert!(parse_port("65536").is_err());
    assert!(parse_port("abc").is_err());
}
```

Key points:
- No `#[cfg(test)]` needed — the entire `tests/` tree is test-only by definition
- Each `tests/*.rs` file is compiled as a separate crate: `use my_crate::...` is required
- `tests/common/mod.rs` is shared — it's not treated as a test binary because it has no top-level `#[test]`
- Run a single file: `cargo test --test config_test`

## What This Unlocks

- **Public API confidence**: integration tests guarantee you haven't accidentally broken the external interface during refactors
- **Shared test fixtures**: `tests/common/mod.rs` provides reusable builders and assertion helpers across all integration test files
- **Per-scenario test binaries**: each `tests/*.rs` file compiles independently, so a panic in one doesn't abort others

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Integration tests | Separate executable linked against the library | `tests/` directory — each file is a separate crate |
| Shared helpers | Separate module compiled with tests | `tests/common/mod.rs` — referenced explicitly |
| API visibility | Module signature controls exports | `pub` keyword — `tests/` files can only see `pub` items |
| Running one file | Select by name in Dune | `cargo test --test filename` |
| Setup/teardown | `with_setup` or resource bracketing | No built-in; use helper functions that return fixtures |
