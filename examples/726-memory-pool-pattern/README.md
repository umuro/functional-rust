# Memory Pool Pattern
**Difficulty:** ⭐  
**Category:** Functional Programming  



> **Functional Rust** · [hightechmind.io](https://hightechmind.io)

## Problem Statement

General-purpose allocators (`malloc`/`free`) handle arbitrary allocation sizes and
lifetimes, but pay a price: lock contention, fragmentation, and header overhead per
allocation. In workloads that allocate many objects of the same type in bursts and free
them all at once—game frame allocation, parser arenas, database query plans, HTTP
request contexts—a specialized allocator can outperform `malloc` by 10–100×.

A **memory pool** (or **bump arena**) reserves a large contiguous slab up front and
satisfies individual allocations by advancing a pointer. Deallocation is `O(1)` (or
free-all-at-once). The tradeoff: individual items cannot be freed independently; the
entire pool is reset or dropped together. The pattern appears in Linux's slab allocator,
Nginx's per-request pools, LLVM's `BumpPtrAllocator`, and Apache Arrow's buffer pools.

## Learning Outcomes

- Implement a typed pool allocator using `Vec<T>` with pre-reserved capacity
- Implement a bump-pointer arena using `Cell<usize>` for interior mutability
- Understand when pools outperform `Box<T>` (same-size same-lifetime objects)
- Recognize the typed pool vs arena tradeoff (typed safety vs flexibility)
- Use the `bumpalo` crate for production-quality bump allocation

## Rust Application

```rust
use std::cell::Cell;
use std::ptr::NonNull;

// Typed pool: fixed capacity, O(1) alloc/dealloc via free-list
pub struct Pool<T, const CAP: usize> {
    storage: Box<[std::mem::MaybeUninit<T>; CAP]>,
    free:    Vec<usize>,   // indices of free slots
    len:     usize,
}

impl<T, const CAP: usize> Pool<T, CAP> {
    pub fn new() -> Self {
        Pool {
            storage: Box::new(std::array::from_fn(|_| std::mem::MaybeUninit::uninit())),
            free:    (0..CAP).collect(),
            len:     0,
        }
    }

    pub fn alloc(&mut self, val: T) -> Option<usize> {
        let idx = self.free.pop()?;
        self.storage[idx].write(val);
        self.len += 1;
        Some(idx)
    }

    pub fn get(&self, idx: usize) -> &T {
        // SAFETY: idx was returned by alloc and not yet freed
        unsafe { self.storage[idx].assume_init_ref() }
    }

    pub fn free_slot(&mut self, idx: usize) {
        // SAFETY: idx is valid and was previously allocated
        unsafe { self.storage[idx].assume_init_drop() };
        self.free.push(idx);
        self.len -= 1;
    }
}

// Bump arena: fast allocation, free-all semantics
pub struct Arena {
    buf:    Vec<u8>,
    offset: Cell<usize>,
}

impl Arena {
    pub fn new(capacity: usize) -> Self {
        Arena {
            buf:    vec![0u8; capacity],
            offset: Cell::new(0),
        }
    }

    pub fn alloc<T>(&self, val: T) -> &T {
        let align = std::mem::align_of::<T>();
        let size  = std::mem::size_of::<T>();
        let off   = (self.offset.get() + align - 1) & !(align - 1);
        assert!(off + size <= self.buf.len(), "arena exhausted");
        self.offset.set(off + size);
        // SAFETY: aligned, within buf, lifetime tied to &self
        let ptr = unsafe {
            (self.buf.as_ptr().add(off) as *mut T).as_mut().unwrap()
        };
        *ptr = val;
        ptr
    }

    pub fn reset(&self) {
        self.offset.set(0);
    }
}
```

The `Cell<usize>` offset allows shared `&Arena` references to allocate without
`&mut` — safe because allocations never overlap and the arena is reset atomically.

## OCaml Approach

OCaml's GC is itself a form of arena allocator: the minor heap is a bump-pointer arena
that is evacuated to the major heap after each minor GC. User-space arenas are unusual
in OCaml but available via the `Memory_block` or `Bigarray` approach:

```ocaml
(* OCaml: use Bytes as a bump buffer *)
type arena = {
  buf    : bytes;
  mutable offset : int;
}

let make_arena size = { buf = Bytes.create size; offset = 0 }

let arena_alloc_int arena v =
  let off = arena.offset in
  Bytes.set_int64_be arena.buf off (Int64.of_int v);
  arena.offset <- off + 8;
  off   (* return offset as "pointer" *)
```

The real analog in OCaml is `Obj.obj` tricks or C stubs. The GC handles lifetime
automatically, which eliminates the need for explicit arenas in most OCaml programs.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Allocation strategy | Manual pool/arena | Minor heap bump (automatic) |
| Free semantics | Explicit (typed pool) or reset (arena) | GC; no explicit free |
| Fixed-capacity pool | `Pool<T, CAP>` with `MaybeUninit` | Not idiomatic |
| Interior mutability | `Cell<usize>` for arena offset | `ref` fields |
| Production library | `bumpalo`, `typed-arena` crates | OCaml 5 local allocators |

## Exercises

1. Benchmark `Pool<u64, 1024>` vs `Box<u64>` allocation for 100,000 objects using
   `criterion`. Measure throughput and fragmentation.
2. Implement `Arena::alloc_slice<T>(&self, vals: &[T]) -> &[T]` that copies a slice
   into the arena and returns a borrowed reference with aligned layout.
3. Use the `bumpalo` crate to implement a per-request HTTP header parser that
   allocates all header `&str` borrows into a `Bump` arena and resets between requests.
4. Add a `Pool` free-list using a singly-linked list of indices stored inside the
   free slots themselves (`MaybeUninit<usize>`), eliminating the `Vec<usize>` overhead.
5. Implement thread-local arenas (one `Arena` per thread via `thread_local!`) for a
   parallel workload and compare throughput vs a single `Mutex<Arena>`.
