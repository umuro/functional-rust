# 731: Tiered Memory Strategy

**Difficulty:** 4  **Level:** Expert

Route allocations to the fastest tier that fits: stack for tiny, arena bump-allocator for medium, heap for large and long-lived.

## The Problem This Solves

Not all allocations are equal. A 64-byte scratch buffer for a frame computation has a completely different lifecycle from a multi-megabyte image buffer. Routing both through the global heap allocator is technically correct but wasteful: the allocator pays lock contention, free-list traversal, and fragmentation costs even for the short-lived scratch buffer that could have lived on the stack.

Real-time and high-throughput systems use tiered memory strategies. Tier 1 (stack): compile-time-known, short-lived, small — free at frame exit. Tier 2 (arena/bump): medium-sized, same lifetime as a batch/request — allocated with a single pointer bump, freed all at once by resetting the offset to zero. Tier 3 (heap): large, long-lived, individually freed — the global allocator.

A bump allocator is the simplest arena: maintain a byte slab and an offset. Allocation is `O(1)` — bump the offset by `n` bytes and return a pointer to the old offset. Deallocation is not supported for individual allocations; instead, reset the entire arena when the epoch ends. This eliminates fragmentation and per-object allocator overhead for burst-allocation workloads like request processing, parsing, or per-frame game logic.

## The Intuition

Think of the bump allocator as a notepad. When you need scratch space, write at the current position and advance the pen. When you're done with the whole page (the request ends, the frame ends), tear off the sheet and start fresh. You never erase individual words — you throw the whole sheet away. This is safe because all the allocations on that sheet have the same lifetime.

The `'arena` lifetime in Rust encodes this: references into the arena cannot outlive the arena itself. The borrow checker enforces the lifetime at compile time — you cannot accidentally hold a reference into a reset arena.

## How It Works in Rust

```rust
use std::cell::Cell;

struct BumpArena<const CAP: usize> {
    slab: [u8; CAP],
    offset: Cell<usize>,
}

impl<const CAP: usize> BumpArena<CAP> {
    const fn new() -> Self {
        BumpArena { slab: [0u8; CAP], offset: Cell::new(0) }
    }

    fn alloc(&self, n: usize) -> Option<&mut [u8]> {
        let start = self.offset.get();
        let end = start.checked_add(n)?;
        if end > CAP { return None; }
        self.offset.set(end);
        unsafe {
            // SAFETY: bounds checked above; slab valid for 'self lifetime.
            Some(std::slice::from_raw_parts_mut(
                (self.slab.as_ptr() as *mut u8).add(start), n
            ))
        }
    }

    fn reset(&self) { self.offset.set(0); }  // O(1) free-all
}

fn process_request(arena: &BumpArena<4096>) {
    // Tier 1: stack — tiny scratch values
    let counter: u32 = 0;

    // Tier 2: arena — medium, request-scoped
    let buf = arena.alloc(256).expect("arena full");
    buf.fill(0);

    // arena.reset() called after the request — all tier-2 memory freed at once.
}
```

## What This Unlocks

- **Latency-free burst allocation**: Parse a JSON request by bumping the arena offset for each token — no lock contention, no free-list traversal, no fragmentation.
- **Predictable memory usage**: Each arena has a fixed capacity. You know at compile time how much memory a request can use.
- **Per-frame game or simulation loops**: Allocate scratch data into the frame arena, reset at end of frame — zero allocator overhead in the hot loop.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Stack allocation | Limited (unboxed values only) | Full control — any `[T; N]` is on stack |
| Arena allocator | Custom via `Bigarray` or C FFI | Native bump allocator with const-generic capacity |
| Heap fallback | GC manages automatically | Explicit `Box` / `Vec` |
| Lifetime enforcement | Runtime (GC roots) | Compile-time (`'arena` lifetime) |
| Free-all reset | GC major collection | `arena.reset()` — single store instruction |
| Memory safety | GC guarantees | Borrow checker + `unsafe` in `alloc` impl |
