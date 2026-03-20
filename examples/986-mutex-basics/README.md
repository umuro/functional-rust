**Difficulty:** ⭐  
**Category:** Functional Programming  

[mutex-basics on hightechmind.io](https://hightechmind.io/posts/functional-rust/mutex-basics)

---

## Problem Statement

Use `Mutex<T>` to protect shared mutable state across threads. Combine with `Arc<T>` for shared ownership. Demonstrate RAII-based lock acquisition (the guard drops automatically at end of scope), shared counter increment from 10 threads, and a mutex-protected `BankAccount` struct for structured state.

## Learning Outcomes

- Combine `Arc<Mutex<T>>` for shared mutable state: `Arc::clone` shares the pointer, `Mutex::lock` protects access
- Understand that `Mutex::lock` returns `MutexGuard<T>` — a RAII guard that releases the lock when dropped
- Write `*n += 1` to increment through the guard's `DerefMut` implementation
- Avoid deadlock by keeping lock scopes short: lock, mutate, drop guard, not lock across blocking operations
- Understand the OCaml equivalent: `Mutex.lock` / `Mutex.unlock` or `Mutex.protect`

## Rust Application

```rust
fn shared_counter() -> i32 {
    let counter = Arc::new(Mutex::new(0i32));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..100 {
                    let mut n = counter.lock().unwrap();
                    *n += 1;
                    // lock released when `n` drops at end of block
                }
            })
        })
        .collect();

    for h in handles { h.join().unwrap(); }
    *counter.lock().unwrap()  // should be 1000
}
```

`Arc::clone(&counter)` increments the reference count and gives each thread its own `Arc` pointer to the same `Mutex`. `counter.lock().unwrap()` blocks until the lock is available, then returns a `MutexGuard<i32>`. When `n` drops (end of the `for` loop body), the guard's `Drop` impl releases the mutex.

`counter.lock().unwrap()` — the `.unwrap()` handles poisoned mutexes (a mutex becomes poisoned if a thread panics while holding the lock). In production, handle `PoisonError` explicitly.

## OCaml Approach

```ocaml
let shared_counter () =
  let m = Mutex.create () in
  let counter = ref 0 in

  let threads = List.init 10 (fun _ ->
    Thread.create (fun () ->
      for _ = 1 to 100 do
        Mutex.lock m;
        incr counter;
        Mutex.unlock m
      done
    ) ()
  ) in

  List.iter Thread.join threads;
  !counter

(* Safer with protect *)
let with_mutex m f =
  Mutex.lock m;
  match f () with
  | v -> Mutex.unlock m; v
  | exception e -> Mutex.unlock m; raise e
```

OCaml's `Mutex.lock` / `Mutex.unlock` are explicit. `with_mutex` wraps them with exception safety — analogous to Rust's RAII guard. OCaml 5.0+ uses `Mutex.protect f` as the built-in equivalent.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Lock scope | RAII guard — automatic release | Manual lock/unlock or `Mutex.protect` |
| Shared ownership | `Arc<Mutex<T>>` | `Mutex.t` + `ref` (shared implicitly) |
| Poisoning | Mutex poisons on thread panic | No equivalent — mutex stays usable |
| Data ownership | `Mutex` owns `T` | Mutex and data are separate |

Rust's `Mutex<T>` is unique: the lock and the data it protects are the same object. You cannot access the data without holding the lock — the type system enforces this invariant.

## Exercises

1. Verify that 10 threads × 100 increments = 1000 (no data races).
2. Implement `withdraw` on `BankAccount` that returns `Err` if balance would go negative, while holding the lock for the entire check-and-debit operation.
3. Implement a deadlock scenario and explain why it deadlocks: two threads each try to acquire two mutexes in opposite orders.
4. Replace `Arc<Mutex<Vec<T>>>` with `Arc<RwLock<Vec<T>>>` for a read-heavy workload and benchmark.
5. Implement `try_lock` that returns `None` if the lock is contended rather than blocking.
