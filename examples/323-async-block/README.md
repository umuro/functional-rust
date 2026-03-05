📖 **[View on hightechmind.io →](https://hightechmind.io/rust/323-async-block)**

---

# 323: async blocks and Lazy Evaluation

**Difficulty:** 3  **Level:** Advanced

An `async { }` block creates an anonymous Future inline — lazy by default, nothing runs until it's awaited or driven.

## The Problem This Solves

Sometimes you want to create a piece of async work without defining a whole named function for it. Maybe you're building a list of tasks dynamically, or you need to capture some local variables into a future that will run later. Named `async fn` works for reusable operations, but for one-off deferred computations, it's too heavy.

More importantly, lazy evaluation is a superpower in concurrent programming. If you can describe work without starting it, you can decide *whether* to run it, *when* to run it, and *which* of several options to run. Eagerly starting work you might not need wastes resources and can cause race conditions.

## The Intuition

In Rust, `async { }` is to functions what closures are to named functions — anonymous, inline, capturing their environment. The difference from a regular closure: it returns a `Future`, not a value.

Think of it like a thunk in functional programming: `fun () -> expensive_computation()` in OCaml, or a lazy `val` in Haskell. The work is *described* but not *done*.

This example uses regular closures (`FnOnce`) as the synchronous analogy — same laziness, same capture semantics, no runtime needed.

## How It Works in Rust

```rust
// Create a lazy computation (like: let fut = async { expensive() })
fn lazy_comp<F: FnOnce() -> T, T>(label: &str, f: F) -> impl FnOnce() -> T + '_ {
    println!("Creating: {label}");   // runs immediately on creation
    move || {
        println!("Executing: {label}");  // runs only when called
        f()
    }
}

// Conditionally run — like: if cond { fut.await } else { None }
fn run_if<F: FnOnce() -> T, T>(cond: bool, t: F) -> Option<T> {
    if cond { Some(t()) } else { None }  // the work is skipped entirely if cond is false
}
```

The `move` capture transfers values into each closure by value. In async code, `async move { }` does the same thing.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Inline async | `fun () -> Lwt.return (f ())` | `async { f() }` |
| Lazy by default | explicit thunks needed | implicit — not polled until awaited |
| Capture by value | `let x = x in fun () -> x` | `async move { x }` |
| Type of lazy work | `unit -> 'a Lwt.t` | `impl Future<Output = A>` |
