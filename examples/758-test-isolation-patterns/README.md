📖 **[View on hightechmind.io →](https://hightechmind.io/rust/758-test-isolation-patterns)**

---

# 758: Test Isolation: Avoiding Shared Mutable State

**Difficulty:** 3  **Level:** Advanced

`cargo test` runs tests in parallel threads. Shared mutable globals make tests flaky and order-dependent. Five patterns for writing isolated, deterministic tests.

## The Problem This Solves

The moment you introduce a global mutable variable shared across tests — a counter, a registry, an environment variable — you invite non-determinism. Two tests running in parallel might both read zero, both increment, and one sees an unexpected value. Or one test sets an environment variable that a different test reads, getting a wrong result. The failures are intermittent, hard to reproduce, and nearly impossible to debug without understanding the threading model.

`cargo test` runs test functions in parallel threads by default (one thread per test). Any global state is shared across all of them simultaneously. The solution is to design tests that don't share state: create fresh instances per test, use `OnceLock` for read-only shared data, wrap mutable shared data in `Mutex`, and use RAII guards to restore environment variables on drop.

## The Intuition

The golden rule: a test that can't run independently is a broken test. Each `#[test]` function should set up its own state, exercise the code, and assert — with no dependency on what other tests do or don't do, and no impact on their execution.

When you genuinely need shared state (expensive initialization, shared read-only data), use `OnceLock` — it runs the initializer exactly once across all threads. When you need shared *mutable* state, wrap it in `Mutex` and use unique keys to avoid inter-test conflicts.

## How It Works in Rust

**Pattern 1 — Per-test instance (preferred):**
```rust
#[test]
fn counter_increments_from_zero() {
    let mut c = Counter::new(0);  // fresh, no sharing
    c.increment();
    assert_eq!(c.value(), 1);
}
```
No globals, no parallelism issues. Every test starts from a known state.

**Pattern 2 — Read-only shared data via `OnceLock`:**
```rust
static SHARED_DATA: OnceLock<Vec<u32>> = OnceLock::new();

fn shared_data() -> &'static [u32] {
    SHARED_DATA.get_or_init(|| (0u32..=100).map(|i| i * i).collect())
}

#[test]
fn shared_data_index_3() {
    assert_eq!(shared_data()[3], 9);
}
```
`get_or_init` is thread-safe: the closure runs once, all tests see the same `Vec`. No mutex needed because it's never mutated after initialization.

**Pattern 3 — Mutex-guarded mutable shared state:**
```rust
static GLOBAL_REGISTRY: OnceLock<Mutex<Registry>> = OnceLock::new();

fn registry() -> &'static Mutex<Registry> {
    GLOBAL_REGISTRY.get_or_init(|| Mutex::new(Registry::new()))
}

#[test]
fn registry_register_and_contains() {
    // Use thread-unique key to avoid inter-test conflicts
    let unique = format!("svc_{}", std::thread::current().id().as_u64().get());
    registry().lock().unwrap().register(&unique);
    assert!(registry().lock().unwrap().contains(&unique));
}
```
Each test uses a unique key — parallel tests don't interfere with each other's entries.

**Pattern 4 — Environment variable guard (RAII restore):**
```rust
struct EnvGuard { key: &'static str, original: Option<String> }

impl EnvGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let original = std::env::var(key).ok();
        unsafe { std::env::set_var(key, value); }
        EnvGuard { key, original }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.original {
            Some(v) => unsafe { std::env::set_var(self.key, v); },
            None    => unsafe { std::env::remove_var(self.key); },
        }
    }
}
```
The variable is restored when `_guard` goes out of scope — even on panic. Note: `set_var` is inherently racy in multi-threaded tests; serialize env-mutating tests with a separate `Mutex` or use `--test-threads=1`.

**Pattern 5 — Independent instances proving isolation:**
```rust
#[test]
fn registry_instances_are_isolated() {
    let mut r1 = Registry::new();
    let mut r2 = Registry::new();
    r1.register("alice");
    r2.register("bob");
    assert!(!r1.contains("bob"));   // r1 doesn't see r2's entries
    assert!(!r2.contains("alice")); // r2 doesn't see r1's entries
}
```

## What This Unlocks

- **Deterministic CI** — tests pass or fail based on code, not on thread scheduling.
- **Safe parallelism** — `cargo test` can use all CPU cores without flakiness.
- **RAII test cleanup** — guard objects restore state on drop, even when tests panic.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Test runner | `alcotest` / `ounit` (sequential by default) | `cargo test` (parallel threads by default) |
| One-time init | `Lazy.t` | `OnceLock<T>` (thread-safe, `std`) |
| Shared mutable global | `ref` (single-threaded) | `OnceLock<Mutex<T>>` |
| RAII cleanup in test | Manual `teardown` | `Drop` impl on guard struct |
