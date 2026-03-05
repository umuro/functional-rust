# 334: Pin and Unpin

**Difficulty:** 5  **Level:** Master

`Pin<P>` prevents a value from moving in memory — required for self-referential futures that async state machines create.

## The Problem This Solves

Self-referential types contain pointers to their own fields. After a move, those pointers are dangling. Async state machines generated from `async fn` can be self-referential across `.await` points.

`Pin<P>` statically prevents movement (unless the type implements `Unpin`). Most types (`i32`, `String`, `Vec`) implement `Unpin` automatically.

## The Intuition

Imagine labeling a box with its own address. Move the box, and the label is wrong. `Pin` says: "this box is bolted to the shelf — you cannot move it."

## How It Works in Rust

```rust
use std::marker::PhantomPinned;

struct SelfRef {
    data: String,
    ptr: *const u8,  // points into self.data — invalid after move!
    _pin: PhantomPinned,  // removes auto-Unpin impl
}

impl SelfRef {
    fn new(s: &str) -> Pin<Box<Self>> {
        let mut b = Box::new(Self {
            data: s.to_string(),
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        });
        b.ptr = b.data.as_ptr();
        unsafe { Pin::new_unchecked(b) }
    }

    fn ptr_valid(self: Pin<&Self>) -> bool {
        self.ptr == self.data.as_ptr()
    }
}
```

`PhantomPinned` removes `Unpin`, preventing `Pin::into_inner` and safe movement.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Self-ref types | GC handles | Requires `Pin` + `PhantomPinned` |
| Move safety | All heap-allocated | Stack values can move |
| Unpin | N/A | Auto-trait for safe-to-move types |
