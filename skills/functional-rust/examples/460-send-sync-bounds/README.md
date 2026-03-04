# 460: Send + Sync — Compile-Time Thread Safety Proofs

**Difficulty:** 3  **Level:** Intermediate

Understand Rust's two marker traits that make "if it compiles, no data races" possible — and learn to read the errors when you violate them.

## The Problem This Solves

Every other language with threads relies on runtime checks, discipline, or luck for thread safety. Java has `synchronized` that you can forget. Python has the GIL that hides races until you switch to multiprocessing. Go has a race detector that's opt-in and runtime-only. These approaches catch errors late — in production, under load, on machines you can't inspect.

Rust catches thread safety violations at compile time. The mechanism is two marker traits: `Send` and `Sync`. If a type isn't `Send`, the compiler refuses to let you move it to another thread. If a type isn't `Sync`, the compiler refuses to let you share a reference to it across threads. These checks are exhaustive: every type in every crate is checked, including your own. You cannot accidentally forget to apply them.

The practical consequence: `thread::spawn` has a bound `F: Send + 'static` on its closure. If you accidentally capture an `Rc<T>` (which is not `Send` because its reference count is not atomic), the compiler tells you immediately with a clear error message, not a race condition observed months later in production.

## The Intuition

- **`Send`**: "This value can be moved to another thread." Types like `i32`, `String`, `Vec<T>`, `Arc<T>`, `Mutex<T>` are `Send`. Types like `Rc<T>` (non-atomic refcount), `*mut T` (raw pointer), and `RefCell<T>` are not.
- **`Sync`**: "A shared reference `&T` to this value can be accessed from multiple threads." If `T: Sync`, then `&T: Send`. Types like `i32`, `Mutex<T>`, `AtomicUsize` are `Sync`. Types like `RefCell<T>` are not (its borrow tracking is not thread-safe).

These are **auto-traits**: the compiler derives them automatically based on all fields. A struct where every field is `Send` is automatically `Send`. A struct containing an `Rc<T>` is automatically `!Send`. You only implement them manually in `unsafe impl` blocks when you have raw pointers and can prove thread safety yourself (as in example 455).

## How It Works in Rust

```rust
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::thread;

// All fields are Send + Sync → Counter is automatically Send + Sync
struct Counter {
    value: Mutex<u64>,  // Mutex<u64>: Send + Sync
    label: String,      // String: Send + Sync
}

impl Counter {
    fn new(s: &str) -> Self { Counter { value: Mutex::new(0), label: s.to_string() } }
    fn inc(&self) { *self.value.lock().unwrap() += 1; }
    fn get(&self) -> u64 { *self.value.lock().unwrap() }
}

// Use across threads — works because Counter: Send + Sync
let c = Arc::new(Counter::new("test"));
let handles: Vec<_> = (0..4).map(|_| {
    let c = Arc::clone(&c);
    thread::spawn(move || { for _ in 0..100 { c.inc(); } })
}).collect();
for h in handles { h.join().unwrap(); }
println!("{}: {}", c.label, c.get()); // test: 400

// Compile-time proofs — these functions are zero-cost assertions
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}
assert_send::<Counter>();   // compiles → Counter is Send
assert_sync::<Counter>();   // compiles → Counter is Sync

// This would NOT compile:
// struct LocalOnly { _rc: Rc<String> }  // Rc is !Send
// thread::spawn(move || { let _ = LocalOnly { _rc: Rc::new("".into()) }; });
// Error: `Rc<String>` cannot be sent between threads safely
```

When you see a compile error mentioning `Send` or `Sync`, read it as: "you're trying to cross a thread boundary with a type that isn't safe to cross." The fix is almost always: replace `Rc` with `Arc`, replace `RefCell` with `Mutex`, or restructure to avoid sharing.

## What This Unlocks

- **Confidence in concurrent code** — any struct that compiles with `thread::spawn` is provably free of data races due to the types it contains.
- **Library API design** — annotate your types with `Send + Sync` bounds to communicate thread-safety guarantees to callers; the compiler enforces them.
- **Diagnosing concurrency errors** — thread-safety errors in Rust are compile errors with actionable messages, not Heisenbugs discovered under load.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Send types | GC-managed — all values movable by runtime | `Arc<T>`, `Mutex<T>`, `String`, `i32` — auto-derived |
| `!Send` types | N/A (GC handles all) | `Rc<T>`, `*mut T`, `RefCell<T>` — not safe to send |
| Sync types | immutable values freely shared | `Arc<T>`, `Mutex<T>`, `AtomicUsize` — auto-derived |
| `!Sync` types | N/A | `Cell<T>`, `RefCell<T>` — interior mutability without locks |
| Enforcement | runtime + programmer discipline | compile-time — `Send`/`Sync` violations are errors |
| Manual impl | N/A | `unsafe impl Send for T {}` — you promise it's safe |
