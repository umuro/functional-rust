**Difficulty:** ⭐  
**Category:** Functional Programming  

**Difficulty:** ⭐  

[rwlock-pattern on hightechmind.io](https://hightechmind.io/posts/functional-rust/rwlock-pattern)

---

## Problem Statement

Demonstrate Rust's `RwLock<T>` — a readers-writer lock that allows multiple concurrent readers OR a single exclusive writer. Show that many threads can hold read locks simultaneously, and that a write lock excludes all readers. Implement a read-heavy shared configuration pattern.

## Learning Outcomes

- Use `RwLock::read()` to acquire a shared read guard — multiple threads can hold these simultaneously
- Use `RwLock::write()` to acquire an exclusive write guard — blocks until all readers release
- Combine with `Arc<RwLock<T>>` for shared ownership across threads
- Understand when `RwLock` is preferred over `Mutex`: read-heavy workloads where concurrent reads improve throughput
- Understand the write-starvation risk: many readers can delay writers indefinitely

## Rust Application

```rust
fn concurrent_readers() -> Vec<i32> {
    let data = Arc::new(RwLock::new(42i32));
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let guard = data.read().unwrap();
                *guard  // all 5 can hold read lock simultaneously
            })
        })
        .collect();
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

fn write_then_read() -> i32 {
    let data = Arc::new(RwLock::new(0i32));
    {
        let mut guard = data.write().unwrap();  // exclusive
        *guard = 100;
        // guard drops here — write lock released
    }
    let guard = data.read().unwrap();
    *guard  // = 100
}

// Shared config: many readers, occasional writer
struct SharedConfig {
    inner: Arc<RwLock<HashMap<String, String>>>,
}

impl SharedConfig {
    fn get(&self, key: &str) -> Option<String> {
        self.inner.read().unwrap().get(key).cloned()
    }
    fn set(&self, key: String, value: String) {
        self.inner.write().unwrap().insert(key, value);
    }
}
```

Five `read()` guards can exist simultaneously — the threads run truly in parallel. A `write()` call blocks until all current readers release their guards. Once the writer holds the lock, new readers also block.

The `SharedConfig` pattern is common for application configuration: reads are frequent (every request), writes are rare (config reload). `RwLock` provides better throughput than `Mutex` for this workload.

## OCaml Approach

```ocaml
(* OCaml 5.0+ Stdlib has RwLock *)
let rwlock = RwLock.create 42

let concurrent_readers () =
  let threads = List.init 5 (fun _ ->
    Thread.create (fun () ->
      RwLock.read_lock rwlock;
      let v = RwLock.read_value rwlock in
      RwLock.read_unlock rwlock;
      v
    ) ()
  ) in
  List.map Thread.join threads

(* Pre-5.0: use Mutex with read tracking *)
let read_lock m readers = Mutex.lock m; incr readers; Mutex.unlock m
let read_unlock m readers cond =
  Mutex.lock m; decr readers;
  if !readers = 0 then Condition.signal cond;
  Mutex.unlock m
```

OCaml 5.0+ added `RwLock` to the standard library. Earlier versions required manual implementation using `Mutex` + `Condition` + reader count — exactly how Rust's `RwLock` is implemented internally.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Read guard | `data.read().unwrap()` — RAII | `RwLock.read_lock` + manual unlock |
| Write guard | `data.write().unwrap()` — RAII | `RwLock.write_lock` + manual unlock |
| Starvation | Writer-prefer or reader-prefer (OS-dependent) | Same OS-level behavior |
| Poison | Read/write guards handle poisoned locks | No equivalent |

`RwLock` is a trade-off: better throughput for read-heavy workloads, but higher complexity than `Mutex`. If writes are frequent, `Mutex` is simpler and may perform equally well.

## Exercises

1. Measure the throughput difference between `Mutex` and `RwLock` for 8 readers and 1 writer with a 99:1 read-write ratio.
2. Implement write-starvation: start 10 long-running read threads, then try to acquire a write lock — observe the delay.
3. Implement `upgrade_from_read_to_write` — release the read lock and acquire the write lock atomically (spoiler: not possible in std; discuss why).
4. Build a `CachingConfig` where reads check an in-memory `HashMap` (under read lock) and miss falls through to a "database" (uses write lock to update cache).
5. Implement a version counter alongside the data: increment a `AtomicUsize` on every write, read it under the read lock to detect stale views.
