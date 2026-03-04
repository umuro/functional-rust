# 125: Send and Sync

**Difficulty:** 3  **Level:** Intermediate

Compile-time thread safety: `Send` lets you move a value to another thread; `Sync` lets you share a reference between threads.

## The Problem This Solves

Data races are one of the hardest bugs to debug: two threads access the same memory simultaneously, at least one writes, and there's no synchronization. In most languages, including OCaml, the programmer must manually ensure thread safety — the compiler won't stop you from sharing a non-thread-safe type across threads.

Rust makes data races a compile error. The mechanism is two marker traits: `Send` and `Sync`. `Send` means "it's safe to transfer ownership of this value to another thread." `Sync` means "it's safe to share a reference to this value between threads" — which is equivalent to `&T: Send`. Most types are both `Send` and `Sync` automatically — the compiler derives these for any type whose fields are all `Send`/`Sync`. The exceptions are explicit: `Rc<T>` is neither (non-atomic reference count), `Cell<T>` is `Send` but not `Sync` (interior mutability without synchronization), raw pointers are neither.

When you write `thread::spawn(move || { ... })`, the closure must be `Send`. If you accidentally capture an `Rc` or a `Cell`, the compiler refuses with a clear error. No segfault at 3am — a clear message at `cargo build`.

## The Intuition

`Send` = can be transferred to another thread. `Sync` = can be shared between threads. The compiler checks these automatically — if a type contains anything non-thread-safe, it inherits that restriction.

## How It Works in Rust

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// Vec<i32> is Send — ownership can transfer to another thread
let data = vec![1, 2, 3, 4, 5];
let handle = thread::spawn(move || data.iter().sum::<i32>());
assert_eq!(handle.join().unwrap(), 15);

// Arc<Mutex<T>> is both Send and Sync — the standard pattern for
// shared mutable state across threads
let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);  // Arc is Sync: safe to clone across threads
    handles.push(thread::spawn(move || {
        *counter.lock().unwrap() += 1;   // Mutex ensures only one thread at a time
    }));
}
for h in handles { h.join().unwrap(); }
assert_eq!(*counter.lock().unwrap(), 10);

// Types that are NOT Send — compiler catches misuse:
use std::rc::Rc;
let rc = Rc::new(42);
// thread::spawn(move || *rc);  // ERROR: `Rc<i32>` cannot be sent between threads safely
// Rc uses non-atomic reference counting — race condition waiting to happen.
// Use Arc<T> instead for thread-safe shared ownership.

// You can verify at compile time:
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}
assert_send::<String>();       // ✓
assert_send::<Arc<i32>>();     // ✓
assert_sync::<Arc<i32>>();     // ✓
assert_sync::<Mutex<i32>>();   // ✓
// assert_send::<Rc<i32>>();   // ✗ — compile error
```

## What This Unlocks

- **Data-race-free concurrency** — the compiler rejects programs that would share non-thread-safe state across threads.
- **`Arc<Mutex<T>>` pattern** — the standard Rust idiom for shared mutable state: `Arc` for shared ownership, `Mutex` for exclusive access.
- **Channel-based message passing** — `mpsc::channel` requires `Send` on the message type, making the data flow explicit and safe.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Thread safety enforcement | Runtime — programmer's responsibility | Compile time — `Send`/`Sync` trait bounds |
| Data races | Possible — no compiler check | Impossible — compile error |
| Shared ownership across threads | Mutex + GC | `Arc<T>` (requires `T: Send + Sync`) |
| Shared mutable state | Mutex (manual discipline) | `Arc<Mutex<T>>` (enforced by types) |
| Cost of the safety check | None at compile time, runtime risk | Zero runtime cost — types only |
