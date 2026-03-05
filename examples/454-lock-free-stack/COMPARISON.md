# Lock-Free Stack

## Compare-and-Swap Pattern

### Rust
```rust
loop {
    let head = self.head.load(Ordering::Relaxed);
    (*new_node).next = head;
    if self.head.compare_exchange_weak(
        head, new_node,
        Ordering::Release, Ordering::Relaxed
    ).is_ok() {
        break;
    }
}
```

## Key Points
- CAS loop retries on contention
- `compare_exchange_weak` may spuriously fail (but faster)
- Watch out for ABA problem
