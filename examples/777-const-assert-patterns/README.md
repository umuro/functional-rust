# 777: Compile-Time Assertions with const

**Difficulty:** 3  **Level:** Intermediate

Use `const { assert!(...) }` to catch invalid invariants at compile time — zero runtime overhead, build fails instead.

## The Problem This Solves

Some invariants are best caught at compile time. If your wire protocol requires `PacketHeader` to be exactly 16 bytes, you want the build to fail — not a runtime panic — when someone adds a field and breaks that guarantee. If a const generic parameter must be a power of two (for bitwise modulo to work), you want the compiler to tell you immediately, not after your ring buffer silently produces wrong results.

`const` assertions accomplish this: the expression is evaluated during compilation, and if it's false, compilation fails with your custom message. Unlike `#[static_assert]` in C++, Rust's `const { assert!(...) }` is first-class syntax that works anywhere a `const` block is valid. Combined with `std::mem::size_of` and `std::mem::align_of`, you can express ABI and layout guarantees that survive any future refactoring.

This pattern is used extensively in embedded Rust (ensuring structs have the right layout for hardware registers), networking (wire format compatibility), and performance-critical code (alignment for SIMD, power-of-two for fast modulo).

## The Intuition

`const _: () = assert!(condition, "message");` evaluates `condition` at compile time. If false, the build fails with "message". Use it for: struct size/alignment guarantees (`size_of::<T>() == N`), const parameter validation (`N.is_power_of_two()`), domain invariants (protocol version > 0), and interop requirements (FFI struct layout). The `const fn` variant lets you validate at the point of instantiation in generic code.

## How It Works in Rust

```rust
use std::mem::{size_of, align_of};

// Basic compile-time size assertions
const _: () = assert!(size_of::<u64>() == 8, "u64 must be 8 bytes");
const _: () = assert!(usize::BITS >= 32,     "need at least 32-bit usize");

// Wire format: this struct MUST be exactly 16 bytes
#[repr(C)]
pub struct PacketHeader {
    pub magic:   u32,   // 4 bytes
    pub version: u8,    //   1 byte
    pub flags:   u8,    //   1 byte
    pub length:  u16,   //   2 bytes
    pub seq:     u64,   //   8 bytes = 16 total
}
const _: () = assert!(
    size_of::<PacketHeader>() == 16,
    "PacketHeader must be exactly 16 bytes for wire compatibility"
);

// const fn: validate at point of use in generic context
const fn must_be_power_of_two(n: usize) -> usize {
    assert!(n.is_power_of_two(), "N must be a power of two");
    n
}
const CACHE_SIZE: usize = must_be_power_of_two(1024);  // ok
// const BAD: usize = must_be_power_of_two(1000);       // compile error!

// Generic struct with inline validation
pub struct BoundedVec<T, const MAX: usize> {
    data: Vec<T>,
}
impl<T, const MAX: usize> BoundedVec<T, MAX> {
    const _CHECK: () = assert!(MAX > 0 && MAX <= 1_000_000, "MAX out of range");

    pub fn new() -> Self {
        let _ = Self::_CHECK;  // trigger the assert when new() is called in const context
        Self { data: Vec::new() }
    }
}
```

`#[repr(C)]` is required for layout guarantees — without it, Rust may reorder fields for optimization. The `const _: ()` pattern uses the unit type and the wildcard `_` to create anonymous compile-time checks without polluting the namespace.

## What This Unlocks

- **`repr(C)` + size assertion** — the combination of controlled layout and compile-time size verification is the standard pattern for FFI structs and network wire formats.
- **`const fn` as validated constructors** — functions that panic in const context produce compile errors, not runtime panics; use this for const generic type-level validation.
- **Zero-cost invariants** — const assertions have no runtime overhead; the check is completely eliminated after compilation; you get correctness guarantees for free.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Compile-time assertion | No direct equivalent | `const _: () = assert!(...)` — first-class |
| Struct layout control | `[@unboxed]`, `[@noalloc]` | `#[repr(C)]`, `#[repr(packed)]`, `#[repr(align(N))]` |
| Size of type | `Obj.size` (runtime) | `std::mem::size_of::<T>()` — const, compile-time |
| Generic const validation | Type-level tricks | `const _CHECK: () = assert!(...)` in impl block |
