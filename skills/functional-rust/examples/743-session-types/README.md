# 743: Session Types: Protocol Safety via Types

**Difficulty:** 4  **Level:** Expert

Encode a communication protocol as type parameters — calling methods out of order is a compile error, not a runtime error.

## The Problem This Solves

Network protocols have strict ordering requirements: you must connect before sending, send before receiving, receive before closing. HTTP/1.1 requires a request before a response. A database transaction must begin before executing statements and commit/rollback before disconnecting.

Violating these orderings causes subtle bugs: using a half-closed connection, reading from a socket that hasn't sent a request, committing a transaction that was never opened. These bugs are hard to catch with tests because the code *looks* correct — it just calls methods in the wrong order.

Session types solve this by making the *protocol* part of the type system. `Session<Connected>` has a `send_request` method but no `receive_response` method. `Session<RequestSent>` has `receive_response` but not `send_request`. The only way to get a `Session<RequestSent>` is to call `send_request` on a `Session<Connected>`. Wrong order = compile error.

## The Intuition

This is the typestate pattern (example 734) applied to a multi-step communication protocol. Where typestate typically has a few simple states, session types handle richer protocols: sequences, choices, recursion. The key insight is the same: encode the protocol as types, and transitions as methods that consume the current state and return the next.

The academic formalization (process calculi, linear types) is complex, but the practical Rust pattern is approachable: one `impl` block per protocol state, methods take `self` (consuming the state), and each method returns the next state.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// Protocol state markers — zero-sized types
pub struct Connected;
pub struct RequestSent;
pub struct ResponseReceived;
pub struct Closed;

// Session carries a communication channel + protocol state in the type
pub struct Session<State> {
    channel: Channel,
    log: Vec<String>,
    _state: PhantomData<State>,
}

/// Entry point — always starts in Connected state
pub fn open_session() -> Session<Connected> {
    Session { channel: Channel::new(), log: vec![], _state: PhantomData }
}

// Connected → RequestSent (only after connecting)
impl Session<Connected> {
    pub fn send_request(mut self, method: &str, path: &str) -> Session<RequestSent> {
        let msg = format!("{} {}", method, path);
        self.channel.send(msg.into_bytes());
        Session { channel: self.channel, log: self.log, _state: PhantomData }
    }
    // No receive_response, no close — only send_request is valid here
}

// RequestSent → ResponseReceived (only after sending)
impl Session<RequestSent> {
    pub fn receive_response(mut self) -> (String, Session<ResponseReceived>) {
        let data = self.channel.recv().expect("no response");
        let response = String::from_utf8_lossy(&data).into_owned();
        (response, Session { channel: self.channel, log: self.log, _state: PhantomData })
    }
    // No send_request, no close — must receive before anything else
}

// ResponseReceived → Closed or Connected (protocol complete — can close or reuse)
impl Session<ResponseReceived> {
    pub fn close(self) -> Vec<String> {
        self.log   // return the audit log
    }
    // Can add: pub fn send_request(self) -> Session<RequestSent> for pipelining
}

// The correct usage — the protocol sequence
let session = open_session();
let session = session.send_request("GET", "/api/data");
let (response, session) = session.receive_response();
let log = session.close();

// These would NOT compile:
// open_session().receive_response();     // ERROR: no method on Session<Connected>
// session.send_request("POST", "/");     // ERROR: session is already consumed
// open_session().close();                // ERROR: no method on Session<Connected>
```

State transitions via `self`:
```rust
// Each step returns a DIFFERENT type — the previous type is consumed and gone
//         Session<Connected>       ─── send_request ───►  Session<RequestSent>
//         Session<RequestSent>     ─── receive_response ► (String, Session<ResponseReceived>)
//         Session<ResponseReceived> ── close ────────────►  Vec<String>
```

Testing session types:

```rust
#[test]
fn full_protocol_cycle_compiles() {
    let s = open_session();
    let s = s.send_request("GET", "/");
    let (resp, s) = s.receive_response();
    let _ = s.close();
    assert!(resp.contains("RESP:"));
}

// This test CANNOT EXIST — it doesn't compile:
// #[test]
// fn skip_send_and_receive() {
//     open_session().receive_response();  // compile error
// }
```

Key points:
- `self` (not `&self`) — each method *moves* the session, making reuse of the old state impossible
- Shadowing `let session = session.send_request(...)` is idiomatic — same variable name, new type
- State markers are never instantiated by users — they only appear in type positions
- The approach works for any protocol: database transactions, streaming protocols, two-phase commit
- Unlike runtime state machines, there's no match on state and no impossible-state handling code

## What This Unlocks

- **Correct-by-construction APIs**: HTTP clients, database connections, streaming encoders — users of your API literally cannot call methods in the wrong order
- **Audit logs and resource cleanup guaranteed**: the only way to finish is to call `close()`, which can enforce that cleanup always happens (no `Session<RequestSent>` left open)
- **Foundation for linear types**: session types are an application of linear logic — each capability is used exactly once, which is also how Rust's ownership tracks resource usage

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Session types | First-class support in `session_types` crate / research | Typestate pattern with `PhantomData` |
| State marker | Phantom type parameter | `struct Connected;` zero-sized struct |
| State consumption | Functional: return new state, discard old | Move semantics: `self` consumed, new type returned |
| Protocol enforcement | GADTs or phantom types | Generic struct + per-state `impl` block |
| Compile-time protocol | Type-checked sequence via GADT | Method availability enforced by type |
| Runtime overhead | Zero with phantom types | Zero — markers and `PhantomData` are erased |
