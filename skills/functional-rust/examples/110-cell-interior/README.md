# 110: Cell\<T\> — Interior Mutability for Copy Types

**Difficulty:** 2  **Level:** Intermediate

`Cell<T>` lets you mutate a value through a shared reference — for `Copy` types — by moving values in and out instead of handing out references.

## The Problem This Solves

Rust's borrow checker enforces a strict rule: if you have a shared reference (`&T`), the value is read-only. This is correct almost all the time — but not always. Sometimes you have a logically immutable data structure where one field needs to be updated as a side effect: a cache that stores the last computed result, a hit counter on an otherwise-immutable record, lazy initialization of a derived value.

Without interior mutability, you'd be forced to make the entire struct mutable (`&mut T`), even though you only want to mutate one field. That removes the ability to share the struct through multiple `&T` references simultaneously. The whole thing becomes awkward.

`Cell<T>` solves this for `Copy` types. Instead of handing out references to the inner value (which would violate borrow rules), `Cell` lets you only copy values in and out. The `set` method replaces the value; `get` copies it out. No references to the interior are ever created, so there's no aliasing — the borrow checker's rule is respected, not worked around.

## The Intuition

`Cell<T>` sidesteps the borrow checker for `Copy` types by never handing out references to the interior — you can only swap values in and out, making mutation through a shared reference safe by design.

## How It Works in Rust

```rust
use std::cell::Cell;

struct HitCounter {
    label: String,
    hits: Cell<u32>, // mutable through &self, no &mut needed
}

impl HitCounter {
    fn new(label: &str) -> Self {
        HitCounter {
            label: label.to_string(),
            hits: Cell::new(0),
        }
    }
    
    fn record_hit(&self) { // note: &self, not &mut self
        self.hits.set(self.hits.get() + 1);
    }
    
    fn count(&self) -> u32 {
        self.hits.get()
    }
}

fn demo() {
    let counter = HitCounter::new("homepage");
    
    // Multiple shared references — normally can't mutate through these
    let r1 = &counter;
    let r2 = &counter;
    
    r1.record_hit(); // mutation through shared ref — Cell makes this safe
    r2.record_hit();
    
    println!("{}: {} hits", counter.label, counter.count()); // 2 hits
}

// Cell<T> with flag types
use std::cell::Cell as Flag;
struct LazyNode {
    value: i32,
    computed: Cell<bool>,
}

impl LazyNode {
    fn mark_computed(&self) {
        self.computed.set(true);
    }
    
    fn is_computed(&self) -> bool {
        self.computed.get()
    }
}

// Cell<T> only works for Copy types
// Cell<String> would not work — String is not Copy
// For non-Copy types, use RefCell<T> (example 111)
```

## What This Unlocks

- **Selective mutability** — mark only the fields that need interior mutability with `Cell`, keeping the rest of the struct immutable through shared references.
- **Hit counters and caches** — logically read-only operations that update internal bookkeeping are expressible without infecting the entire struct with `&mut`.
- **No runtime cost** — `Cell<T>` has no locking, no reference counting; it's a zero-overhead wrapper. For `Copy` types there are no references to manage.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Mutable field in immutable record | `mutable` field + `ref` | `Cell<T>` for `Copy` types |
| OCaml ref equivalent | `ref` (always available) | `Cell<T>` (explicit, only for `Copy`) |
| Mutation through shared access | Freely available | Requires `Cell` or `RefCell` |
| Thread safety | Not applicable (single-threaded model common) | `Cell` is NOT thread-safe; use `AtomicU32` etc. for threads |
| Non-Copy types | All types are GC-managed | Use `RefCell<T>` instead (example 111) |
