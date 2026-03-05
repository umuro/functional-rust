📖 **[View on hightechmind.io →](https://hightechmind.io/rust/527-closure-once-consumer)**

---

# 527: FnOnce — Consuming Closures

**Difficulty:** 3  **Level:** Intermediate

Model "call this exactly once" as a type guarantee — the compiler enforces that the closure consumes its captured values and cannot be called again.

## The Problem This Solves

Some operations are inherently one-shot: send a message on a channel (the channel is consumed), close a file descriptor (closing twice is a bug), authenticate a one-time token (replay is a security hole), join a thread (you can't join the same handle twice). In most languages, preventing double-invocation requires a runtime flag, a `bool` guard, or documentation that developers are expected to read and follow. Bugs happen when they don't.

Rust's `FnOnce` trait encodes this at the type level. A `FnOnce` closure moves its captured variables when called — after the call, those variables are gone. The closure *cannot* be called again because it no longer owns the values it needs to run. The compiler catches the double-call at compile time, not at runtime. No guard needed. No documentation to miss.

This is Rust's linear-type capability in practice: a value that can be used exactly once, after which it ceases to exist. It's the only mainstream language where the type system enforces "this callable can be invoked only once."

## The Intuition

An `FnOnce` closure is like a self-destructing envelope. The moment you open it (call it), the contents are consumed — gone. You can hold onto the envelope as long as you like; you just can't open it twice. The type system is the lock on the envelope: `Box<dyn FnOnce()>` is the envelope, calling `.call_once()` opens it, and the borrow checker prevents any subsequent call.

The three closure traits form a hierarchy:
- `FnOnce`: can be called at most once (moves captures)  
- `FnMut`: can be called multiple times (mutates captures)  
- `Fn`: can be called from multiple threads simultaneously (only shared borrows)

Every `Fn` is also `FnMut` and `FnOnce`. Every `FnMut` is also `FnOnce`. Use the most permissive bound your code can accept.

## How It Works in Rust

```rust
// A resource that can only be consumed once.
struct OneTimeToken { value: String }
impl OneTimeToken {
    fn consume(self) -> String { self.value }  // moves self — cannot call twice
}

// FnOnce captures OneTimeToken by move.
fn make_consumer(token: OneTimeToken) -> impl FnOnce() -> String {
    move || token.consume()  // token is moved on call — gone after
}

let consumer = make_consumer(OneTimeToken { value: "auth-xyz".into() });
let result = consumer();  // ✅ works
// consumer();            // ❌ compile error: use of moved value

// Store a FnOnce in a struct — must use Box<dyn FnOnce()> or Option<F>.
struct OnceAction {
    action: Option<Box<dyn FnOnce()>>,
}

impl OnceAction {
    fn run(&mut self) {
        if let Some(f) = self.action.take() {
            f();  // consumes the box; action is now None
        }
    }
}

// Thread spawn always takes FnOnce + Send — the closure runs once, in another thread.
let handle = std::thread::spawn(move || {
    // captured variables moved here
    println!("running in thread");
});
handle.join().unwrap();
```

## What This Unlocks

- **Resource safety without runtime guards**: File handles, sockets, channels — model "close once" as a type constraint, not a documentation requirement.
- **One-shot initialisation**: `std::sync::OnceLock` / `once_cell` use the same concept: run exactly once, then cache the result forever.
- **Thread spawning**: `thread::spawn` requires `FnOnce + Send` — the type system ensures captured data is moved into the thread and not shared.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| One-shot enforcement | No enforcement — manual flag | `FnOnce` — compiler enforced |
| Consuming resource on call | Manual convention | Value moved on call — type error if called again |
| Returning owned value | Function returns value | `FnOnce() -> T` — callee gets ownership |
| Storing FnOnce | First-class function | `Box<dyn FnOnce()>` or `Option<F>` |
| Thread closure | `Thread.create` (GC manages) | `thread::spawn(move || ...)` — `FnOnce + Send` |
| Double-call prevention | Runtime check or trust | Compile-time error |
