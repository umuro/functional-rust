# 130: Typestate Pattern

**Difficulty:** ⭐⭐⭐  **Level:** Advanced

Make invalid state transitions impossible by encoding state in the type — a `Door<Locked>` has no `open()` method because it doesn't exist.

## The Problem This Solves

Consider a database connection. You need to connect, then authenticate, then query. What stops someone from calling `query()` on a connection that hasn't authenticated yet? Without typestate: runtime checks and panics, or `Result` errors everywhere, or trusting documentation. The type `Connection` is the same whether you've authenticated or not — the compiler can't help you.

The same problem shows up constantly: a file that's readable before it's opened, a network socket that's writable before it's bound, an HTTP request that can be sent before a URL is set. In each case, some methods are only valid in some states, but the type doesn't say which.

With typestate, each state is a different type. `Connection<Disconnected>` and `Connection<Authenticated>` are different types. The `query()` method only exists on `Connection<Authenticated>`. You *cannot* call it on `Connection<Disconnected>` — the method simply doesn't exist there. The compiler enforces the protocol at every call site. This pattern is used in `tokio` (channel Sender/Receiver states) and `sqlx` (prepared statements).

## The Intuition

The trick is simple: make the state a phantom type parameter. `struct Door<State>` where `State` is a zero-sized marker struct (`struct Open;`, `struct Closed;`, `struct Locked;`). These markers store nothing — they're pure type-level labels.

Then write separate `impl` blocks for each state. `impl Door<Open>` gets the `walk_through()` and `close()` methods. `impl Door<Closed>` gets `open()` and `lock()`. `impl Door<Locked>` gets only `unlock()`. Each transition method *consumes* the old door (takes `self` by value) and returns a door with a new state type. The old door is gone — you can't accidentally use a locked door as if it were open because the variable has been moved.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// State markers — zero-sized, no data stored
struct Open;
struct Closed;
struct Locked;

// Door parameterized by its current state
struct Door<State> {
    material: String,
    _state: PhantomData<State>,  // PhantomData: use State in type without storing it
}

// Methods available only when the door is Open
impl Door<Open> {
    fn new(material: &str) -> Self {
        Door { material: material.to_string(), _state: PhantomData }
    }

    fn walk_through(&self) { println!("Walking through {} door", self.material); }

    fn close(self) -> Door<Closed> {  // self is consumed — Door<Open> is gone
        Door { material: self.material, _state: PhantomData }
    }
}

// Methods available only when the door is Closed
impl Door<Closed> {
    fn open(self) -> Door<Open> {
        Door { material: self.material, _state: PhantomData }
    }
    fn lock(self) -> Door<Locked> {
        Door { material: self.material, _state: PhantomData }
    }
}

// Methods available only when the door is Locked
impl Door<Locked> {
    fn unlock(self) -> Door<Closed> {
        Door { material: self.material, _state: PhantomData }
    }
    // No open() here — you can't open a locked door directly
}
```

Valid usage:
```rust
let door = Door::<Open>::new("oak");
door.walk_through();               // ✓ Open has walk_through
let door = door.close();           // door is now Door<Closed>
let door = door.lock();            // door is now Door<Locked>
// door.open();                    // compile error: no method `open` on Door<Locked>
let door = door.unlock();          // Door<Closed>
let door = door.open();            // Door<Open>
door.walk_through();               // ✓
```

Connection protocol example:
```rust
struct Connection<S> { host: String, _state: PhantomData<S> }

impl Connection<Disconnected> {
    fn connect(self) -> Connection<Connected> { /* ... */ }
}
impl Connection<Connected> {
    fn authenticate(self, password: &str) -> Connection<Authenticated> { /* ... */ }
}
impl Connection<Authenticated> {
    fn query(&self, q: &str) -> String { /* ... */ }  // only here!
}
```

## What This Unlocks

- **Protocol enforcement** — TCP connections, TLS handshakes, OAuth flows: each step is a type transition; skipping a step is a compile error.
- **Resource lifecycle** — files that must be opened before reading, released before re-acquiring; the type tracks the lifecycle without runtime flags.
- **Builder APIs** — each `with_*` method transitions the builder type, and `build()` only exists when all required fields have been set (see example 131).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| State encoding | GADT: `type _ door = OpenDoor : open_state door \| ClosedDoor : closed_state door` | Phantom type param: `struct Door<State>` with marker structs |
| Method availability | Pattern-match phantom type in function signature | Separate `impl Door<Open>`, `impl Door<Closed>` blocks |
| Value consumption | Type annotation, value not consumed | `self` taken by value — old state is *moved away*, unusable |
| Zero runtime cost | Depends on OCaml's unboxing | `PhantomData` is zero-sized — no memory overhead |
