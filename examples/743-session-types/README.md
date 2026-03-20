📖 **[View on hightechmind.io →](https://hightechmind.io/rust/743-session-types)**

---

# 743-session-types — Session Types
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Communication protocols have strict ordering requirements: you must connect before sending, send before receiving, and receive before closing. Violating the order causes protocol errors that are caught only at runtime, often in production. Session types encode the entire protocol as a sequence of type-level state transitions, making it impossible to call operations out of order. Pioneered in pi-calculus research, session types are used in the `session_types` Rust crate and Haskell's `sesh` library for typed channel communication.

## Learning Outcomes

- Model a request-response protocol as a four-state typestate: `Connected`, `RequestSent`, `ResponseReceived`, `Closed`
- Enforce protocol ordering: `send_request` only on `Connected`, `recv_response` only on `RequestSent`
- Collect a log of protocol events while maintaining typestate discipline
- Understand how session types generalize typestate to multi-party protocols
- See how consuming `self` in each step prevents protocol reuse or skipping

## Rust Application

`Session<State>` wraps an in-memory `Channel` and a `Vec<String>` log. `open_session()` returns `Session<Connected>`. `Session<Connected>::send_request` consumes the session and returns `Session<RequestSent>`. `Session<RequestSent>::recv_response` transitions to `Session<ResponseReceived>`. `Session<ResponseReceived>::close` consumes the session and returns the accumulated log. The `Channel` simulates an echo server: it echoes the request as the response.

## OCaml Approach

OCaml implements session types using continuation-passing style or GADTs. A channel `('send, 'recv) channel` carries phantom types for the send and receive types at each step. Libraries like `mpst-ocaml` implement multiparty session types for distributed systems. The continuation-passing approach threads the protocol through function types: `connect : unit -> (send_t -> recv_t -> close_t -> 'a) -> 'a`.

## Key Differences

1. **Expressiveness**: Full session types (as in `session_types` crate) encode the entire protocol as a type; Rust's typestate approach requires a new `Session<S>` struct per state.
2. **Multiparty**: OCaml's `mpst-ocaml` supports multiparty session types (multiple protocol participants); Rust's typestate handles only binary (two-party) protocols easily.
3. **Duality**: Session types have a dual type for the other end of the channel; Rust typestate typically only encodes one side of the protocol.
4. **Runtime**: Both have zero runtime overhead for the state tracking — protocol violations are caught entirely at compile time.

## Exercises

1. Extend the protocol to include an `Authenticated` state between `Connected` and `RequestSent`, requiring `authenticate(token: &str)` before any requests can be sent.
2. Implement a server-side dual session type `ServerSession<State>` that mirrors the client transitions in reverse order.
3. Write a typed RPC client that enforces the protocol `Connect → Auth → (Request → Response)*N → Disconnect` where N requests can be made before disconnecting.
