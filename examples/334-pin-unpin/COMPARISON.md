# OCaml vs Rust: Pin and Unpin

## Self-Referential Struct

**OCaml (no pinning needed):**
```ocaml
type self_ref = {
  data: string;
  mutable ptr: int option;  (* Index, not pointer *)
}
(* GC handles memory, no concern about moves *)
```

**Rust:**
```rust
struct SelfRef {
    data: String,
    ptr: *const u8,  // Raw pointer into data
    _pin: PhantomPinned,  // Prevents Unpin
}

impl SelfRef {
    fn new(s: &str) -> Pin<Box<Self>> {
        let mut b = Box::new(Self { ... });
        b.ptr = b.data.as_ptr();
        unsafe { Pin::new_unchecked(b) }
    }
}
```

## Key Differences

| Aspect | OCaml | Rust |
|--------|-------|------|
| Memory management | GC | Manual / ownership |
| Self-ref safety | Automatic | Requires `Pin` |
| Move semantics | Copy by default | Move by default |
| `Unpin` equivalent | N/A | Auto-trait |
