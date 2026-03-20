📖 **[View on hightechmind.io →](https://hightechmind.io/rust/754-testing-async-code)**

---

# 754-testing-async-code — Testing Async Code

## Problem Statement

Async code introduces new testing challenges: futures must be driven to completion, timeouts must be tested without real waits, and concurrent operations must be coordinated. In production Rust, `#[tokio::test]` provides a single-threaded or multi-threaded test runtime. This example uses threads and channels as a sync-compatible substitute to demonstrate the core patterns: driving async work to completion, testing timeouts, and injecting controllable fake implementations.

## Learning Outcomes

- Test thread-based concurrent code using channels as synchronization primitives
- Simulate timeout behavior with `recv_timeout` and verify timeout errors
- Use dependency injection to replace real async clients with controllable fakes
- Understand why `#[tokio::test]` is needed for real async code and how it relates to sync patterns
- Structure tests that verify both success and error paths through concurrent code

## Rust Application

`HttpClient` spawns a thread per request, simulating async IO. The test client uses `recv_timeout` to implement timeouts. `Response` carries `status` and `body`. Tests cover: successful 200 response, 404 response, timeout when the server does not respond, and correct body content. A `MockHttpClient` trait-based substitute is used for service-level tests to avoid thread spawning in fast unit tests.

## OCaml Approach

OCaml's `Lwt` and `Eio` (effect-based) runtimes require their own test runners. `Lwt_main.run` drives a promise to completion in tests. `Alcotest_lwt` provides `Alcotest_lwt.test_case` for async test cases. OCaml's `Mock_clock` from `Core_kernel` allows time manipulation in tests without real sleeps. The `eio` library's `Eio_mock` provides controllable IO for testing.

## Key Differences

1. **Runtime**: Rust uses `#[tokio::test]` or `#[async_std::test]` to run async tests; OCaml uses `Lwt_main.run (test_fn ())` or `Eio_main.run`.
2. **Timeout testing**: Rust's `tokio::time::pause/advance` enables time manipulation without real waits; OCaml's `Core.Time_ns` with mocked clock serves the same purpose.
3. **Mock ease**: Both languages use trait/module injection for mock HTTP clients; Rust's trait objects give runtime flexibility while OCaml's functor approach is compile-time.
4. **Cancellation**: Tokio provides structured cancellation via `CancellationToken`; OCaml/Lwt uses promise cancellation; this example uses thread-join timeouts.

## Exercises

1. Add a `retry` wrapper that retries a failed HTTP request up to N times with exponential backoff, and write tests that verify retry counts using a `CountingMockClient`.
2. Implement a `Circuit Breaker` that opens after 3 consecutive failures and write tests that verify the open/half-open/closed state transitions.
3. Write a test for concurrent requests using `thread::scope` to spawn 10 parallel requests and verify all responses are received correctly.
