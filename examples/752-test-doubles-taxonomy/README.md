📖 **[View on hightechmind.io →](https://hightechmind.io/rust/752-test-doubles-taxonomy)**

---

# 752-test-doubles-taxonomy — Test Doubles Taxonomy

## Problem Statement

Gerard Meszaros coined the term "test doubles" in 2007 to categorize the different ways to replace real dependencies in tests. Confusingly, many people call all test doubles "mocks." The taxonomy — Stub, Spy, Mock, Fake, Dummy — has precise meanings that guide which technique to use. Using the wrong double leads to tests that are either too coupled to implementation details (mocks everywhere) or too permissive (stubs that hide bugs). This example implements all five categories for a `Logger` dependency.

## Learning Outcomes

- Implement a `NullLogger` stub that silently discards all log calls
- Build a `SpyLogger` that records all calls for later assertion
- Create a `MockLogger` that has pre-configured expectations and verifies them on drop
- Implement a `FakeLogger` that is a real working logger (writes to a Vec instead of a file)
- Know when to use each double: Dummy (don't care), Stub (canned return), Spy (verify calls), Mock (verify interactions), Fake (real implementation)

## Rust Application

`Logger` trait has `log`, `error`, and `warn` methods. `NullLogger` discards everything (stub/dummy). `SpyLogger` uses `RefCell<Vec<String>>` to record calls, exposing `log_count()`, `error_count()`, and `contains()` for assertions. `MockLogger` records calls and provides `assert_called_with` for interaction verification. `FakeLogger` writes to an in-memory `Vec<String>` — it's a real logger without the file system. Tests demonstrate when each type is appropriate.

## OCaml Approach

OCaml uses module types and functors for the same purpose. A `NULL_LOGGER` module discards calls (stub). A spy implementation uses `Queue.t ref` to accumulate calls. OCaml's `Alcotest` checks are made after the function under test returns, inspecting the recorded calls. The `ppx_mock` package auto-generates mock implementations from module signatures.

## Key Differences

1. **Drop-based verification**: Rust's `MockLogger` can verify expectations in `Drop` when the mock goes out of scope — OCaml has no automatic cleanup hook.
2. **Interior mutability**: Rust needs `RefCell` for mutable spy state accessed via `&self`; OCaml uses `ref` cells naturally.
3. **Expectation DSL**: Rust's `mockall` provides a rich `.expect().times(2).returning(...)` DSL; OCaml has no equivalent mature library.
4. **Fake implementations**: Both languages implement fakes as real implementations against a test backend (in-memory vs file); Rust's approach is structurally identical to OCaml's.

## Exercises

1. Implement a `ThrottledLogger` fake that is a real logger but rate-limits to N messages per second, and write tests that verify the throttling behavior.
2. Extend `MockLogger` to support `expect_log("message")` that asserts a specific log message was recorded, and `expect_no_errors()` that asserts no `error()` calls occurred.
3. Build a `CompositeLogger` that forwards to multiple loggers simultaneously, and write a test using a `SpyLogger` + `NullLogger` to verify all messages reach both.
