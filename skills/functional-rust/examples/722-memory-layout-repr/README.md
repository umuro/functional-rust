# 722: Memory Layout — repr(C), repr(packed), repr(align(N))

**Difficulty:** 4  **Level:** Expert

Control struct field order, padding, and alignment for FFI, serialisation, and SIMD work.

## The Problem This Solves

Rust's default struct layout (`repr(Rust)`) makes no guarantees about field order or padding. The compiler may reorder fields to minimise padding, and that reordering can change between compiler versions. For application code that never crosses an ABI boundary, this is fine — the compiler does the right thing. For FFI, network protocols, file formats, or SIMD buffers, you need byte-exact layout control.

Three attributes give you that control. `#[repr(C)]` matches C's layout rules: fields in declaration order, each field padded to its natural alignment, the struct padded to the alignment of its largest field. `#[repr(packed)]` strips all padding — every field is adjacent in memory, regardless of alignment. `#[repr(align(N))]` forces the struct to be aligned to at least N bytes, which is required for SIMD types and some MMIO register maps.

Combining them is allowed — `#[repr(C, packed)]` gives you C field order with no padding, which is common for wire-format structs. But mixing `repr(packed)` with references is an instant UB trap: taking a reference to an unaligned field violates Rust's aliasing rules.

## The Intuition

Memory layout is like packing a suitcase with rigid dividers. The default `repr(Rust)` lets the compiler move things around to fill gaps efficiently. `repr(C)` puts in the same dividers as the C compiler — both sides of the FFI boundary see the same gaps in the same places. `repr(packed)` removes all dividers — maximum density, but you might need special unaligned reads. `repr(align(16))` is like choosing a suitcase whose handle is always in the right position for the luggage carousel (SIMD lane alignment).

## How It Works in Rust

```rust
// repr(Rust) — default, do NOT use for FFI.
struct DefaultLayout { a: u8, b: u32, c: u16 }
// Likely 8 bytes: compiler reorders to b(4)+c(2)+a(1)+pad(1)

// repr(C) — C-compatible, field order preserved.
#[repr(C)]
pub struct CLayout { pub a: u8, pub b: u32, pub c: u16 }
// Exactly 12 bytes: a(1)+pad(3)+b(4)+c(2)+pad(2)

// repr(packed) — no padding, potentially unaligned.
#[repr(C, packed)]
pub struct WireFrame { pub a: u8, pub b: u32, pub c: u16 }
// Exactly 7 bytes: a(1)+b(4)+c(2), no padding

// repr(align(16)) — for SIMD and cache-line alignment.
#[repr(C, align(16))]
pub struct SimdVec4 { pub x: f32, pub y: f32, pub z: f32, pub w: f32 }
// 16 bytes, 16-byte aligned — maps to XMM register directly.
```

Inspect at compile time: `std::mem::size_of::<T>()`, `std::mem::align_of::<T>()`. Use `std::mem::offset_of!(T, field)` (stabilised in 1.77) to get field byte offsets.

**Critical rule**: Never take a reference to a field of a `repr(packed)` struct. Use `ptr::read_unaligned` / `ptr::write_unaligned` instead.

## What This Unlocks

- **Correct FFI structs**: Match C struct layouts exactly — essential for `ioctl` buffers, Win32 API, and any library that passes structs across the ABI boundary.
- **Zero-copy serialisation**: `repr(C, packed)` structs can be cast directly to byte slices for network transmission or file I/O without any marshalling code.
- **SIMD alignment**: `repr(align(16))` or `repr(align(32))` ensures your data lands on the alignment boundary SIMD intrinsics require — no runtime alignment check needed.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Default struct layout | Fields in declaration order (GC-managed) | Compiler may reorder (unspecified) |
| C-compatible layout | `ctypes` library | `#[repr(C)]` |
| Packed layout | Not in stdlib | `#[repr(packed)]` |
| Forced alignment | Not available | `#[repr(align(N))]` |
| Inspect layout at compile time | Not available | `size_of!`, `align_of!`, `offset_of!` |
| Reference to packed field | Not applicable | UB — use `read_unaligned` instead |
