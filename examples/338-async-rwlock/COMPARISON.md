# OCaml vs Rust: Async RwLock

## Read-Write Lock

**OCaml:** No stdlib RwLock. Use Mutex or Lwt_rwlock.

**Rust:**
```rust
struct SharedDb { data: RwLock<HashMap<String, i32>> }

fn read(&self, k: &str) -> Option<i32> {
    self.data.read().unwrap().get(k).copied()
}

fn write(&self, k: &str, v: i32) {
    self.data.write().unwrap().insert(k.to_string(), v);
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| RwLock in stdlib | No | Yes |
| Concurrent reads | Requires Lwt_rwlock | Built-in with `read()` |
| Exclusive write | Manual | `write()` blocks readers |
| Data wrapping | Separate | RwLock wraps HashMap |
