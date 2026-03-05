📖 **[View on hightechmind.io →](https://hightechmind.io/rust/342-arc-mutex-pattern)**

---

# 342: Arc<Mutex<T>> Pattern

Thread-safe shared mutable state.

```rust
let counter = Arc::new(Mutex::new(0));
let c = Arc::clone(&counter);
thread::spawn(move || { *c.lock().unwrap() += 1; });
```
