# 553: Self-Referential Structs (Pin)

**Difficulty:** 5  **Level:** Advanced

You can't store a `Vec` and a reference to its elements in the same struct. The borrow checker forbids it — because moving the struct would invalidate the reference. `Pin` is the solution: it promises the struct won't move, making internal pointers safe.

## The Problem This Solves

The fundamental impossibility:

```rust
struct SelfRef {
    data: Vec<i32>,
    ptr: *const i32,  // intended to point at data[0]
}

// If you move this struct, `data` moves to a new address — but `ptr` still points to the old address.
// After the move: ptr is a dangling pointer.
let s1 = SelfRef { data: vec![1,2,3], ptr: /* &data[0] */ };
let s2 = s1;  // MOVE — data is now at a different address, ptr is invalid
```

Rust allows moving any value at any time — `let s2 = s1` is valid. A self-referential struct breaks this. `Pin<Box<T>>` solves it by preventing the struct from ever being moved after the pointer is set up.

This isn't just academic: async functions are self-referential. An `async fn` that stores a reference across an `.await` point is a self-referential state machine. `Pin` is why async Rust works.

## The Intuition

`Pin<P>` is a wrapper around a pointer `P` (like `Box<T>` or `&mut T`) that promises: "the value behind this pointer will not move." Once pinned, you can safely set up internal pointers — they'll stay valid.

The `Unpin` trait is the escape hatch: if `T: Unpin`, then `Pin<Box<T>>` gives you no guarantees (you can still move T). Most types are `Unpin`. To opt out — to say "don't let this be moved" — you add `PhantomPinned` to your struct.

The rule: **once a `!Unpin` value is pinned, it must never be moved.** `Pin` enforces this by making the safe API not expose `&mut T` (only `Pin<&mut T>`).

## How It Works in Rust

**The problem demonstrated:**

```rust
// Safe alternative first — use indices, not pointers:
struct SafeSelfRef {
    data: Vec<String>,
    current_index: usize,  // "reference" by index — valid after any move
}

// Works great — can be moved freely, current_index always valid
let mut r = SafeSelfRef { data: vec!["a".to_string(), "b".to_string()], current_index: 0 };
let moved = r; // fine — index is just a usize
```

**When you truly need a self-referential pointer:**

```rust
use std::pin::Pin;
use std::marker::PhantomPinned;

struct SelfRef {
    value: String,
    self_ptr: *const String,  // will point at `value`
    _pin: PhantomPinned,      // marks as !Unpin — prevents moves
}

impl SelfRef {
    fn new(s: &str) -> Pin<Box<Self>> {
        // Step 1: Box and pin the struct
        let mut boxed = Box::pin(SelfRef {
            value: s.to_string(),
            self_ptr: std::ptr::null(), // placeholder — will be set below
            _pin: PhantomPinned,
        });

        // Step 2: Now it's pinned — safe to set up the internal pointer
        let ptr: *const String = &boxed.value;
        unsafe {
            let this = boxed.as_mut().get_unchecked_mut();
            this.self_ptr = ptr; // points at value — safe because value won't move
        }

        boxed
    }

    fn get_value(self: Pin<&Self>) -> &str {
        unsafe { &self.get_ref().value }
    }

    fn get_via_ptr(self: Pin<&Self>) -> &str {
        // self_ptr is valid because the struct can't move
        unsafe { &*self.get_ref().self_ptr }
    }
}

let pinned = SelfRef::new("Hello, Pin!");
println!("{}", pinned.as_ref().get_value());   // "Hello, Pin!"
println!("{}", pinned.as_ref().get_via_ptr()); // same — via raw pointer
// let moved = *pinned; // ERROR — Pin<Box<T>> where T: !Unpin prevents this
```

**Why async uses Pin:**

```rust
async fn example() {
    let x = 42;
    let r = &x;          // r references x within the same frame
    do_something().await; // suspension point — the frame must stay in place!
    println!("{}", r);   // r must still be valid after resumption
}
// The compiler generates a !Unpin state machine that Pin keeps in place
```

## What This Unlocks

- **Async state machines** — understanding `Pin` explains why async functions are `!Unpin`, why `Future::poll` takes `Pin<&mut Self>`, and why you can't just `Box<dyn Future>` without careful thought.
- **Intrusive linked lists** — nodes that point to neighbors can be self-referential if pinned. Used in OS kernels and some high-performance data structures.
- **Safe arena patterns** — pin a slab allocator, then store references into it within the same arena. The arena won't move, so the references stay valid.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Self-referential values | GC handles — nodes can reference themselves freely | Forbidden without `Pin` — moving the struct invalidates internal pointers |
| Moving values | GC never "moves" in the visible sense | Any `let b = a` moves — self-referential structs break this |
| Async state machines | Continuation-passing or effect handlers | Self-referential async state machines — `Pin` prevents movement |
| Internal pointers | GC tracks all — always valid | `Pin<Box<T>>` + `PhantomPinned` — explicit promise not to move |
| Safe alternative | N/A | Use indices instead of pointers — movable, no unsafe needed |
