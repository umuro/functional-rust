# OCaml vs Rust: Custom Global Allocator

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml: closest concept is tracking GC statistics.
   OCaml does not expose a custom-allocator hook in stdlib;
   memory is managed entirely by the GC. *)

let total_allocated = ref 0

let tracked_alloc n =
  total_allocated := !total_allocated + n;
  Bytes.create n   (* goes through the GC *)

let tracked_free _buf n =
  total_allocated := !total_allocated - n

let () =
  let buf = tracked_alloc 64 in
  Printf.printf "allocated: %d bytes live\n" !total_allocated;
  tracked_free buf 64;
  Printf.printf "after free: %d bytes live\n" !total_allocated
```

### Rust (idiomatic — tracking wrapper)
```rust
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct TrackingAllocator { inner: System }

static LIVE_BYTES: AtomicUsize = AtomicUsize::new(0);
static ALLOC_COUNT: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for TrackingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { self.inner.alloc(layout) };
        if !ptr.is_null() {
            LIVE_BYTES.fetch_add(layout.size(), Ordering::Relaxed);
            ALLOC_COUNT.fetch_add(1, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { self.inner.dealloc(ptr, layout) };
        LIVE_BYTES.fetch_sub(layout.size(), Ordering::Relaxed);
    }
}

#[global_allocator]
static ALLOCATOR: TrackingAllocator = TrackingAllocator { inner: System };
```

### Rust (functional/arena — bump allocator)
```rust
pub struct BumpAllocator<const N: usize> {
    buf: [u8; N],
    cursor: usize,
}

impl<const N: usize> BumpAllocator<N> {
    pub const fn new() -> Self { Self { buf: [0u8; N], cursor: 0 } }

    pub fn alloc_bytes(&mut self, size: usize, align: usize) -> Option<&mut [u8]> {
        let aligned = self.cursor.wrapping_add(align - 1) & !(align - 1);
        let end = aligned.checked_add(size)?;
        if end > N { return None; }
        self.cursor = end;
        Some(&mut self.buf[aligned..end])
    }

    pub fn reset(&mut self) {
        self.cursor = 0;
        self.buf.iter_mut().for_each(|b| *b = 0);
    }
}
```

## Type Signatures

| Concept | OCaml | Rust |
|---------|-------|------|
| Allocation unit | `Bytes.t` (GC-managed) | `*mut u8` (raw pointer, unsafe) |
| Allocation tracking | `ref int` (mutable reference) | `AtomicUsize` (lock-free atomic) |
| Memory layout | Implicit (GC decides) | Explicit `Layout { size, align }` |
| Custom allocator hook | Not available in stdlib | `unsafe impl GlobalAlloc` + `#[global_allocator]` |
| Arena / bump style | Manual with `Bytes.create` | `BumpAllocator<const N>` with const generics |

## Key Insights

1. **No GC escape hatch in OCaml.** OCaml's memory management is entirely in the runtime's hands. You can observe GC statistics (`Gc.stat()`) and tune collection parameters, but you cannot intercept individual allocations or replace the allocator. Rust exposes the full allocation pathway as a safe interface you implement.

2. **`unsafe` is quarantined, not absent.** `GlobalAlloc` is an `unsafe trait` because the contract — pointer validity, layout matching between `alloc` and `dealloc` — cannot be machine-checked. Rust forces the implementor to declare they accept responsibility with `unsafe impl`, keeping the unsafety localized to the allocator crate rather than spread across callsites.

3. **Atomics vs mutable state.** OCaml's single-threaded GC allows plain `ref int` for counters. Rust's allocator runs on any thread, so `LIVE_BYTES` must be an `AtomicUsize`. `Ordering::Relaxed` suffices for statistics that don't guard other memory operations.

4. **Const generics enable zero-heap arenas.** `BumpAllocator<const N: usize>` lives entirely on the stack (or in a static). There is no heap allocation at all — ideal for embedded systems or deterministic latency scenarios. OCaml's `Bytes.create` always allocates on the GC heap.

5. **One global allocator per binary.** Rust enforces this at link time: placing `#[global_allocator]` on two statics in the same binary is a linker error. OCaml has no equivalent constraint because you cannot replace its allocator at all.

## When to Use Each Style

**Use `TrackingAllocator` (wrapping style) when:** you want observability — counting allocations, enforcing per-request byte budgets, or logging allocation patterns in production — without changing allocation semantics or performance characteristics.

**Use `BumpAllocator` (arena style) when:** you need deterministic latency, zero fragmentation, or embedded targets with no OS allocator. Suited for request-scoped scratch memory: allocate freely during a request, then `reset()` at the boundary.
