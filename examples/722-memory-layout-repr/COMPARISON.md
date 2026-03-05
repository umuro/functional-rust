# OCaml vs Rust: Memory Layout — repr(C), repr(packed), repr(align(N))

## Side-by-Side Code

### OCaml
```ocaml
(* OCaml records are laid out in declaration order; every field occupies
   one word (8 bytes on 64-bit). There is no padding between fields because
   all values are uniformly word-sized. Float record fields are unboxed.
   OCaml has no repr attributes — layout is fixed by the runtime. *)

type compact = { a: int; b: bool; c: int }
(* 3 words = 24 bytes on 64-bit OCaml (each field is one word) *)

type vec4 = { x: float; y: float; z: float; w: float }
(* 4 unboxed floats = 4 words = 32 bytes; OCaml cannot force 16-byte alignment *)

let () =
  let v = { a = 1; b = true; c = 3 } in
  (* Obj.size gives word count, not bytes *)
  assert (Obj.size (Obj.repr v) = 3);
  print_endline "ok"
```

### Rust — repr(C): C-compatible field order and padding
```rust
use std::mem;

// Fields in declaration order; each padded to its natural alignment.
// a(1) + pad(3) + b(4) + c(2) + pad(2) = 12 bytes, align 4
#[repr(C)]
pub struct CLayout {
    pub a: u8,
    pub b: u32,
    pub c: u16,
}

assert_eq!(mem::size_of::<CLayout>(), 12);
assert_eq!(mem::align_of::<CLayout>(), 4);
// Field offsets are guaranteed: a@0, b@4, c@8
```

### Rust — repr(C, packed): wire-format layout, no padding
```rust
// All padding removed; fields are byte-adjacent.
// a(1) + b(4) + c(2) = 7 bytes, align 1
#[repr(C, packed)]
pub struct PackedLayout {
    pub a: u8,
    pub b: u32,
    pub c: u16,
}

assert_eq!(mem::size_of::<PackedLayout>(), 7);

// NEVER take &field on a packed struct — use addr_of! + read_unaligned
let v = PackedLayout { a: 0, b: 42, c: 0 };
let b_val = unsafe { std::ptr::read_unaligned(std::ptr::addr_of!(v.b)) };
assert_eq!(b_val, 42);
```

### Rust — repr(C, align(N)): SIMD and cache-line alignment
```rust
// 16-byte aligned for SSE/NEON SIMD loads
#[repr(C, align(16))]
pub struct SimdVec4 {
    pub x: f32, pub y: f32, pub z: f32, pub w: f32,
}

assert_eq!(mem::size_of::<SimdVec4>(), 16);
assert_eq!(mem::align_of::<SimdVec4>(), 16);

// 64-byte aligned to fill one cache line, preventing false sharing
#[repr(C, align(64))]
pub struct CacheLinePadded { pub counter: u64 }

assert_eq!(mem::size_of::<CacheLinePadded>(), 64);
```

## Type Signatures

| Concept               | OCaml                                     | Rust                                     |
|-----------------------|-------------------------------------------|------------------------------------------|
| Default layout        | Declaration order, word-sized fields      | `repr(Rust)` — compiler reorders freely  |
| C-compatible layout   | No equivalent (ctypes library needed)     | `#[repr(C)]`                             |
| No-padding layout     | No equivalent                             | `#[repr(C, packed)]`                     |
| Forced alignment      | No equivalent                             | `#[repr(C, align(N))]`                   |
| Size inspection       | `Obj.size (Obj.repr v)` (in words)        | `std::mem::size_of::<T>()` (in bytes)    |
| Alignment inspection  | Not exposed                               | `std::mem::align_of::<T>()`              |
| Field offset          | Not exposed                               | `std::ptr::addr_of!(v.field) as usize`   |

## Key Insights

1. **OCaml has no repr attributes.** Every record field is one word (8 bytes on
   64-bit); the runtime imposes a uniform, fixed layout. This makes layout
   inspection simple but removes control. Rust's `repr` attributes give you
   full control at the cost of needing to understand alignment rules.

2. **repr(Rust) vs repr(C) trade-off.** The default `repr(Rust)` lets the
   compiler reorder fields to minimise padding — `{u8, u32, u16}` becomes 8
   bytes instead of 12. `repr(C)` pays the 12-byte cost to guarantee a stable,
   ABI-compatible layout that C code can read without any translation.

3. **repr(packed) eliminates padding but disqualifies references.** Taking
   `&packed_struct.field` is undefined behaviour if the field is unaligned.
   The safe pattern is `addr_of!(h.field)` (raw pointer, no reference) followed
   by `ptr::read_unaligned`. Forgetting this is a silent UB trap with no
   compile-time warning in older Rust; recent clippy versions warn about it.

4. **repr(align(N)) is additive.** It raises the minimum alignment without
   changing the field layout — `#[repr(C, align(16))]` is C order plus forced
   16-byte alignment. Aligning a 64-byte cache-line struct prevents false
   sharing: two cores writing adjacent counters never invalidate each other's
   L1/L2 lines if each counter is in a different `align(64)` struct.

5. **OCaml's uniform word layout avoids the pitfalls of packed structs.**
   Because OCaml's GC always accesses values through aligned pointers, there is
   no concept of an unaligned field. Rust's packed structs are unsafe territory
   precisely because the CPU may trap or silently misread unaligned multi-byte
   values — making this an advanced topic requiring care and unsafe blocks.

## When to Use Each Style

**Use `repr(C)` when:** crossing an FFI boundary, defining a struct whose layout
must match a C header, or writing data that will be read by another language or
process using C ABI conventions (e.g., shared memory, mmap'd files).

**Use `repr(C, packed)` when:** implementing a binary network protocol or file
format where padding bytes are illegal (e.g., Ethernet frames, ELF headers,
custom serialisation with a fixed wire format).

**Use `repr(C, align(N))` when:** writing SIMD kernels that require aligned
loads/stores (SSE: 16, AVX: 32, AVX-512: 64), or isolating per-thread data
structures to different cache lines to prevent false sharing in lock-free code.
