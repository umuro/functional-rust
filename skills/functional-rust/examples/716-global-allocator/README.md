# 716: Custom Global Allocator with #[global_allocator]

**Difficulty:** 5  **Level:** Master

Replace Rust's default system allocator with a custom implementation — for embedded, arena allocation, or memory accounting.

## The Problem This Solves

Every `Box::new`, `Vec::push`, and `String::from` goes through Rust's global allocator. By default, that's the system allocator (glibc `malloc` on Linux, jemalloc on some targets). For most programs, this is fine. For embedded systems, production servers, or diagnostic tooling, you need more control.

`#[global_allocator]` lets you replace the global allocator entirely. Implement `GlobalAlloc` — two methods, `alloc` and `dealloc` — and annotate your static instance. From that point on, every heap allocation in your program goes through your code. You can track live bytes, count allocations, enforce a per-request budget, use a pre-allocated arena, or route all allocation to a region in physical memory you've mapped yourself.

The contract is strict. `alloc` must return a pointer to memory of at least `layout.size()` bytes, aligned to `layout.align()`. `dealloc` must receive the exact same pointer and layout that `alloc` returned. The compiler cannot verify this. You own these invariants completely — which is why `GlobalAlloc` is an `unsafe trait`.

## The Intuition

The global allocator is the lowest layer of Rust's memory model. Every `Box`, `Vec`, `String`, `Arc`, and `HashMap` eventually calls `alloc` and `dealloc`. Replacing it means you intercept every heap allocation in the program. A tracking allocator wraps the system allocator and maintains atomic counters around each call — the same approach profilers use, but at zero external overhead. A bump allocator skips the system allocator entirely: allocation is just a pointer increment, reset is setting the offset to zero.

## How It Works in Rust

```rust
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct TrackingAllocator { inner: System }

static LIVE_BYTES: AtomicUsize = AtomicUsize::new(0);
static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = self.inner.alloc(layout);
        if !ptr.is_null() {
            LIVE_BYTES.fetch_add(layout.size(), Ordering::Relaxed);
            ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.inner.dealloc(ptr, layout);
        LIVE_BYTES.fetch_sub(layout.size(), Ordering::Relaxed);
    }
}

#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator { inner: System };
```

For embedded or `#![no_std]` targets, a bump allocator replaces the `System` inner with a `static mut [u8; N]` slab and a single `AtomicUsize` offset. Allocation: compare-exchange the offset forward. Reset: store zero.

## What This Unlocks

- **Memory accounting**: Track live bytes and allocation counts per subsystem — essential for memory budgeting in game engines, servers, and embedded firmware.
- **Embedded bare-metal**: Provide a fixed-size bump allocator over a statically-linked buffer — enables `alloc` crate without a system heap.
- **Arena allocation**: Build a per-request allocator that frees everything at once by resetting the offset — eliminates fragmentation and `O(n)` deallocation overhead.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Custom allocator | Not supported (GC-managed heap) | `impl GlobalAlloc` + `#[global_allocator]` |
| Allocation tracking | `Gc.stat()` for GC stats only | Atomic counters in your allocator |
| Fixed backing store | Not possible in standard OCaml | `static mut [u8; N]` + bump pointer |
| Thread-safety | GC handles it | `AtomicUsize` for counters |
| Dealloc contract | GC finalizes | Must match exact pointer + layout from `alloc` |
| Per-allocation cost | GC amortizes | Custom: can be as low as one `fetch_add` |
