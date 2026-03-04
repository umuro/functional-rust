# 736: Typestate Connection

**Difficulty:** 4  **Level:** Expert

Model a TCP connection lifecycle as a typestate machine — `send()` and `recv()` exist only on `TcpConn<Connected>`, `connect()` only on `TcpConn<Disconnected>` — making invalid usage a compile error.

## The Problem This Solves

Network connections have a strict lifecycle: connect before send, don't send after close, don't connect twice. Runtime state machines enforce this with `match self.state { ... }` guards and `Result` errors for invalid transitions. These guards must be written for every method, can be forgotten, and only fail at runtime — when your test suite is incomplete, they fail in production.

API consumers face the same problem from the outside: nothing in the type signature of `fn send(&mut self, data: &[u8])` tells you that the connection must be in a connected state. The precondition is invisible until you read the docs, get a runtime error, or study the implementation.

The typestate pattern makes these constraints visible and enforced in the type signature. `send()` is defined on `TcpConn<Connected>`, not on `TcpConn<Disconnected>` or `TcpConn<Closed>`. Call it at the wrong time and the compiler tells you immediately. The documentation *is* the type.

## The Intuition

The state lives in the type, not in a field. `TcpConn<Disconnected>` and `TcpConn<Connected>` are different Rust types — they have different methods and cannot be used interchangeably. The state parameter is a `PhantomData<State>` — zero bytes, purely a type-level annotation.

Transitions *consume* the connection and return a new one with a different state type. `conn.connect()` takes `TcpConn<Disconnected>` by value and returns `Result<TcpConn<Connected>, String>`. After `.connect()`, the original `conn` variable is moved — you can't accidentally use the disconnected version. Rust's ownership system enforces single-state use automatically.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// State markers — zero-sized, carry no data
pub struct Disconnected;
pub struct Connected;
pub struct Closed;

pub struct TcpConn<State> {
    host: String,
    port: u16,
    bytes_sent: usize,
    bytes_recv: usize,
    _state: PhantomData<State>,  // zero bytes; tracks state in the type
}

impl TcpConn<Disconnected> {
    pub fn new(host: impl Into<String>, port: u16) -> Self { /* ... */ }

    // Consuming transition: Disconnected → Connected
    pub fn connect(self) -> Result<TcpConn<Connected>, String> {
        println!("Connecting to {}:{}", self.host, self.port);
        Ok(TcpConn { _state: PhantomData, ..self })
    }
    // No send(), recv(), or close() here — Disconnected can't do those
}

impl TcpConn<Connected> {
    // send() only on Connected — consumes self, returns self (same state)
    pub fn send(mut self, data: &[u8]) -> Result<TcpConn<Connected>, String> {
        self.bytes_sent += data.len();
        Ok(self)
    }

    pub fn recv(mut self) -> Result<(Vec<u8>, TcpConn<Connected>), String> {
        let data = b"HTTP/1.1 200 OK\r\n".to_vec();
        self.bytes_recv += data.len();
        Ok((data, self))
    }

    // Consuming transition: Connected → Closed
    pub fn close(self) -> TcpConn<Closed> {
        TcpConn { _state: PhantomData, ..self }
    }
}

impl TcpConn<Closed> {
    pub fn bytes_sent(&self) -> usize { self.bytes_sent }
    pub fn bytes_recv(&self) -> usize { self.bytes_recv }
    // No send(), recv(), or connect() — Closed is terminal
}

// ── Valid usage ────────────────────────────────────────────────────────────────
let conn = TcpConn::<Disconnected>::new("example.com", 80);
let conn = conn.connect()?;
let conn = conn.send(b"GET / HTTP/1.1\r\n")?;
let (response, conn) = conn.recv()?;
let closed = conn.close();

// ── Compile errors for invalid usage ──────────────────────────────────────────
// closed.send(b"data");      // error: no method `send` on TcpConn<Closed>
// closed.connect();          // error: no method `connect` on TcpConn<Closed>
// let disc = TcpConn::<Disconnected>::new("x", 80);
// disc.send(b"data");        // error: no method `send` on TcpConn<Disconnected>
```

## What This Unlocks

- **Protocol enforcement** — any multi-step protocol (TLS handshake, HTTP pipeline, SMTP sequence) can be modelled so wrong-order calls are compile errors.
- **Self-documenting APIs** — function signatures like `fn process(conn: TcpConn<Connected>)` communicate preconditions without docs; the type is the contract.
- **Exhaustive state coverage in tests** — the type system forces you to handle the return type of each transition; you can't silently ignore that a connection might fail to connect.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| State encoding | Variant in a sum type (runtime); GADT for compile-time | Zero-sized phantom type parameter — zero runtime cost |
| Invalid operation | Runtime exception or `Result` return | Compile error — method doesn't exist on wrong type |
| State transition | Function returning new state value | Consuming method returning new generic instantiation |
| Ownership of state | Shared via ref-counting or explicit passing | Moved — original unavailable after transition |
| Connection pool | `('a, connected) conn` lifetime approach | `Vec<TcpConn<Connected>>` — only connected conns |
