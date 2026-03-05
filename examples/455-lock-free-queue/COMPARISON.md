# Lock-Free Queue

## MPSC Pattern

Multiple producers push, single consumer pops.

### Rust
```rust
// Producer (any thread)
let prev = self.head.swap(node, Ordering::AcqRel);
(*prev).next.store(node, Ordering::Release);

// Consumer (single thread only!)
let next = (*tail).next.load(Ordering::Acquire);
*self.tail.get() = next;
```

## Safety
- MPSC: multiple producers OK
- Single consumer only!
- Use crossbeam for MPMC
