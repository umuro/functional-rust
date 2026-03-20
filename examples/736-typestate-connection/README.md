📖 **[View on hightechmind.io →](https://hightechmind.io/rust/736-typestate-connection)**

---

# 736-typestate-connection — Typestate Connection
**Difficulty:** ⭐⭐⭐  
**Category:** Functional Programming  


## Problem Statement

Network connection objects are the canonical example of runtime state misuse: calling `send()` before `connect()` or after `close()` is a logic error that only manifests at runtime. Production code adds defensive `if !self.connected { return Err(...) }` checks everywhere. The typestate pattern eliminates these checks: `send` and `recv` only exist on `TcpConn<Connected>`, so calling them on a disconnected connection is a compile error. Used in production in the `tokio-serial` and `embedded-hal` crates.

## Learning Outcomes

- Model TCP connection lifecycle as typestate: `Disconnected`, `Connecting`, `Connected`, `Closed`
- Enforce that `send` and `recv` are only callable in the `Connected` state
- Return `Result` from transition methods to handle IO errors without breaking the typestate invariant
- Track connection statistics (`bytes_sent`, `bytes_recv`) safely within the typed connection
- Understand how typestate composes with `Result`: `connect()` returns `Result<TcpConn<Connected>, _>`

## Rust Application

`TcpConn<Disconnected>` exposes only `new` and `connect`. `TcpConn<Connected>` exposes `send`, `recv`, and `close`. `TcpConn<Closed>` exposes no further operations. All transitions consume `self` and return the new state, preventing use-after-close. The `bytes_sent` and `bytes_recv` fields accumulate across send/recv calls; `close` returns final statistics. A real implementation would hold a `TcpStream` instead of tracking simulated state.

## OCaml Approach

OCaml models connection typestate using abstract types in separate modules. A `Disconnected.t` and `Connected.t` are distinct types exposed through module signatures. The `connect : Disconnected.t -> (Connected.t, exn) result` function enforces the transition. OCaml's `Lwt` and `Async` add monadic sequencing, making it natural to chain `connect >>= send >>= recv >>= close`.

## Key Differences

1. **Move semantics**: Rust's consuming `self` prevents double-use; OCaml's modules must use abstract types and hide constructors to achieve the same effect.
2. **Error handling**: Rust's `Result<TcpConn<Connected>, E>` combines typestate with error propagation naturally; OCaml uses `result` type or exceptions similarly.
3. **Async integration**: OCaml's `Lwt_unix` returns promise-wrapped connection types; Rust's `tokio::net::TcpStream` uses `async fn` with the same typestate ideas.
4. **Runtime cost**: Both approaches have zero runtime cost for the state tracking — no runtime enum, no branch.

## Exercises

1. Add a `Reconnecting` state between `Closed` and `Connected` with a `reconnect()` method that retries up to N times.
2. Implement a `TlsConn<State>` that wraps `TcpConn<Connected>` and adds a `tls_handshake()` transition to `TlsConn<Secured>` before allowing encrypted `send`/`recv`.
3. Write a generic `Pipeline<C: Connected>` that accepts any `Connected` connection type and sends a sequence of protocol messages, returning accumulated statistics.
