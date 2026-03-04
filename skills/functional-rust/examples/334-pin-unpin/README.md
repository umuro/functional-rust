# 334: Pin and Unpin

**Difficulty:** 5  **Level:** Master

`Pin<P>` prevents a value from moving in memory — required for self-referential futures that async state machines create.

## The Problem This Solves

Rust normally allows moving any value anywhere. `let x = y` moves `y`; passing a value to a function moves it. This is fine for most types — but not for self-referential types: structs that contain a pointer to their own field. After a move, the pointer points to the old location. That's a dangling pointer and undefined behavior.

Why does this matter for async? When you write `async { let x = 1; foo().await; use(x); }`, the compiler generates a state machine struct that, across the `await` point, stores both `x` and potentially a reference to `x` inside the same struct. That's self-referential. Moving this struct after it starts polling would corrupt the internal pointer.

`Pin<P>` is a wrapper that statically prevents the value from being moved (unless it implements `Unpin`). `Unpin` is an auto-trait: all "normal" types implement it (`i32`, `String`, `Vec`), meaning they're safe to move. Only types that opt out (by containing `PhantomPinned`) lose `Unpin` and must be accessed through `Pin`.

## The Intuition

Imagine you've labeled a box and written the box's address on a post-it stuck to the box. If you move the box to a new shelf, the post-it address is now wrong. `Pin` says: "this box is bolted to the shelf — you can read from it, write to it, but you cannot move it."

Most types in Rust (`i32`, `String`, `Vec`) don't have internal pointers and don't care about being moved — they're `Unpin`. Async state machines and explicitly self-referential structs are the exceptions.

## How It Works in Rust

```rust
use std::marker::PhantomPinned;
use std::pin::Pin;

struct SelfRef {
    data: String,
    ptr: *const u8,  // points into self.data — invalid after a move!
    _pin: PhantomPinned,  // removes the auto-Unpin impl
}

impl SelfRef {
    fn new(s: &str) -> Pin<Box<Self>> {
        let mut b = Box::new(Self {
            data: s.to_string(),
            ptr: std::ptr::null(),
            _pin: PhantomPinned,
        });
        b.ptr = b.data.as_ptr();  // now points into the heap allocation
        // Safety: we never move the value out of the Box after this
        unsafe { Pin::new_unchecked(b) }
    }

    fn ptr_valid(self: Pin<&Self>) -> bool {
        self.ptr == self.data.as_ptr()  // would fail after a move
    }
}

// Normal types are Unpin — Pin::into_inner() is safe
let mut n = Normal { x: 42 };
let p = Pin::new(&mut n);
let inner = Pin::into_inner(p);  // fine: Normal: Unpin
```

`PhantomPinned` is a zero-sized marker that removes `Unpin`, preventing `Pin::into_inner` and safe movement. Methods that take `Pin<&Self>` (not `&Self`) signal "this requires the pinning guarantee."

## What This Unlocks

- **Custom `Future` implementations** — correctly implementing `poll` for self-referential state machines.
- **Intrusive data structures** — linked lists or trees where nodes hold pointers to neighbors in the same allocation.
- **Async generators and coroutines** — any structure that holds a reference into itself across yield points.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Self-referential types | Handled by GC (no move semantics) | Requires `Pin<P>` + `PhantomPinned` |
| Move safety | All values are heap-allocated, GC updates pointers | Stack values can move; programmer must opt in to Pin |
| Unpin | Not applicable | Auto-trait on all safe-to-move types |
| Async futures | Lwt/Effect continuations are always heap-allocated | Rust futures are stack by default, pinned when polled |
