# 397: Marker Traits

**Difficulty:** 2  **Level:** Intermediate

Traits with no methods — they mark a type as having a property the compiler can reason about.

## The Problem This Solves

Some properties of a type can't be expressed as methods. "This type is safe to send to another thread" isn't a behavior — it's a guarantee about ownership and memory access. "This type can be trivially copied by bit-copy" isn't something you can test at runtime; it's a structural fact.

Without a type-level mechanism for these properties, you'd rely on documentation and programmer discipline. Rust instead encodes them as marker traits. `Send` marks types safe to move across threads. `Sync` marks types safe to share references across threads. `Copy` marks types whose values can be duplicated by copying their bits. The compiler checks these automatically and rejects code that violates the guarantees.

This is how Rust prevents data races at compile time. `thread::spawn` requires `T: Send`. If you try to send an `Rc<T>` (non-atomic reference counting, not thread-safe) to another thread, the compiler refuses. No runtime check, no data race.

## The Intuition

A marker trait is an empty trait — no methods, no associated types. It's a flag. By implementing it for a type (or deriving it), you're telling the compiler "this type has property X." The compiler then uses that information in bounds on functions and other traits.

Most marker traits in the standard library are **auto traits**: the compiler implements them automatically for any type whose fields all satisfy the bound. `Point { x: f64, y: f64 }` is automatically `Send + Sync + Copy` because `f64` is all three. If any field is not `Send`, the containing struct is not `Send`.

`Copy` is special: it changes semantics. Normally, moving a value transfers ownership. `Copy` types are instead *duplicated* — both the original and the copy remain valid. This is why `let y = x; println!("{}", x)` works for `i32` but not for `String`.

## How It Works in Rust

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// Point is automatically Send + Sync + Copy because all fields are f64
#[derive(Debug, Clone, Copy)]
struct Point { x: f64, y: f64 }

fn demonstrate_copy() {
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = p1;  // Copy — p1 is still valid!
    let p3 = p1;  // Can copy again
    println!("p1: {:?}, p2: {:?}, p3: {:?}", p1, p2, p3);
}

fn demonstrate_send() {
    let p = Point { x: 3.0, y: 4.0 };
    // Point: Send — safe to move to another thread
    let handle = thread::spawn(move || p.x + p.y);
    println!("Result: {}", handle.join().unwrap());
}

fn demonstrate_sync() {
    // Arc<Mutex<T>>: Send + Sync — safe to share across threads
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    let data2 = Arc::clone(&data);
    let handle = thread::spawn(move || {
        data2.lock().unwrap().push(4);
    });
    handle.join().unwrap();
    println!("Shared: {:?}", data.lock().unwrap());
}

// Compile-time guarantee functions — the compiler won't compile if bound fails
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

// These pass:
// assert_send::<Point>();
// But this would FAIL to compile:
// assert_send::<std::rc::Rc<i32>>();  // Rc is !Send
```

For unsafe code, you can manually implement marker traits:
```rust
struct MyBuffer { ptr: *mut u8, len: usize }
// Raw pointer makes MyBuffer !Send by default
// We know our synchronization is correct, so we assert it:
unsafe impl Send for MyBuffer {}
unsafe impl Sync for MyBuffer {}
```

## What This Unlocks

- **Compile-time thread safety** — `thread::spawn` requires `T: Send`; data race prevention is zero-cost and guaranteed.
- **Copy semantics** — derive `Copy` for small value types (coordinates, IDs, flags) to avoid `.clone()` noise and pass by value without moves.
- **Opt-in capabilities** — define your own marker trait (`trait Cacheable {}`) and use it as a bound to restrict which types can be used in sensitive contexts.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Thread safety | Domain separation (OCaml 5) — runtime checked | `Send`/`Sync` marker traits — compile-time checked |
| Copy semantics | Values are immutable by default; mutation via `ref` | `Copy` opt-in; without it, assignment moves ownership |
| Phantom type markers | `type 'a resource` phantom types for encoding properties | Marker traits — enforced by the compiler via bounds |
| Negative impls | Not applicable | `!Send`, `!Sync` — a type can opt OUT of auto-derived markers |
