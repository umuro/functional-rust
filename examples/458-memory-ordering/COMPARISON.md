# Memory Ordering and Fences

## Fence vs Atomic Ordering

```rust
// Equivalent pairs:
data.store(42, Ordering::Release);
// vs
data.store(42, Ordering::Relaxed);
fence(Ordering::Release);
```

## When to Use Fences
- Multiple atomics need same ordering
- Conditional synchronization
- Interfacing with C/hardware

## Ordering Rules
- Store: Relaxed, Release, SeqCst
- Load: Relaxed, Acquire, SeqCst  
- RMW: All orderings
