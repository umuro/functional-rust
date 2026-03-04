# 707: `std::mem::transmute` — Reinterpreting Bytes

**Difficulty:** 4  **Level:** Expert

Reinterpret a value's bits as a different type — the nuclear option of unsafe Rust.

## The Problem This Solves

Sometimes you genuinely need to look at the same bits through a different lens: inspect the IEEE-754 bit pattern of a float, reinterpret a `&str` fat pointer as `&[u8]`, or work with a C library that passes type-erased void pointers. Safe Rust has no way to do this directly — the type system prevents you from pretending a `f32` is a `u32`.

`mem::transmute<T, U>` tells the compiler: "Take these bits, call them type `U` instead of type `T`, and don't generate any conversion code." It is the most dangerous function in the standard library. Misuse causes undefined behaviour immediately — not a panic, not a crash you can catch, but silent corruption or security vulnerabilities. Almost every use case has a safer, named alternative. Reach for `transmute` only after confirming no safer alternative exists.

unsafe is a tool, not a crutch — use only when safe Rust genuinely can't express the pattern.

## The Intuition

Transmute is bitwise reinterpretation. The bits in memory stay exactly the same; only the type label changes. The compiler enforces one compile-time rule: `size_of::<T>() == size_of::<U>()`. Everything else — alignment, validity invariants, aliasing rules, lifetime correctness — is your responsibility.

The canonical safer alternatives:
- Float bits → integer: `f32::to_bits()` / `f32::from_bits()`
- `&str` → `&[u8]`: `.as_bytes()`
- Lifetime extension: use arena allocators or `Rc<T>` instead
- Type erasure: `Box<dyn Any>` or `NonNull<c_void>`

If a safe alternative exists, use it. The safe API documents intent, compiles to the same code, and doesn't require an `unsafe` block.

## How It Works in Rust

```rust
use std::mem;

// ── Safe alternative is always preferred ────────────────────────────────
fn f32_to_u32_safe(f: f32) -> u32 { f.to_bits() }  // use this

// ── Transmute: same result, higher risk ─────────────────────────────────
fn f32_to_u32_transmute(f: f32) -> u32 {
    unsafe {
        // SAFETY: f32 and u32 both have size 4 and align 4 on all targets;
        // every bit pattern of u32 is a valid u32.
        mem::transmute::<f32, u32>(f)
    }
}

// ── Size mismatch is a compile-time error ────────────────────────────────
// let _: u32 = unsafe { mem::transmute::<u8, u32>(0u8) };  // ERROR

// ── Lifetime extension — extremely dangerous ────────────────────────────
/// # Safety
/// Caller must guarantee the referent is valid for the entire lifetime 'b.
unsafe fn extend_lifetime<'a, 'b, T>(r: &'a T) -> &'b T {
    // SAFETY: Caller contract — almost always a design smell.
    // Prefer arena allocators (bumpalo) or Rc/Arc instead.
    mem::transmute::<&'a T, &'b T>(r)
}
```

The size check is a compile-time guard: `const _: () = assert!(mem::size_of::<A>() == mem::size_of::<B>());`. Add this assertion next to any transmute to document the invariant explicitly.

## What This Unlocks

- **Low-level bit manipulation** — SIMD type punning, IEEE-754 sign bit extraction, and hardware register interpretation.
- **FFI type erasure** — convert between `*mut c_void` and a typed pointer when the C API demands it and no safe cast exists.
- **Zero-copy deserialization** — reinterpret a validated byte buffer as a `#[repr(C)]` struct (with careful alignment and validity proofs).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Bit reinterpretation | `Obj.magic` (always unsafe, no size check) | `mem::transmute` (compile error if sizes differ) |
| Float bits | No built-in; need `Int32.bits_of_float` (Obj.magic) | `f32::to_bits()` (safe) or `mem::transmute` (unsafe) |
| &str as bytes | `Bytes.unsafe_of_string` | `.as_bytes()` (safe) or transmute (unsafe) |
| Lifetime extension | Not a typed concept | Lifetime parameter transmute — extremely dangerous |
| Safety default | No safeguards | Compiler blocks mismatched sizes; auditor blocks the rest |
