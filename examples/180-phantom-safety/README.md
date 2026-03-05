# 180: PhantomData for API Safety

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Use phantom types to track resource lifecycle states in the type system — so calling `query()` on a closed connection or `write()` on an unopened file is a compile-time error.

## The Problem This Solves

Resource APIs have lifecycles: a connection must be opened before you query it, a transaction must be started before you commit it, a file must be opened before you read it. The typical Rust solution is to check at runtime — `if !self.is_open { return Err(...) }` — and return `Result` everywhere. This works, but it means every caller must handle an error that the type system could have prevented entirely.

The subtler problem is documentation and discoverability. When a function signature says `fn query(&self, sql: &str) -> Result<Rows, Error>`, there are two reasons it can fail: bad SQL, or calling it in the wrong state. Those are completely different problems. State violations are programming errors, not recoverable runtime errors. They deserve a compile-time error, not an `Err` variant.

The **type-state pattern** using phantom types fixes this: `Connection<Open>` and `Connection<Closed>` are different types. `query` only exists on `Connection<Open>`. The compiler won't let you call it on `Connection<Closed>`. The lifecycle is enforced statically, with zero runtime cost — `PhantomData` takes no memory.

## The Intuition

A key fob for a rental car. When you unlock the car, you hold a `Car<Unlocked>` object. When you lock it, you hold `Car<Locked>`. The `start_engine` method only compiles with a `Car<Unlocked>` — you can't even try to start a locked car in this API. The distinction between locked and unlocked is in the type, not a runtime flag inside the car.

The magic is that `PhantomData<S>` is a zero-sized field. You pay nothing at runtime — no extra byte in the struct, no branch in generated code. The state `S` is erased completely at runtime; it only exists during compilation, where it enforces your invariants.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// State markers — zero-sized, never instantiated
struct Closed;
struct Open;

struct Connection<S> {
    addr: String,
    // PhantomData<S> stores the state in the TYPE, not in memory
    _state: PhantomData<S>,
}

// Constructors always start in the right state
impl Connection<Closed> {
    fn new(addr: &str) -> Connection<Closed> {
        Connection { addr: addr.to_string(), _state: PhantomData }
    }

    // open() consumes the Closed connection, returns an Open one
    fn open(self) -> Result<Connection<Open>, String> {
        println!("Connecting to {}", self.addr);
        Ok(Connection { addr: self.addr, _state: PhantomData })
    }
}

// query() ONLY exists on Connection<Open>
impl Connection<Open> {
    fn query(&self, sql: &str) -> Vec<String> {
        vec![format!("result of: {}", sql)]
    }

    // close() consumes the Open connection — it's gone from the type system
    fn close(self) -> Connection<Closed> {
        Connection { addr: self.addr, _state: PhantomData }
    }
}

// Correct usage:
let conn = Connection::new("localhost:5432");  // Connection<Closed>
let conn = conn.open()?;                        // Connection<Open>
let rows = conn.query("SELECT * FROM users");   // ✓ compiles
let conn = conn.close();                        // Connection<Closed>

// This fails to compile:
// let conn = Connection::new("localhost:5432"); // Connection<Closed>
// conn.query("SELECT 1");                       // error: method not found in `Connection<Closed>`
```

Multi-state transitions work the same way — `Connection<Disconnected> -> Connection<Connected> -> Connection<InTransaction>` — with each state having exactly the methods that make sense for it.

## What This Unlocks

- **Database transaction safety** — `start_transaction()` returns `Connection<InTransaction>`; `commit()` and `rollback()` only exist on that type; double-commit is impossible.
- **File handle safety** — `File<Unopened>` can only call `open()`, `File<Open>` can read/write, `File<Closed>` exists for documentation completeness; the OS-level error is promoted to a type error.
- **Protocol state machines** — TLS handshake stages, OAuth flow steps, any protocol where out-of-order operations are bugs; model the protocol as types and get correctness for free.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mechanism | GADTs or abstract module types that hide the state | `PhantomData<S>` + separate `impl` blocks per state |
| State transitions | Return new tagged value `{ conn with state = Open }` | Methods consume `self` by value and return new phantom-typed struct |
| Compile-time enforcement | Yes — module signature or GADT prevents calling wrong operations | Yes — method simply doesn't exist in the wrong `impl` block |
| Move semantics | Values can be reused (OCaml is GC'd) | `self` consumed by value; old state literally unreachable after transition |
| Runtime cost | None | None — `PhantomData` is zero-sized, erased at compile time |
