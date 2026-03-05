# OCaml vs Rust: Future Trait

## Core State Machine

**OCaml (manual):**
```ocaml
type 'a state = Pending of (unit -> 'a state) | Ready of 'a

let rec run = function
  | Ready v -> v
  | Pending f -> run (f ())
```

**Rust:**
```rust
impl Future for DelayedValue {
    type Output = i32;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.remaining == 0 { Poll::Ready(self.value) }
        else { self.remaining -= 1; cx.waker().wake_by_ref(); Poll::Pending }
    }
}
```

## Delayed Value Creation

**OCaml:**
```ocaml
let delayed_value n steps =
  let rec loop i =
    if i = 0 then Ready n
    else Pending (fun () -> loop (i-1))
  in loop steps
```

**Rust:**
```rust
impl DelayedValue {
    fn new(value: i32, polls: u32) -> Self {
        Self { value, remaining: polls }
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| State representation | ADT variant | `Poll` enum |
| Continuation | Closure `unit -> 'a state` | `Waker` callback |
| Executor | Recursive function | `block_on` loop |
| Memory safety | GC handles | `Pin` prevents moves |
| Zero-cost | Closure allocation | No allocation in poll |
