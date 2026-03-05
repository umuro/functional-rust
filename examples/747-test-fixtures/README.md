📖 **[View on hightechmind.io →](https://hightechmind.io/rust/747-test-fixtures)**

---

# 747: Test Fixtures: Setup/Teardown, Shared State

**Difficulty:** 2  **Level:** Intermediate

Use RAII structs for automatic test teardown and `OnceLock` for shared read-only state across tests.

## The Problem This Solves

Many tests need setup: create a database, write a config file, seed some state. They also need cleanup: remove those resources afterward, even if the test panics. In languages with `setUp`/`tearDown` lifecycle hooks, teardown is skipped on panic. In Rust, you get something better: RAII. When a fixture struct goes out of scope — including on panic — `Drop` runs. No exceptions.

This matters for test isolation: each test gets a fresh fixture, side effects don't leak between tests, and filesystem/network resources are always cleaned up. It also matters for shared resources: some expensive initialization (parsing a large file, building a lookup table) should happen once and be read by many tests. `OnceLock` provides thread-safe lazy initialization with no locks needed after the first access.

These patterns compose naturally with Rust's ownership model — the fixture owns its resources, the test owns the fixture, and when the test ends, cleanup is guaranteed by the type system.

## The Intuition

A **fixture** is a struct that owns test resources, initializes them in `new()`, and cleans them up in `Drop::drop()`. Tests create a fixture at the start, use it, and the cleanup is automatic. For shared read-only data, `OnceLock<T>` is a global that initializes exactly once on first access and then hands out `&'static T` references with no runtime cost.

## How It Works in Rust

```rust
// RAII fixture: teardown guaranteed even on panic
struct DatabaseFixture {
    pub db: Database,
    name: &'static str,
}

impl DatabaseFixture {
    fn new(name: &'static str) -> Self {
        let mut db = Database::new();
        db.insert("user:1", "Alice");
        db.insert("user:2", "Bob");
        println!("[fixture:{}] Set up", name);
        DatabaseFixture { db, name }
    }
}

impl Drop for DatabaseFixture {
    fn drop(&mut self) {
        // Runs even if the test panicked
        println!("[fixture:{}] Torn down", self.name);
    }
}

#[test]
fn test_lookup() {
    let f = DatabaseFixture::new("lookup");
    assert_eq!(f.db.get("user:1"), Some("Alice"));
    // f dropped here → Drop::drop() called
}

// Shared read-only state: initialized once, read by many tests
static SHARED_DATA: OnceLock<Vec<i32>> = OnceLock::new();

fn shared_data() -> &'static [i32] {
    SHARED_DATA.get_or_init(|| (1..=100).collect())
}

#[test]
fn test_sum() {
    assert_eq!(shared_data().iter().sum::<i32>(), 5050);
}
```

For shared mutable state across tests, `OnceLock<Mutex<T>>` works — but prefer per-test isolation when possible; test ordering is not guaranteed. The `Mutex` pattern is shown in the example for completeness.

## What This Unlocks

- **Drop as teardown** — any cleanup that must happen — temp file deletion, connection closing, lock release — belongs in `Drop`, not in "cleanup" helper functions that callers might forget to call.
- **RAII everywhere** — the fixture pattern extends to temporary directories (example 756), mock servers, database transactions, and any scoped resource.
- **`OnceLock` for expensive shared setup** — replaces `lazy_static!` (third-party) with a standard-library primitive; perfect for test helpers that parse test data, build lookup tables, or compile regex patterns.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Setup/teardown | `OUnit` `setup`/`teardown` callbacks | `Drop` — guaranteed by the type system |
| Shared state | Module-level `let` (mutable globals are unsafe) | `OnceLock<T>` — thread-safe lazy init |
| Test isolation | Functional purity or manual reset | Per-test fixture structs, each owning fresh state |
| Panic-safe cleanup | Try/finally in tests | `Drop` runs on panic — no special handling needed |
