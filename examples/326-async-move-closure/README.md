📖 **[View on hightechmind.io →](https://hightechmind.io/rust/326-async-move-closure)**

---

# 326: Capturing with async move

## Problem Statement

Async tasks often need to use data from the surrounding scope — a user ID, a connection string, or a shared counter. Since async tasks may outlive the scope where they are created, they cannot borrow — they must own their data. The `async move { }` block (and `move ||` closure) captures all referenced variables by value, giving the async task ownership. This is required whenever a spawned task needs access to outer-scope data.

## Learning Outcomes

- Understand `move ||` and `async move { }` as capturing environment by ownership
- Recognize why spawned tasks require `'static` lifetime — they must own their data
- Implement shared mutable state across tasks using `Arc<Mutex<T>>`
- Understand the difference between `move` captures (one task per capture) and `Arc` clones (multiple tasks sharing)

## Rust Application

`move` closures and `Arc` patterns for concurrent access:

```rust
// move closure: captures `name` by ownership
pub fn make_greeter(name: String) -> impl Fn() -> String {
    move || format!("Hello, {}!", name)
}

// Arc<Mutex<T>>: share mutable state across multiple closures
pub fn shared_counter() -> impl FnMut() -> i32 {
    let count = Arc::new(Mutex::new(0));
    move || {
        let mut c = count.lock().unwrap();
        *c += 1;
        *c
    }
}
```

In async code: `tokio::spawn(async move { use_data(captured_value) })`.

## OCaml Approach

OCaml closures capture variables from the enclosing scope by reference (the GC handles lifetimes), so explicit `move` is not needed:

```ocaml
let make_greeter name = fun () -> "Hello, " ^ name ^ "!"
(* `name` is captured by the closure — GC ensures it lives long enough *)
```

For Lwt concurrent tasks, `Lwt.async` with shared mutable refs:

```ocaml
let counter = ref 0
let increment () = Lwt.return (incr counter; !counter)
```

## Key Differences

1. **Ownership transfer**: Rust's `move` explicitly transfers ownership to the closure — the original binding can no longer be used; OCaml closures share by reference with GC management.
2. **Lifetime requirement**: Rust's `thread::spawn` / `tokio::spawn` require `'static` (owned) data; `move` is the primary tool to satisfy this.
3. **Multi-consumer sharing**: When multiple tasks need the same data, `Arc::clone()` before each `move ||` gives each task its own reference-counted pointer.
4. **FnOnce vs Fn**: `move ||` that captures an owned non-`Clone` value implements `FnOnce` — can only be called once; `Arc` enables `Fn` (callable many times).

## Exercises

1. Implement a factory function that creates N worker closures, each capturing a unique ID by value, and run them concurrently.
2. Use `Arc<Mutex<Vec<String>>>` to collect results from multiple threads into a shared accumulator.
3. Show the compilation error when trying to spawn a thread that borrows a local variable, then fix it using `move`.
