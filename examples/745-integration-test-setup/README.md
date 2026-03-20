📖 **[View on hightechmind.io →](https://hightechmind.io/rust/745-integration-test-setup)**

---

# 745-integration-test-setup — Integration Test Setup
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Integration tests verify that components work together through their public APIs, not just in isolation. In Rust, integration tests live in the `tests/` directory and link against the compiled library — they cannot access private internals. This forces a clean API boundary. Setting up shared state (database connections, server sockets, test data) across multiple integration tests requires patterns for initialization, teardown, and parallelism-safe shared fixtures.

## Learning Outcomes

- Understand the structural difference between `tests/` integration tests and `#[cfg(test)]` unit tests
- Create a `Config` type with a `validate` method that integration tests exercise through the public API
- Build shared test helpers that initialize complex state once and reuse it across tests
- Use `OnceLock<Mutex<T>>` for global test state that is initialized once per test run
- Write integration tests that test complete request-response cycles through a service

## Rust Application

`Config` models a service configuration with `host`, `port`, and `max_connections`. `validate()` checks port ranges and empty hosts. Integration tests in `tests/` create configs, validate them, and verify error messages through the public API without accessing private fields. A `TestServer` struct wraps the config and simulates starting a service, testing that the complete initialization sequence works correctly.

## OCaml Approach

OCaml integration tests typically live in a `test/` directory with separate executables per test suite. `Alcotest` organizes tests as groups with setup/teardown via `before_test_all` and `after_test_all` hooks. The `OUnit2` framework provides `bracket` for RAII-style setup/teardown around individual tests. Shared state is usually passed explicitly as a function argument rather than using global mutable state.

## Key Differences

1. **Directory convention**: Rust uses `tests/` for integration tests with special cargo handling; OCaml uses `test/` or `tests/` as a convention without special build-tool support.
2. **Visibility**: Rust integration tests can only access public items; OCaml tests can use `open Module` to access any exported function including those not in the public `.mli`.
3. **Shared setup**: Rust uses `OnceLock`/`LazyLock` for one-time test initialization; OCaml's `Alcotest` supports explicit `before_all`/`after_all` hooks.
4. **Parallelism**: Rust integration tests run in parallel by default; OCaml's test runners are typically sequential.

## Exercises

1. Write an integration test that creates a `Config`, validates it, starts a `TestServer`, sends a request, and verifies the response — covering the full request lifecycle.
2. Add a test that verifies the `ConfigError` display strings are user-friendly (not just assert they equal some specific string, but check they contain key words like "invalid", "empty").
3. Implement a `TestHarness` that shares a single validated `Config` across all tests in the suite using `OnceLock`, avoiding redundant initialization.
