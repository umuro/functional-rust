# 326: Capturing with async move

**Difficulty:** 3  **Level:** Advanced

`async move { }` captures its environment by value — transferring ownership into the future so it can safely outlive its creating scope.

## The Problem This Solves

You create a future inside a function and want to send it to another thread or store it for later. But the future references local variables — when the function returns, those variables are gone. `async move` solves this by moving ownership into the future.

This pattern also enables sharing mutable state across concurrent tasks: clone an `Arc` before moving into each task.

## The Intuition

Regular closures can either borrow (`|| x + 1`) or own (`move || x + 1`) their captures. Async blocks work the same way, but `move` is far more common because futures often need to live longer than the scope that created them.

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
    move || println!("Hello, {name}!")
}

// Shared state across threads with Arc<Mutex<T>>
fn shared_state_demo() -> i32 {
    let shared = Arc::new(Mutex::new(0));
    let handles: Vec<_> = (0..5).map(|_| {
        let shared = Arc::clone(&shared);  // clone Arc before moving
        thread::spawn(move || {
            *shared.lock().unwrap() += 1;
        })
    }).collect();
    for h in handles { h.join().unwrap(); }
    *shared.lock().unwrap()
}
```

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Capture by value | `let x = x in fun () -> ...` | `move \|\| ...` |
| Shared ownership | `ref` (mutable reference) | `Arc<T>` |
| Mutual exclusion | `Mutex.create ()` | `Mutex<T>` |
