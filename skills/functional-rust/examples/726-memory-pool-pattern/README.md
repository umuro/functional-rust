# 726: Memory Pool / Bump Allocator Pattern

**Difficulty:** 4  **Level:** Expert

Allocate and deallocate objects in O(1) by managing your own memory — no `malloc`, no fragmentation.

## The Problem This Solves

The general-purpose allocator (`malloc`/`free`) is powerful but has overhead: it must handle arbitrary allocation sizes, maintain free-lists for different size classes, prevent fragmentation, and be thread-safe. For workloads that create and destroy many objects of the same size — game entities, parser AST nodes, database row handles, network connection objects — this overhead adds up.

A memory pool solves this by pre-allocating a fixed block of same-sized slots and managing them with a simple free-list. Allocation is: pop a slot index from the free-list — O(1). Deallocation is: push the index back — O(1). No fragmentation because all objects are the same size. No thread-safety overhead for single-threaded use. The allocator doesn't have to search for a fit; it always has one ready.

A bump allocator (arena) goes further: allocation is a single pointer increment — sub-nanosecond. Deallocation is: drop the entire arena at once. This suits workloads where objects are created together and destroyed together (a parser's working set, a per-request scratch space).

unsafe is a tool, not a crutch — use only when safe Rust genuinely can't express the pattern.

## The Intuition

**Fixed-size pool**: Picture a hotel with 64 rooms. Check-in hands you a room number (alloc returns an index). Check-out puts the room back on the front desk's list (dealloc pushes the index back). Every room is the same size. No searching, no splitting, no coalescing.

**Bump arena**: Picture a notepad. Writing something tears off a page (bump the pointer, give out a slice). When the meeting is over, throw away the entire notepad (drop the arena). Individual "pages" can't be reclaimed — the whole pad goes at once.

## How It Works in Rust

```rust
use std::mem::MaybeUninit;

// ── Fixed-size typed pool ─────────────────────────────────────────────────
pub struct Pool<T, const CAP: usize> {
    slots:     Box<[MaybeUninit<T>; CAP]>,
    free_head: Option<usize>,
    next_free: [usize; CAP],
    live:      usize,
}

impl<T, const CAP: usize> Pool<T, CAP> {
    pub fn alloc(&mut self, val: T) -> Option<usize> {
        let idx = self.free_head?;              // O(1): pop free-list head
        let next = self.next_free[idx];
        self.free_head = if next < CAP { Some(next) } else { None };
        self.slots[idx].write(val);             // initialise the slot
        self.live += 1;
        Some(idx)
    }

    /// # Safety
    /// `idx` must have been returned by `alloc` and not yet freed.
    pub unsafe fn dealloc(&mut self, idx: usize) {
        // SAFETY: Caller guarantees slot is live and won't be accessed again.
        unsafe { self.slots[idx].assume_init_drop(); }
        self.next_free[idx] = self.free_head.unwrap_or(CAP);
        self.free_head = Some(idx);             // O(1): push back to free-list
        self.live -= 1;
    }
}

// ── Bump allocator (arena) ────────────────────────────────────────────────
pub struct BumpArena {
    memory: Vec<u8>,
    offset: usize,
}

impl BumpArena {
    pub fn new(capacity: usize) -> Self {
        Self { memory: vec![0u8; capacity], offset: 0 }
    }

    pub fn alloc_bytes(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        let aligned = (self.offset + align - 1) & !(align - 1);  // align up
        if aligned + size > self.memory.len() { return None; }
        let ptr = unsafe { self.memory.as_mut_ptr().add(aligned) };
        self.offset = aligned + size;
        Some(ptr)
        // No individual free — entire arena resets at once.
    }

    pub fn reset(&mut self) { self.offset = 0; }  // O(1) bulk deallocation
}
```

## What This Unlocks

- **Game engines** — per-frame arena allocators reset every frame (60 Hz), eliminating GC pauses and fragmentation for temporary objects (particle effects, collision data, AI pathfinding scratch space).
- **Parsers and compilers** — AST nodes are allocated in a pool, all freed at once after code generation; `rustc` uses arena allocators internally for this reason.
- **Network servers** — per-request arenas handle the lifetime of headers, parsed URLs, and response buffers without individual `malloc`/`free` calls per field.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Custom allocator | Not idiomatic; GC handles allocation | `Pool<T>` / `BumpArena` as library types |
| Allocation cost | GC amortised; unpredictable latency | O(1) guaranteed — pointer bump or free-list pop |
| Deallocation | GC traces and collects | Manual (pool) or bulk-drop (arena) |
| Memory safety | GC prevents use-after-free | Manual: `dealloc` marks slot dead; caller must not reuse index |
| Fragmentation | GC compacts (in some runtimes) | Pool: none (same-size slots); arena: none (no free holes) |
