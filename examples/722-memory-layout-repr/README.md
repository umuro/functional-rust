# Memory Layout and `repr` Attributes
**Difficulty:** ⭐  
**Category:** Functional Programming  


> **Functional Rust** · [hightechmind.io](https://hightechmind.io)

## Problem Statement

Rust gives the compiler freedom to reorder struct fields for optimal alignment, which
minimizes padding and can improve cache utilization. This is great for pure-Rust code
but catastrophic when interoperating with C, reading binary file formats, mapping
hardware registers, or sending structs over a network. The `repr` attribute family
provides precise, guaranteed layout control:

- `#[repr(C)]` — C-compatible layout, fields in declaration order with C alignment rules
- `#[repr(packed)]` — remove all padding; fields may be unaligned (UB to take references)
- `#[repr(align(N))]` — force struct alignment to N bytes (useful for SIMD, false sharing)
- `#[repr(transparent)]` — single-field wrapper; identical ABI to the inner type
- `#[repr(u8/u16/…)]` — control enum discriminant size for FFI or compact storage

Without understanding these attributes, FFI code silently misreads fields, file parsers
corrupt data, and "optimization" structs actually increase memory usage due to padding.

## Learning Outcomes

- Understand Rust's default field reordering and why it exists
- Apply `repr(C)` for safe C interop and binary format compatibility
- Use `repr(packed)` for compact network/file structs with proper pointer safety
- Use `repr(align(N))` to prevent false sharing or satisfy SIMD alignment requirements
- Verify layout decisions with `std::mem::size_of`, `offset_of!`, and `memoffset`

## Rust Application

```rust
use std::mem;

// Default: compiler may reorder fields to minimize padding
struct DefaultLayout {
    a: u8,   // 1 byte + 7 bytes padding (for f64 alignment)
    b: f64,  // 8 bytes
    c: u32,  // 4 bytes + 4 bytes padding
}
// size_of::<DefaultLayout>() == 24 (or 16 after reorder)

// C layout: fields in declaration order, C alignment rules
#[repr(C)]
struct CLayout {
    a: u8,   // 1 byte + 7 padding
    b: f64,  // 8 bytes
    c: u32,  // 4 bytes + 4 padding
}
// size_of::<CLayout>() == 24 always

// Packed: no padding; unsafe to take references to fields
#[repr(C, packed)]
struct PackedLayout {
    a: u8,
    b: f64,
    c: u32,
}
// size_of::<PackedLayout>() == 13

// Aligned: force 64-byte alignment to occupy a full cache line
#[repr(align(64))]
struct CacheAligned {
    data: [u8; 32],
}
// Prevents false sharing between threads

// Transparent: zero-cost newtype wrapper
#[repr(transparent)]
struct Meters(f64);  // identical ABI to f64

fn print_layout() {
    println!("Default: {}", mem::size_of::<DefaultLayout>());
    println!("C:       {}", mem::size_of::<CLayout>());
    println!("Packed:  {}", mem::size_of::<PackedLayout>());
    println!("Aligned: {}", mem::size_of::<CacheAligned>());
}
```

Packed structs require `ptr::read_unaligned` when reading fields by pointer;
taking a reference to a packed field is a compile error in recent Rust.

## OCaml Approach

OCaml does not provide layout attributes. The runtime uses a uniform boxed
representation: every record on the heap starts with a header word and fields are
stored in declaration order with word alignment. For C interop, OCaml uses
`Ctypes` or `Bigarray` to describe C struct layouts explicitly:

```ocaml
open Ctypes
open Foreign

(* Describe C struct layout manually *)
let c_layout =
  let s = structure "CLayout" in
  let _a = field s "a" uint8_t in
  let _b = field s "b" double in
  let _c = field s "c" uint32_t in
  seal s;
  s
```

This is inherently more verbose than `repr(C)` and requires external libraries.

## Key Differences

| Aspect | Rust | OCaml |
|---|---|---|
| Default layout | Compiler-optimized (may reorder) | Word-aligned, declaration order |
| C interop layout | `#[repr(C)]` | `Ctypes` library, manual |
| Packed structs | `#[repr(packed)]` | Not available natively |
| Cache alignment | `#[repr(align(N))]` | Not available |
| Newtype ABI | `#[repr(transparent)]` | No equivalent; always boxed |
| Layout verification | `mem::size_of`, `offset_of!` | `Ctypes.offsetof` |

## Exercises

1. Define a struct with fields `u8`, `u64`, `u16`, `u32` in that order. Measure
   `size_of` with default layout vs `repr(C)` vs `repr(packed)`. Draw the memory map.
2. Implement a `#[repr(C)]` struct matching a Linux `timespec` (`tv_sec: i64`, `tv_nsec: i64`)
   and call `clock_gettime` via FFI to read the real-time clock.
3. Create `#[repr(align(64))] struct AtomicCounter(AtomicU64)` and demonstrate that
   two counters in adjacent memory locations no longer share a cache line (measure with
   `perf stat -e cache-misses` under contention).
4. Write a binary packet parser that reads a `#[repr(C, packed)]` header from a `&[u8]`
   slice using `ptr::read_unaligned` and verify correctness with a known byte sequence.
5. Use the `memoffset` crate's `offset_of!` macro to assert the byte offsets of all
   fields in your `repr(C)` struct match the expected values.
