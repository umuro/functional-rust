# OCaml vs Rust: Send and Sync — Compile-Time Thread Safety

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml has no Send/Sync concepts; the programmer manually ensures safety.
   OCaml 5 uses Mutex for shared mutable state — same idea, no type enforcement. *)
let parallel_sum data =
  let total = ref 0 in
  let m = Mutex.create () in
  let n = List.length data / 2 in
  let left, right = (* split at n *) ... in
  let t = Thread.create (fun () ->
    let s = List.fold_left ( + ) 0 left in
    Mutex.lock m; total := !total + s; Mutex.unlock m) () in
  let s = List.fold_left ( + ) 0 right in
  Mutex.lock m; total := !total + s; Mutex.unlock m;
  Thread.join t;
  !total
```

### Rust (idiomatic — Arc<Mutex<T>>)
```rust
use std::sync::{Arc, Mutex};
use std::thread;

pub fn parallel_sum(numbers: Vec<i32>) -> i32 {
    let total = Arc::new(Mutex::new(0i32));
    let mid = numbers.len() / 2;
    let (left, right) = numbers.split_at(mid);
    let left = left.to_vec();
    let right = right.to_vec();

    let total_clone = Arc::clone(&total);
    let handle = thread::spawn(move || {
        let partial: i32 = left.iter().sum();
        *total_clone.lock().unwrap() += partial;
    });

    let partial: i32 = right.iter().sum();
    *total.lock().unwrap() += partial;

    handle.join().unwrap();
    *total.lock().unwrap()
}
```

### Rust (functional — channel scatter/gather)
```rust
use std::sync::mpsc;
use std::thread;

pub fn channel_sum(numbers: Vec<i32>) -> i32 {
    let (tx, rx) = mpsc::channel::<i32>();
    let mid = numbers.len() / 2;
    let (left, right) = numbers.split_at(mid);
    let (left, right) = (left.to_vec(), right.to_vec());

    let tx2 = tx.clone();
    thread::spawn(move || tx2.send(left.iter().sum()).unwrap());
    tx.send(right.iter().sum()).unwrap();

    rx.iter().take(2).sum()
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Thread-safe shared ownership | `ref` + `Mutex` (by convention) | `Arc<Mutex<T>>` (enforced by type) |
| Thread safety marker | none — manual discipline | `Send`, `Sync` auto-traits |
| Spawn constraint | none (runtime crash on violation) | `F: FnOnce() -> R + Send + 'static` |
| Channel sender | `Event.channel` / `Queue` | `mpsc::Sender<T>` where `T: Send` |
| Shared immutable ref | `ref` (mutable) or `let` binding | `Arc<T>` where `T: Sync` |

## Key Insights

1. **Compile-time vs runtime enforcement:** OCaml's type system has no notion of thread safety — nothing stops you from sharing a non-thread-safe value across threads. Rust's `Send`/`Sync` auto-traits make unsafe sharing a *compile error*, eliminating an entire class of data-race bugs.

2. **Auto-derivation:** You rarely write `unsafe impl Send` yourself. The compiler automatically derives `Send` for any struct whose fields are all `Send`, and `Sync` for any struct whose fields are all `Sync`. The work happens at the type-composition level, not at the call site.

3. **`Arc` vs `Rc`:** `Rc<T>` uses a non-atomic reference count and is intentionally `!Send + !Sync` — the compiler will refuse to let it cross a thread boundary. `Arc<T>` uses atomics and is `Send + Sync` when `T: Send + Sync`. The naming difference (`A` = atomic) is a deliberate design signal.

4. **`Mutex<T>` owns its data:** Unlike OCaml's `Mutex.create ()` which is separate from the data it protects, Rust's `Mutex<T>` *wraps* `T`. You cannot access `T` without going through the lock. This makes the invariant structurally enforced rather than conventional.

5. **Channel ownership transfers:** `mpsc::Sender<T>` requires `T: Send`, encoding at the type level that values flowing through channels cross thread boundaries. The functional scatter/gather pattern maps cleanly to this: produce partial results in parallel, collect in the main thread — no shared mutable state needed.

## When to Use Each Style

**Use `Arc<Mutex<T>>` when:** multiple threads need to read *and* write a shared value, and the mutation pattern is irregular (not just produce-then-consume).

**Use channels (`mpsc`) when:** work is partitioned upfront and results flow in one direction — spawn workers, collect results. This is the functional style: closer to OCaml's Domain + Event pattern and avoids shared mutable state entirely.
