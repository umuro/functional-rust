📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1025-network-errors)**

---

# 1025-network-errors — Network Error Classification
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Network errors are not all equal: a DNS failure means the host does not exist (do not retry), a timeout might be transient (retry with backoff), a TLS error means a misconfiguration (alert the operator), and an HTTP 5xx means the server is overloaded (retry). Treating all network errors the same leads to either excessive retries or insufficient resilience.

Structuring network errors as an enum with specific variants enables precise recovery strategies. This is the approach taken by the `reqwest`, `hyper`, and `tonic` crates in the Rust ecosystem.

## Learning Outcomes

- Design a `NetError` enum that captures the full taxonomy of network failures
- Implement `is_retryable` and `is_client_error` classification methods
- Use the error enum to drive retry logic
- Implement `Display` for human-readable error messages
- Understand how production HTTP clients structure their error types

## Rust Application

`src/lib.rs` defines `NetError` with variants: `Timeout`, `ConnectionRefused`, `DnsResolutionFailed`, `TlsError`, and `HttpError`. The `is_retryable` method returns `true` for transient errors (timeouts, 5xx responses) and `false` for permanent ones (DNS failures, TLS errors). `is_client_error` identifies 4xx responses.

These classification methods are the key value of typed errors over string-based errors: you can write a generic retry loop that calls `is_retryable()` without knowing the specific failure mode.

## OCaml Approach

OCaml's `Cohttp` and `Eio` libraries use exception hierarchies for network errors. A typed approach mirrors Rust:

```ocaml
type net_error =
  | Timeout of float
  | ConnectionRefused of string
  | DnsError of string
  | HttpError of { status: int; body: string }

let is_retryable = function
  | Timeout _ | ConnectionRefused _ -> true
  | DnsError _ -> false
  | HttpError { status; _ } -> status >= 500
```

## Key Differences

1. **Method dispatch**: Rust's `is_retryable` is a method on `NetError`; OCaml uses a top-level function pattern-matching on the variant.
2. **Record fields in variants**: Rust supports named fields in enum variants (`HttpError { status, body }`); OCaml uses inline record syntax `{ status: int; body: string }`.
3. **Display trait**: Rust requires implementing `fmt::Display` explicitly; OCaml typically uses `Format.fprintf` or derives `show` via ppx.
4. **Production libraries**: Rust's `reqwest::Error` exposes `is_timeout()`, `is_connect()`, etc. as methods; OCaml HTTP libraries vary in their error modelling.

## Exercises

1. Implement a `retry<F>(f: F, max_attempts: u32) -> Result<Response, NetError>` function that retries only when `is_retryable()` returns true.
2. Add a `CircuitBreakerError` variant with a `reset_at: Instant` field, and add it to the `is_retryable` logic.
3. Write a function that maps `NetError` to HTTP response status codes: 504 for timeout, 502 for connection refused, 400 for client errors.
