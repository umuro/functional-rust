# 734: Typestate Pattern: Encode State in the Type

**Difficulty:** 4  **Level:** Expert

Use zero-sized type parameters to make invalid state transitions a compile error — the type system enforces your protocol, with zero runtime overhead.

## The Problem This Solves

State machines are everywhere: a traffic light that can only go Red → Green → Yellow → Red. A TCP connection that must Connect before it can Send. A file handle that must be opened before it can be read. Traditionally, you enforce these transitions at runtime: check a flag, return an error, maybe panic. This means bugs are caught at runtime — or worse, in production.

With the typestate pattern, the state is encoded in the *type*, not in a field. `Light<Red>` and `Light<Green>` are different types. The `go()` method only exists on `Light<Red>`, not on `Light<Green>`. You can't call `.stop()` on a green light because there is no `stop` method on `Light<Green>` — the compiler simply doesn't know what you're talking about.

The result: invalid state transitions don't compile. There's no runtime check, no error handling, no test needed for the "impossible" case. The bug cannot exist in a compiled binary.

## The Intuition

In Python, you'd use an enum for the state and check it in every method: `if self.state != State.RED: raise ValueError("not red")`. This works but requires defensive programming everywhere and can only catch errors at runtime.

The typestate pattern moves that check to the type system. Instead of `self.state == State.RED`, the type `Light<Red>` itself is the check. If you have a `Light<Red>`, it's red — the type guarantees it. No flag needed.

`PhantomData<State>` is the key: it's a zero-sized field that holds the type parameter without occupying any bytes. `Light<Red>` and `Light<Green>` are distinct types despite being structurally identical at the byte level.

## How It Works in Rust

```rust
use std::marker::PhantomData;

// Zero-sized state marker types — no data, just type identity
pub struct Red;
pub struct Green;
pub struct Yellow;

// The light struct carries the state in its type parameter
// PhantomData<State> is 0 bytes at runtime
pub struct Light<State> {
    _state: PhantomData<State>,
}

// Methods only defined on specific states
impl Light<Red> {
    pub fn new() -> Self {                  // can only start Red
        Light { _state: PhantomData }
    }

    pub fn go(self) -> Light<Green> {       // consumes self: Red is GONE
        Light { _state: PhantomData }       // returns a new Light<Green>
    }
    // No `slow()` or `stop()` methods — can't call them on Red
}

impl Light<Green> {
    pub fn slow(self) -> Light<Yellow> {
        Light { _state: PhantomData }
    }
    // No `go()` — can't transition Green → Green
}

impl Light<Yellow> {
    pub fn stop(self) -> Light<Red> {
        Light { _state: PhantomData }
    }
}

// Valid — the full cycle compiles
let red    = Light::<Red>::new();
let green  = red.go();
let yellow = green.slow();
let red2   = yellow.stop();

// These would NOT compile:
// red.slow();    // ERROR: no method `slow` found for `Light<Red>`
// green.go();    // ERROR: no method `go` found for `Light<Green>`
// red.stop();    // ERROR: no method `stop` found for `Light<Red>`

// Size check — PhantomData is zero bytes
assert_eq!(std::mem::size_of::<Light<Red>>(), 0);
```

Key points:
- State transition methods take `self` (not `&self`) — they *consume* the old state and return the new type
- Once you call `red.go()`, the `red` variable is moved and cannot be used again — the old state is gone
- `PhantomData<T>` tells the compiler "this type owns or is associated with T" without storing any T data
- State marker types (`Red`, `Green`, `Yellow`) are zero-sized — no memory cost at all
- The pattern composes: a file `handle` can be `Open`, `Locked`, or `Closed`, with reads only on `Open` handles

## What This Unlocks

- **Protocol enforcement without runtime cost**: HTTP clients, database connections, file handles, network sockets — all can use typestate to enforce correct usage order
- **Impossible states are unrepresentable**: "opened-twice" or "closed-before-opened" bugs literally cannot compile — no test needed for those cases
- **Self-documenting APIs**: the type signature `fn connect(self: Session<Disconnected>) -> Session<Connected>` tells users the protocol without reading documentation

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| State in type | Phantom types with GADTs | `PhantomData<State>` with zero-sized markers |
| Invalid transitions | Compile error via type mismatch | Compile error — method doesn't exist on that type |
| State consumption | Functional style: return new state | Move semantics: `self` is consumed, new type returned |
| Runtime overhead | Zero with phantom types | Zero — `PhantomData` and markers are erased |
| State marker type | `type 'red light` phantom | `struct Red;` zero-sized struct |
| Protocol machines | Type-indexed by phantom | Generic struct `Session<State>` + per-state `impl` |
