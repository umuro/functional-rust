**Difficulty:** ⭐  
**Category:** Functional Programming  

[thread-local on hightechmind.io](https://hightechmind.io/posts/functional-rust/thread-local)

---

## Problem Statement

Demonstrate thread-local storage (TLS) in Rust using the `thread_local!` macro. Each thread gets its own independent copy of the storage — no locks or synchronization needed. Show a thread-local counter where threads set independent values, and a thread-local accumulator that aggregates per-thread sums without shared state.

## Learning Outcomes

- Declare thread-local storage with `thread_local! { static NAME: RefCell<T> = ... }`
- Access TLS via `.with(|cell| ...)` — the closure receives a reference to the thread-local value
- Use `RefCell` for interior mutability: `borrow()` for read, `borrow_mut()` for write
- Understand that `thread_local!` values are not `Send` and cannot be moved between threads
- Recognize the use cases: per-thread performance counters, per-request contexts, PRNG state

## Rust Application

```rust
thread_local! {
    static COUNTER: RefCell<i32> = const { RefCell::new(0) };
}

fn thread_local_counter() -> Vec<i32> {
    let results = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..5i32)
        .map(|i| {
            let results = Arc::clone(&results);
            thread::spawn(move || {
                // Each thread has its own COUNTER
                COUNTER.with(|c| *c.borrow_mut() = i * 10);
                thread::yield_now();
                let v = COUNTER.with(|c| *c.borrow());
                results.lock().unwrap().push(v);
            })
        })
        .collect();

    for h in handles { h.join().unwrap(); }
    let mut v = results.lock().unwrap().clone();
    v.sort();
    v
}
```

`thread_local!` with `RefCell<T>` provides interior mutability without synchronization — since each thread has its own instance, `borrow_mut()` never contends. The `const { RefCell::new(0) }` initializer runs once per thread on first access.

`.with(|c| ...)` — the closure receives `&RefCell<i32>`. Inside, `borrow_mut()` gives `RefMut<i32>` for mutation. The borrow is checked at runtime but practically free since no other thread accesses the same cell.

## OCaml Approach

```ocaml
(* OCaml: Thread.self() as key into a Hashtbl — manual TLS *)
let tls_table : (int, int) Hashtbl.t = Hashtbl.create 16
let tls_mutex = Mutex.create ()

let tls_set v =
  let tid = Thread.id (Thread.self ()) in
  Mutex.lock tls_mutex;
  Hashtbl.replace tls_table tid v;
  Mutex.unlock tls_mutex

let tls_get () =
  let tid = Thread.id (Thread.self ()) in
  Mutex.protect tls_mutex (fun () ->
    Hashtbl.find_opt tls_table tid
  )

(* OCaml 5.0+: Domain.DLS for domain-local storage *)
let key = Domain.DLS.new_key (fun () -> 0)
let set v = Domain.DLS.set key v
let get () = Domain.DLS.get key
```

OCaml before 5.0 lacks built-in TLS — it requires a `Hashtbl` keyed by thread ID with manual locking. OCaml 5.0+'s `Domain.DLS` provides domain-local storage analogous to Rust's `thread_local!`.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Declaration | `thread_local! { static N: T = ... }` | No built-in (pre-5.0); `Domain.DLS` (5.0+) |
| Access | `.with(|r| ...)` closure | `Domain.DLS.get key` |
| Interior mutability | `RefCell<T>` in TLS | Mutable domain-local slot |
| Lock-free | Yes — no concurrent access possible | Yes (domain-local) |

TLS is ideal for per-thread random number generators, per-request logging contexts, and accumulating performance counters that are merged at the end. The key advantage over `Mutex<T>` is zero synchronization overhead.

## Exercises

1. Implement a thread-local RNG: each thread seeds its own `rand::thread_rng()` equivalent.
2. Implement per-thread allocation counters that are summed at program end without a shared counter.
3. Implement a "request ID" TLS that is set at thread entry and read by all functions without passing it as a parameter.
4. Demonstrate that modifying `COUNTER` in one thread does not affect another thread's `COUNTER` value.
5. Implement `thread_local_cache<K: Hash+Eq, V>` — a per-thread HashMap that serves as a local cache before hitting a shared `Mutex<HashMap>`.
