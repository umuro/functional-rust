# OCaml vs Rust: Async Sink

## Sink Structure

**OCaml:**
```ocaml
type 'a sink = { mutable buf: 'a list; cap: int; flush_fn: 'a list -> unit }

let send s x =
  s.buf <- x :: s.buf;
  if List.length s.buf >= s.cap then (s.flush_fn (List.rev s.buf); s.buf <- [])
```

**Rust:**
```rust
struct BatchSink<T> {
    buffer: VecDeque<T>,
    capacity: usize,
    flushed_batches: Vec<Vec<T>>,
}

impl<T: Clone> BatchSink<T> {
    fn send(&mut self, item: T) -> Result<(), String> {
        self.buffer.push_back(item);
        if self.buffer.len() >= self.capacity { self.flush()?; }
        Ok(())
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Buffer type | List (prepend, reverse) | VecDeque (O(1) both ends) |
| Flush callback | First-class function | Stored or called directly |
| Error handling | Unit/exceptions | `Result` type |
| Ownership | GC handles | Explicit drain/move |
