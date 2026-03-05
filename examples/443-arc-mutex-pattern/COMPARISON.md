# OCaml vs Rust: Arc<Mutex<T>>

## Shared Counter Pattern

### OCaml
```ocaml
let counter = ref 0
let mutex   = Mutex.create ()

let () =
  let threads = List.init 10 (fun _ ->
    Thread.create (fun () ->
      for _ = 1 to 100 do
        Mutex.lock mutex;
        incr counter;
        Mutex.unlock mutex
      done) ()
  ) in
  List.iter Thread.join threads;
  Printf.printf "Counter = %d\n" !counter
```

### Rust
```rust
let counter = Arc::new(Mutex::new(0u64));

let handles: Vec<_> = (0..10).map(|_| {
    let c = Arc::clone(&counter);
    thread::spawn(move || {
        for _ in 0..100 {
            *c.lock().unwrap() += 1;
        }
    })
}).collect();

for h in handles { h.join().unwrap(); }
println!("Counter: {}", *counter.lock().unwrap());
```

## Key Differences

| Feature | OCaml | Rust |
|---------|-------|------|
| Data + Lock | Separate (`ref` + `Mutex.t`) | Unified (`Mutex<T>` wraps data) |
| Forget to lock | Possible (data accessible without lock) | Impossible (data inside mutex) |
| Lock acquisition | `Mutex.lock m` | `m.lock().unwrap()` |
| Unlock | Manual `Mutex.unlock m` | Automatic (guard drops) |
| Shared ownership | GC | `Arc::clone(&arc)` |
| Error on held lock | Blocks | Blocks (or `try_lock` → `Err`) |

## Lock Guard RAII

### OCaml
```ocaml
(* Manual unlock required — easy to forget on error path *)
Mutex.lock mutex;
(* do work *)
Mutex.unlock mutex
```

### Rust
```rust
{
    let mut guard = mutex.lock().unwrap();
    *guard += 1;
} // guard dropped → lock released automatically
// Even on panic, Drop runs and releases the lock
```

## Shared Collections

### OCaml
```ocaml
let log = ref []
let mutex = Mutex.create ()

let add_log msg =
  Mutex.lock mutex;
  log := msg :: !log;
  Mutex.unlock mutex
```

### Rust
```rust
let log: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

let log_clone = Arc::clone(&log);
thread::spawn(move || {
    log_clone.lock().unwrap().push("message".into());
});
```

## Poisoning (Rust-specific)

```rust
// If a thread panics while holding the lock, mutex becomes "poisoned"
let mutex = Arc::new(Mutex::new(0));
let m = Arc::clone(&mutex);

let _ = thread::spawn(move || {
    let _guard = m.lock().unwrap();
    panic!("boom");
}).join();

// Subsequent locks return Err(PoisonError)
match mutex.lock() {
    Ok(guard) => println!("got {}", *guard),
    Err(poisoned) => {
        // Can still recover the data
        let guard = poisoned.into_inner();
        println!("recovered: {}", *guard);
    }
}
```

OCaml has no equivalent — exceptions in threads propagate differently.
