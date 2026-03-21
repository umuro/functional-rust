📖 **[View on hightechmind.io →](https://hightechmind.io/rust/758-test-isolation-patterns)**

---

# 758-test-isolation-patterns — Test Isolation Patterns
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Tests that share mutable global state are non-deterministic when run in parallel: test A's changes leak into test B's state, causing intermittent failures that are painful to debug. The solution is test isolation: every test operates on its own independent state. Dependency injection, scoped state, and per-test instances replace global singletons. This is a fundamental principle of reliable test suites used in every professional codebase.

## Learning Outcomes

- Identify global state as a source of test pollution and flakiness
- Use dependency injection to replace global state with per-test instances
- Implement a `Counter` trait with `AtomicCounter` for isolated per-test counting
- Use `Arc<Mutex<T>>` for shared-but-isolated test state
- Understand how Rust's test runner parallelism exacerbates global state problems

## Rust Application

`increment_global()` demonstrates the anti-pattern: a `OnceLock<Mutex<u64>>` global counter that accumulates across tests. The `Counter` trait with `AtomicCounter` implementation replaces this: each test creates its own `AtomicCounter::new()`, ensuring complete isolation. A `UserService` that accepts `Arc<dyn Counter>` via dependency injection is testable without any global state. `ThreadLocalCounter` using `thread_local!` provides another isolation strategy.

## OCaml Approach

OCaml's immutable-by-default style naturally avoids most global state issues. Mutable state uses `ref` cells, which tests can scope locally. For shared mutable state across OCaml threads, `Mutex.t` wraps a `ref`. The `Alcotest` framework runs tests sequentially, reducing (but not eliminating) global state hazards. OCaml's effect system (5.0+) provides another mechanism for scoped state injection.

## Key Differences

1. **Default behavior**: Rust tests run in parallel by default (multiple threads); OCaml's `Alcotest` is sequential, making global state less hazardous.
2. **Global statics**: Rust's `static` variables with `OnceLock` create permanent global state; OCaml's module-level `ref` cells are equivalent.
3. **Isolation mechanism**: Rust uses per-test struct instances; OCaml uses local `let` bindings for isolated state.
4. **Thread safety**: Rust's type system (`Send`, `Sync`) prevents accidental sharing of non-thread-safe state; OCaml's runtime lock (before 5.0) serialized all threads.

## Exercises

1. Write a `TestDatabase` that wraps a `HashMap` and is created fresh per test, then refactor `UserService` to accept `Box<dyn Database>` for full isolation.
2. Implement a `test_serial!` macro that serializes specific tests using a process-wide `Mutex` for tests that genuinely cannot avoid shared resources (e.g., a real file path).
3. Build a `Sandbox` type that encapsulates a `TempDir` + `AtomicCounter` + `MockEmailSender` and provides a single entry point for all test dependencies in a service test.
