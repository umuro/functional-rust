# 326: Capturing with async move

**Difficulty:** 3  **Level:** Advanced

`async move { }` captures its environment by value — transferring ownership into the future so it can safely outlive its creating scope.

## The Problem This Solves

You create a future inside a function and want to send it to another thread or store it for later. But the future references local variables — when the function returns, those variables are gone, and the future is holding dangling pointers. The compiler rejects this.

`async move` solves this by moving ownership into the future. The future becomes self-contained, `'static`, and safe to send across threads. Without `move`, every async closure that captures its environment would require lifetime annotations threading through your entire call stack.

This pattern also enables the standard way to share mutable state across concurrent tasks: clone an `Arc` before moving into each task. Each task gets its own `Arc` pointer (cheap to clone), all pointing to the same data, protected by a `Mutex`.

## The Intuition

Regular closures can either borrow (`|| x + 1`) or own (`move || x + 1`) their captures. Async blocks work the same way, but the "own" version is far more common because futures often need to live longer than the scope that created them.

Think of it as packing your bags before a trip. A regular closure is like leaving your stuff at home and visiting — you need to return. An `async move` closure is like taking everything with you — you can go anywhere, indefinitely.

In JavaScript, closures in async contexts close over variables by reference, which is why `let` vs `var` in loops matters so much. Rust is explicit about this: `move` keyword, compiler enforces it.

```rust
// Without move: borrows from current scope — can't outlive it
let fut = async { compute(&local_var) };  // borrows local_var

// With move: owns the data — can live as long as needed
let fut = async move { compute(local_var) };  // local_var moved into fut
```

## How It Works in Rust

```rust
// move closure: owns `name`, can be called from anywhere
fn make_greeter(name: String) -> impl Fn() {
    move || println!("Hello, {name}!")   // name is moved in, greeter owns it
}

// Shared state across threads with Arc<Mutex<T>>
fn shared_state_demo() -> i32 {
    let shared = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..5).map(|_| {
        let shared = Arc::clone(&shared);  // clone Arc before moving (cheap: just ref count)
        thread::spawn(move || {            // move the cloned Arc into the thread
            *shared.lock().unwrap() += 1; // lock, modify, auto-unlock on drop
        })
    }).collect();

    for h in handles { h.join().unwrap(); }
    *shared.lock().unwrap()  // returns 5
}
```

`Arc::clone(&shared)` before the `move` is the canonical pattern — each thread gets its own Arc handle pointing to the same allocation. `Mutex` ensures only one thread modifies the data at a time.

## What This Unlocks

- **Spawning futures**: `tokio::spawn(async move { ... })` requires `'static` — `async move` is the standard way to satisfy this.
- **Parallel state updates**: Multiple threads safely modifying shared data via `Arc<Mutex<T>>`.
- **Factory functions**: Return closures or futures that carry their dependencies, making them independent of the creating scope.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Capture by value | `let x = x in fun () -> ...` | `move \|\| ...` or `async move { }` |
| Shared ownership | `ref` (mutable reference) | `Arc<T>` (atomic reference count) |
| Mutual exclusion | `Mutex.create ()` | `Mutex<T>` (wraps the data itself) |
| Thread safety | `Thread.create` with shared ref | `Arc<Mutex<T>>` — enforced by compiler |
