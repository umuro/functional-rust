# OCaml vs Rust: RwLock Pattern

## Read-Write Lock Semantics

### OCaml (no native RwLock — uses Mutex)
```ocaml
let config = ref [("host","localhost")]
let mutex = Mutex.create ()

let read_config k =
  Mutex.lock mutex;
  let v = List.assoc_opt k !config in
  Mutex.unlock mutex; v

let write_config k v =
  Mutex.lock mutex;
  config := (k,v) :: List.filter (fun (a,_) -> a<>k) !config;
  Mutex.unlock mutex
```

### Rust
```rust
let cfg: Arc<RwLock<HashMap<&str, &str>>> = 
    Arc::new(RwLock::new(HashMap::new()));

// Multiple readers — simultaneous, no blocking
let guard = cfg.read().unwrap();
let value = guard.get("host");

// Exclusive write
let mut guard = cfg.write().unwrap();
guard.insert("host", "example.com");
```

## Key Differences

| Feature | OCaml | Rust |
|---------|-------|------|
| RwLock available | No (stdlib) | Yes (`std::sync::RwLock`) |
| Multiple readers | Blocked (Mutex only) | Concurrent (shared guard) |
| Guard types | Single type | `RwLockReadGuard` / `RwLockWriteGuard` |
| Unlock | Manual | Automatic (RAII) |

## Concurrent Readers

### OCaml
```ocaml
(* All readers serialize on the single mutex *)
let readers = List.init 4 (fun _ ->
  Thread.create (fun () ->
    Mutex.lock mutex;  (* blocks even for read-only *)
    let _ = List.assoc "host" !config in
    Mutex.unlock mutex
  ) ()
)
```

### Rust
```rust
// All readers run concurrently — no blocking
let readers: Vec<_> = (0..4).map(|_| {
    let c = Arc::clone(&cfg);
    thread::spawn(move || {
        let guard = c.read().unwrap();  // shared — many OK
        let _ = guard.get("host");
    })
}).collect();
```

## Writer Priority

### Rust
```rust
// Writer waits for all current readers to release
let writer = thread::spawn(move || {
    let mut guard = cfg.write().unwrap();  // blocks until readers done
    guard.insert("host", "newhost");
});
// Once writer has lock, new readers block
```

## When to Use RwLock vs Mutex

| Use Case | Recommendation |
|----------|----------------|
| Reads >> Writes | `RwLock` — parallel reads |
| Balanced read/write | `Mutex` — simpler, less overhead |
| Short critical sections | `Mutex` — RwLock overhead not worth it |
| Long reads, rare writes | `RwLock` — maximizes read throughput |
