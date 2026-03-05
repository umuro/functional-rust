# OCaml vs Rust: Semaphore Async

## Semaphore Implementation

**OCaml:**
```ocaml
let acquire sem =
  Mutex.lock sem.m;
  while !(sem.count) = 0 do
    Condition.wait sem.cv sem.m
  done;
  decr sem.count;
  Mutex.unlock sem.m
```

**Rust:**
```rust
fn acquire(&self) {
    let mut c = self.count.lock().unwrap();
    while *c == 0 {
        c = self.cond.wait(c).unwrap();
    }
    *c -= 1;
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Stdlib support | No | `tokio::sync::Semaphore` |
| RAII release | No | `Permit` with `Drop` |
| Condition wait | `Condition.wait cv m` | `cond.wait(guard)` |
| Atomic ops | Not needed | `AtomicUsize` for stats |
