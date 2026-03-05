# OCaml vs Rust: Arc<T> — Thread-Safe Shared Ownership

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml's GC handles shared lifetimes automatically.
   In OCaml 5, Domains share heap values with no extra annotation. *)
let parallel_sum data =
  let mid = Array.length data / 2 in
  let d1 = Domain.spawn (fun () ->
    Array.fold_left (+) 0 (Array.sub data 0 mid)) in
  let d2 = Domain.spawn (fun () ->
    Array.fold_left (+) 0 (Array.sub data mid (Array.length data - mid))) in
  Domain.join d1 + Domain.join d2
```

### Rust (idiomatic — Arc for shared ownership)
```rust
use std::sync::Arc;
use std::thread;

pub fn parallel_sum(data: Arc<Vec<i32>>) -> i32 {
    let mid = data.len() / 2;

    let left = Arc::clone(&data);
    let h1 = thread::spawn(move || left[..mid].iter().sum::<i32>());

    let right = Arc::clone(&data);
    let h2 = thread::spawn(move || right[mid..].iter().sum::<i32>());

    h1.join().unwrap() + h2.join().unwrap()
}
```

### Rust (Arc<Mutex<T>> for shared mutable state)
```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub fn concurrent_count(items: Vec<i32>) -> i32 {
    let total = Arc::new(Mutex::new(0_i32));
    let handles: Vec<_> = items.into_iter().map(|x| {
        let t = Arc::clone(&total);
        thread::spawn(move || { *t.lock().unwrap() += x; })
    }).collect();
    for h in handles { h.join().unwrap(); }
    *total.lock().unwrap()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Shared reference | GC-managed `'a` | `Arc<T>` (atomic ref count) |
| Thread handle | `'a Domain.t` | `JoinHandle<T>` |
| Shared mutable | `Mutex.t` + GC ref | `Arc<Mutex<T>>` |
| Clone cost | pointer copy (GC) | atomic increment (cheap) |
| Lifetime proof | runtime (GC traces) | compile-time (ownership rules) |

## Key Insights

1. **Explicit vs implicit sharing:** OCaml's GC tracks all references invisibly; Rust requires you to opt in to shared ownership with `Arc::clone`, making the sharing explicit and auditable at the call site.

2. **`Rc<T>` vs `Arc<T>`:** `Rc<T>` uses non-atomic (single-threaded) reference counting and is not `Send`; the compiler rejects sending it across thread boundaries. `Arc<T>` uses atomic operations and is both `Send + Sync`, so the compiler allows it.

3. **Immutability is the default:** `Arc<T>` gives you `&T` — shared immutable access. For mutation you must add `Mutex<T>` (or `RwLock<T>`), spelling out both "this is shared" and "this is mutable" separately. This prevents data races at compile time.

4. **No GC overhead:** `Arc<T>` pays only for the two atomic integers (strong count + weak count) stored next to the value on the heap. There is no stop-the-world pause and no scanning of the entire heap; the value is freed the instant the last `Arc` drops.

5. **`move` closures capture the clone:** Rust's `move` keyword transfers the cloned `Arc` into the thread closure, proving to the borrow checker that the thread owns its own reference and cannot outlive the data.

## When to Use Each Style

**Use `Arc<T>` (immutable):** when multiple threads only need to *read* shared data — config, lookup tables, parsed input. Zero-cost reads after the initial atomic clone.

**Use `Arc<Mutex<T>>`:** when multiple threads need to *write* shared state — counters, accumulators, work queues. The Mutex ensures exclusive access; the Arc ensures the Mutex itself lives long enough.
