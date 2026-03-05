# Memory Ordering

## Ordering Levels

| Ordering | Use Case |
|----------|----------|
| Relaxed | Counters, stats |
| Acquire | Consumer sees producer's writes |
| Release | Producer's writes visible to consumer |
| AcqRel | Read-modify-write |
| SeqCst | Total ordering, simplest |

## Rust
```rust
// Relaxed - cheapest
counter.fetch_add(1, Ordering::Relaxed);

// Release-Acquire pair
data.store(42, Ordering::Relaxed);
flag.store(true, Ordering::Release);
// ... other thread ...
while !flag.load(Ordering::Acquire) {}
let v = data.load(Ordering::Relaxed); // sees 42
```
