# Epoch-Based GC

## Concept
- Global epoch counter
- Threads pin to current epoch
- Garbage deferred to current epoch
- When all threads leave epoch, garbage can be freed

### Rust Pattern
```rust
let _guard = epoch.pin();  // Enter critical section
// ... access data safely ...
// Guard drops, thread unpins

epoch.defer(old_value);    // Defer for later collection
epoch.try_advance();       // Maybe advance and collect
```

## vs Hazard Pointers
| Feature | Hazard Pointers | Epoch GC |
|---------|-----------------|----------|
| Space | Per-pointer | Per-epoch |
| Latency | Immediate check | Batch reclaim |
| Complexity | Higher | Lower |
