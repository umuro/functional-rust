# 506: Move Closures and Ownership

**Difficulty:** 3  **Level:** Intermediate

The `move` keyword gives closures full ownership of their captures — essential for threads, async tasks, and returning closures from functions.

## The Problem This Solves

You write `thread::spawn(|| { use_data(data) })` and get: `closure may outlive the current function, but it borrows data`. The thread might outlive the function that created it — Rust refuses to let a thread borrow something that might be deallocated while the thread is still running.

The same error appears when returning closures from functions: `cannot return value referencing local variable`. The closure borrows something on the stack, but the stack is gone after the function returns.

Without `move`, closures borrow their environment, creating lifetime constraints that the borrow checker enforces strictly. These errors protect you from use-after-free bugs — the kind that cause CVEs in C/C++.

## The Intuition

A borrowing closure is like a library checkout form — it says "I need this book for a while, please don't throw it away." When the function returns or the thread outlives the scope, the form expires but the book might already be gone.

A `move` closure is like *buying* the book — you own it now. No return date, no dependency on the original shelf. The closure carries its own copy of everything it needs.

In Python and JavaScript, all closures implicitly own their captured values (or the GC keeps them alive). Rust makes the distinction explicit: borrow (default, free, has lifetime constraints) vs. own (`move`, may copy/move data, no lifetime constraints).

## How It Works in Rust

```rust
// Without move: closure borrows — constrained to outlive 'x
let x = 10;
let f = |n| n + x;     // borrows x as &i32 — fine here
println!("{}", f(5));  // x still usable

// With move: required for threads
let data = vec![1, 2, 3, 4, 5];
let handle = thread::spawn(move || {       // data MOVED into closure
    let sum: i32 = data.iter().sum();
    println!("Thread sum: {}", sum);
    sum
});
// data is gone from this scope — the thread owns it now
let result = handle.join().unwrap();

// With move: required for returning from functions
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x + n    // n moved into closure; closure owns it
}                      // make_adder's stack is gone, but closure survives

// Copy types: both owner and closure have the value
let value = 42i32;    // i32 is Copy
let f1 = move || value * 2;   // value COPIED into f1
let f2 = move || value + 10;  // value COPIED into f2
println!("{}", value); // original still usable — it was copied, not moved

// Non-Copy types: ownership transfers
let msg = String::from("hello");
let print_msg = move || println!("{}", msg);   // String MOVED
// println!("{}", msg);  // ✗ ERROR — msg was moved
print_msg(); // still callable — closure owns the String
print_msg(); // Fn: can call multiple times
```

## What This Unlocks

- **Safe concurrency** — `thread::spawn(move || ...)` transfers data ownership to the thread, guaranteeing no data races at compile time.
- **Async tasks** — async runtimes require `'static` futures; `move` gives closures ownership of their data, satisfying this requirement.
- **Factory functions** — return closures from functions without lifetime parameters by moving the configuration into the closure.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Closure lifetime | GC handles — closures live forever | Bounded by captures unless `move` |
| Thread closure | GC-safe by default | `move ||` required for `Send + 'static` |
| Ownership transfer | Implicit (GC traces references) | Explicit `move` keyword |
| Non-Copy type in closure | Shared via GC | Moved in — original binding invalidated |
| Copy type in closure | Always shared | Copied — both owner and closure have it |
