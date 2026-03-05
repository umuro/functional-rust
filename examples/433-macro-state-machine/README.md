📖 **[View on hightechmind.io →](https://hightechmind.io/rust/433-macro-state-machine)**

---

# 433: State Machine via Macro

**Difficulty:** 4  **Level:** Expert

Generate typestate state machine boilerplate — states as zero-sized types, transitions as consuming methods — so that invalid sequences are compile errors, not runtime panics.

## The Problem This Solves

Protocol implementations — network connections, file handles, authentication flows, payment processors — have strict sequencing rules. You can only `send()` after `connect()`. You can only `commit()` inside a transaction. Calling methods out of order is a logic error that should be caught immediately, not at runtime in production when a user hits an unexpected state.

Encoding these rules as runtime enums plus guards (`if self.state != State::Connected { panic!() }`) is fragile: the guard is easy to forget, every method has boilerplate, and the compiler won't help you find callers that got the sequence wrong.

The typestate pattern moves state into the type system: each state is a zero-sized type, and methods are only defined on the correct state. Call `send()` on a `Connection<Disconnected>` and the compiler refuses. The compiler *is* your state machine validator.

Implementing this by hand requires: state structs, a generic struct, `PhantomData`, and an `impl` block per state. A state machine macro generates all of this from a concise declaration.

## The Intuition

Each state (`Disconnected`, `Connected`, `Authenticated`) is a zero-sized struct — it carries no data, costs nothing at runtime. The connection struct `Connection<S>` is generic over state `S`. Methods exist only on specific `Connection<ConcreteState>` types:

```
Connection<Disconnected>  →  .connect()  →  Connection<Connected>
Connection<Connected>     →  .authenticate()  →  Connection<Authenticated>
Connection<Authenticated> →  .send()  →  same state
Connection<Authenticated> →  .disconnect()  →  Connection<Closed>
```

Each transition *consumes* `self` and returns a new typed value. Once consumed, you can't call the old state's methods — the original variable is moved. Impossible sequences are literally unrepresentable.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// State marker types — zero bytes at runtime
struct Disconnected;
struct Connected;
struct Authenticated;
struct Closed;

// The state machine struct — generic over current state
struct Connection<State> {
    host: String,
    port: u16,
    messages_sent: u32,
    _state: PhantomData<State>,  // zero cost, carries type info
}

// Methods only on Disconnected
impl Connection<Disconnected> {
    fn new(host: &str, port: u16) -> Self {
        Connection { host: host.into(), port, messages_sent: 0, _state: PhantomData }
    }
    // Consuming transition: Disconnected → Connected
    fn connect(self) -> Connection<Connected> {
        println!("Connecting to {}:{}", self.host, self.port);
        Connection { _state: PhantomData, ..self }  // state type changes, data moves
    }
}

// Methods only on Connected
impl Connection<Connected> {
    // Consuming transition: Connected → Authenticated
    fn authenticate(self, _token: &str) -> Connection<Authenticated> {
        Connection { _state: PhantomData, ..self }
    }
}

// Methods only on Authenticated
impl Connection<Authenticated> {
    fn send(&mut self, msg: &str) {        // &mut self — doesn't transition
        println!("→ {}", msg);
        self.messages_sent += 1;
    }
    fn disconnect(self) -> Connection<Closed> {
        Connection { _state: PhantomData, ..self }
    }
}

// Valid sequence — compiles
let conn = Connection::new("api.example.com", 443)
    .connect()
    .authenticate("secret-token");
// ... use conn.send() ...
let closed = conn.disconnect();

// These DO NOT COMPILE:
// Connection::new("h", 80).send("x");  // Disconnected has no send()
// closed.send("x");                    // Closed has no send()
```

**The state machine macro** (`state_machine!` in the example) generates the state structs, the generic struct with `PhantomData`, and the `impl` blocks for each transition from the same concise DSL.

## What This Unlocks

- **Protocol correctness at compile time** — TCP, TLS, HTTP, OAuth, database transactions — any multi-step protocol can be modelled so misuse is a build failure.
- **Self-documenting APIs** — the type signature of a function that accepts `Connection<Authenticated>` is documentation that cannot go stale.
- **Macro-generated machines** — a `state_machine!` DSL lets you declare dozens of states and transitions concisely without hand-writing every `PhantomData` impl.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| State encoding | Variant in a GADT or module type | Zero-sized struct as phantom type parameter |
| Invalid transitions | Runtime exception or explicit result type | Compile error — method doesn't exist on the wrong type |
| Transition cost | Pattern match + allocation if boxing | Zero cost — state type is erased, only the generic changes |
| State machine DSL | GADT or first-class modules | `macro_rules!` generating `impl` blocks per state |
| Mutating within a state | Record update syntax | `&mut self` methods on specific `impl Connection<State>` |
