📖 **[View on hightechmind.io →](https://hightechmind.io/rust/747-test-fixtures)**

---

# 747-test-fixtures — Test Fixtures
**Difficulty:** ⭐  
**Category:** Functional Programming  



## Problem Statement

Tests that rely on shared mutable state are fragile: one test's teardown failure corrupts the next test's starting state. RAII-based test fixtures solve this: the fixture sets up state in a constructor and tears it down in `Drop`, guaranteeing cleanup even if the test panics. A `DatabaseFixture` that seeds test data and clears it on drop ensures every test starts from a known clean state, regardless of test order or failures.

## Learning Outcomes

- Implement RAII teardown using `Drop` for automatic test cleanup
- Use `OnceLock<Mutex<T>>` to share expensive initialization across tests in a suite
- Build a `FixtureBuilder` that creates fixtures with customizable seed data
- Understand the tension between test isolation (each test gets fresh state) and performance (shared setup)
- Write tests that use the fixture's `Drop` guarantee to test cleanup behavior itself

## Rust Application

`Database` provides `insert`, `get`, `delete`, and `count` operations. `TestFixture` wraps a `Database` seeded with known data; its `Drop` impl calls `database.clear()` to restore state. `OnceLock<Mutex<GlobalDb>>` provides a process-wide shared database for tests that need realistic initialization overhead amortized across the suite. The `FixtureBuilder` pattern allows each test to customize its starting data while sharing the teardown logic.

## OCaml Approach

OCaml's `Alcotest` framework provides `bracket : (unit -> 'a) -> ('a -> unit) -> ('a -> unit) -> unit` for setup/teardown pairs. `OUnit2` uses similar `bracket` combinators. Since OCaml lacks RAII, teardown is never guaranteed on exception — tests must use `try ... with` or the framework's bracket to ensure cleanup. Jane Street's `Async_kernel` provides `Deferred.bracket` for asynchronous test fixtures.

## Key Differences

1. **Cleanup guarantees**: Rust's `Drop` runs even on panic, guaranteeing cleanup; OCaml requires explicit `try ... finally` or framework brackets.
2. **Shared initialization**: Rust uses `OnceLock` for lazy singleton initialization; OCaml uses `lazy` values or `Lazy.force` for the same purpose.
3. **Parallel tests**: Rust tests run in parallel by default; shared `Mutex<Database>` must be used carefully to avoid test interference.
4. **Setup ergonomics**: Rust's builder pattern for fixtures is idiomatic; OCaml typically passes setup parameters as function arguments to avoid mutable state.

## Exercises

1. Extend `TestFixture` with a `checkpoint()` method that saves the current database state and a `restore_checkpoint()` that rolls back to it, enabling partial-state tests.
2. Implement a `ParallelFixture` that uses `Arc<Mutex<Database>>` and creates per-test copies of the shared state at the start of each test, ensuring full isolation.
3. Write a test that verifies the `Drop` cleanup actually runs by checking the database count before and after a fixture goes out of scope in a nested block.
