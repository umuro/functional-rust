# OCaml vs Rust: Waker and Context

## Promise/Resolver Pattern

**OCaml (Lwt):**
```ocaml
let (promise, resolver) = Lwt.wait () in
Lwt.wakeup resolver 42
```

**Rust:**
```rust
let (fut, resolver) = make_future();
resolver.fulfill(42);
```

## Future Implementation

**OCaml:** Internal to Lwt library, uses callbacks.

**Rust:**
```rust
impl Future for ExternalFuture {
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<i32> {
        let mut s = self.state.lock().unwrap();
        if let Some(v) = s.value { Poll::Ready(v) }
        else { s.waker = Some(cx.waker().clone()); Poll::Pending }
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Wake mechanism | `Lwt.wakeup` | `waker.wake()` |
| Suspension | `Lwt.wait()` | `Poll::Pending` |
| Executor notify | Internal | Explicit waker |
| Shared state | Mutable ref | `Arc<Mutex<T>>` |
