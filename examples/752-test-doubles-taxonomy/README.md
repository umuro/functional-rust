# 752: Stubs, Mocks, Fakes, Spies Taxonomy

**Difficulty:** 2  **Level:** Intermediate

The four kinds of test doubles — Stub, Fake, Mock, Spy — and when to use each in Rust using traits.

## The Problem This Solves

When testing code that depends on external systems (databases, loggers, HTTP clients, clocks), you replace those dependencies with **test doubles** — objects that stand in for the real thing during testing. But "mock" has been overloaded to mean everything; the precise taxonomy (Stub, Fake, Mock, Spy) helps you pick the right tool for each testing goal.

In Rust, all test doubles are implemented by defining a trait for the dependency, then writing multiple implementations: one for production, several for tests. The trait boundary is the key — it's what makes your code testable without changing its logic.

This pattern is ubiquitous in enterprise Rust: web handlers that accept `&dyn Database`, CLIs that accept `&dyn Clock`, event processors that accept `&dyn Queue`. Dependency injection via traits is how you make code testable without reaching for DI frameworks.

## The Intuition

The four test doubles serve different purposes:

- **Stub** (`NullLogger`): returns nothing / ignores everything. Use when you don't care about the dependency's behavior at all — just want the code to run.
- **Fake** (`InMemoryLogger`): a working but simplified implementation. Use when you need the dependency to actually store/process data, but not with real infrastructure.
- **Mock** (`MockLogger`): records calls and asserts on them. Use when you want to verify *what* was called, *how many times*, and *with what arguments*.
- **Spy** (`SpyLogger`): wraps a real implementation and adds call recording. Use when you need both real behavior *and* observability.

## How It Works in Rust

```rust
pub trait Logger {
    fn log(&self, message: &str);
    fn error(&self, message: &str);
    fn warn(&self, message: &str);
}

// 1. Stub: no-op implementation
pub struct NullLogger;
impl Logger for NullLogger {
    fn log(&self, _: &str) {}
    fn error(&self, _: &str) {}
    fn warn(&self, _: &str) {}
}

// 2. Fake: real logic, simplified storage
pub struct InMemoryLogger { logs: RefCell<Vec<String>>, errors: RefCell<Vec<String>> }
impl Logger for InMemoryLogger {
    fn log(&self, msg: &str) { self.logs.borrow_mut().push(msg.to_owned()); }
    // ...
}

// 3. Mock: records calls for assertion
pub struct MockLogger { calls: RefCell<Vec<LogCall>> }
impl MockLogger {
    pub fn assert_called_with(&self, level: &str, msg: &str) { /* ... */ }
    pub fn assert_call_count(&self, expected: usize) { /* ... */ }
}

// 4. Spy: delegates to inner + counts calls
pub struct SpyLogger<Inner: Logger> { inner: Inner, call_count: RefCell<usize> }
impl<I: Logger> Logger for SpyLogger<I> {
    fn log(&self, msg: &str) {
        *self.call_count.borrow_mut() += 1;
        self.inner.log(msg);  // real behavior preserved
    }
}

// Business logic accepts &dyn Logger — works with any double
pub fn process_items(items: &[i32], logger: &dyn Logger) -> (usize, usize) { ... }
```

`RefCell` enables interior mutability: the `Logger` trait takes `&self`, but the fake/mock/spy need to mutate their recorded state. `RefCell` provides runtime-checked `&mut` access through a shared reference — safe in single-threaded test code.

## What This Unlocks

- **Trait-based dependency injection** — accepting `&dyn Trait` instead of concrete types makes every function unit-testable without real infrastructure; it's zero-cost (monomorphization) in production when generics are used instead of trait objects.
- **`RefCell` for interior mutability in tests** — the standard pattern for test doubles that need to record state through `&self`; for multi-threaded tests, use `Mutex` instead.
- **Mock-less assertion style** — Rust doesn't have a `mockall`-style DSL built in, but `RefCell`-based mocks with `assert_called_with` helpers provide the same power with explicit, readable test code.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Test double mechanism | Module substitution or first-class functions | Trait implementations — compile-time checked |
| Interior mutability | Mutable references always explicit | `RefCell<T>` — runtime-checked `borrow_mut()` |
| Trait objects | First-class modules, functors | `&dyn Trait` (dynamic) or `impl Trait` (static) |
| Call recording | Imperative mutation in closures | `RefCell<Vec<LogCall>>` — standard pattern |
