📖 **[View on hightechmind.io →](https://hightechmind.io/rust/363-arena-allocation)**

---

# 363: Arena Allocation

## Problem Statement

General-purpose allocators (malloc/jemalloc) have overhead: each allocation needs bookkeeping metadata, thread-local allocation caches, and potentially lock contention. For workloads that allocate thousands of small objects and free them all at once — AST nodes during parsing, temporary nodes in a graph algorithm, frame data in a game loop — arena allocation (bump allocation) is dramatically faster. The arena pre-allocates one large block and serves allocations by simply advancing a pointer. "Freeing" individual objects is a no-op; the entire arena resets in O(1). This pattern powers programming language parsers, game engines, and database query planners.

## Learning Outcomes

- Implement a bump allocator arena using a pre-allocated `Vec<u8>` and `Cell<usize>` offset
- Handle alignment by rounding the offset up to the required alignment boundary
- Use `Cell<usize>` for interior mutability so `&self` methods can mutate the offset
- Count allocations with a second `Cell<usize>` for diagnostics
- Reset the arena in O(1) by setting offset back to zero
- Understand that arena allocation trades allocation/free speed for batch-free semantics

## Rust Application

```rust
use std::cell::Cell;

pub struct Arena {
    data: Vec<u8>,
    offset: Cell<usize>,
    allocations: Cell<usize>,
}

impl Arena {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![0u8; capacity],
            offset: Cell::new(0),
            allocations: Cell::new(0),
        }
    }

    pub fn alloc_bytes(&self, size: usize, align: usize) -> Option<usize> {
        let offset = self.offset.get();
        let aligned = (offset + align - 1) & !(align - 1); // round up
        let new_offset = aligned + size;
        if new_offset > self.data.len() { return None; }
        self.offset.set(new_offset);
        self.allocations.set(self.allocations.get() + 1);
        Some(aligned)
    }

    pub fn allocated(&self) -> usize { self.offset.get() }

    pub fn reset(&mut self) {
        self.offset.set(0);
        self.allocations.set(0);
    }
}
```

`(offset + align - 1) & !(align - 1)` is the standard power-of-2 alignment calculation. For `align = 8`: if offset is 5, `(5 + 7) & !7 = 12 & 0b...11111000 = 8`. The `Cell<usize>` (not `RefCell`) is used because `usize` is `Copy` — `Cell` works for `Copy` types without borrowing overhead.

## OCaml Approach

OCaml's GC serves as a kind of arena — you can allocate objects freely and the GC handles collection. For explicit arena semantics, `Bigarray` or `Bytes` with a ref-based offset:

```ocaml
type arena = {
  data: bytes;
  mutable offset: int;
}

let create capacity = { data = Bytes.create capacity; offset = 0 }

let alloc a size align =
  let aligned = (a.offset + align - 1) land (lnot (align - 1)) in
  if aligned + size > Bytes.length a.data then None
  else begin
    a.offset <- aligned + size;
    Some aligned
  end

let reset a = a.offset <- 0
```

In practice, OCaml's generational GC already provides fast allocation for short-lived objects (minor heap bump allocation). Explicit arenas are less common in OCaml than in Rust, where every allocation is explicit and arena-vs-per-object is a meaningful choice.

## Key Differences

| Aspect | Rust arena | OCaml GC / manual arena |
|--------|-----------|-------------------------|
| Allocation cost | O(1) pointer bump | O(1) GC minor heap (usually) |
| Deallocation | O(1) arena reset (all at once) | GC-managed individually |
| Safety | `unsafe` needed for typed access | Safe (GC handles lifetime) |
| Alignment | Manual calculation required | GC handles alignment |
| Use case | Parsers, compilers, games | Rarely needed; GC does it |

## Exercises

1. **Typed allocation**: Using `unsafe`, implement `alloc<T>(&self) -> Option<*mut T>` that returns a properly aligned pointer into the arena's buffer for type `T` using `std::mem::size_of::<T>()` and `std::mem::align_of::<T>()`.
2. **AST arena**: Build a simple expression parser where all AST nodes are allocated into a single arena; reset the arena after each parse to reuse memory for the next input.
3. **Fragmentation analysis**: Allocate a mix of 1-byte, 4-byte, and 8-byte values and measure internal fragmentation (wasted alignment padding) as a percentage of total allocated bytes.
