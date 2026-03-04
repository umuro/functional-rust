# 713: `#[repr(C)]` Structs for FFI Interop

**Difficulty:** 4  **Level:** Expert

Guarantee your Rust struct's memory layout matches its C counterpart, field for field.

## The Problem This Solves

Rust is free to reorder a struct's fields, insert padding, or choose any layout that satisfies alignment constraints. This is deliberate — it allows the compiler to minimise the struct's size or improve cache behaviour. But when a Rust struct must interoperate with a C struct at the binary level — passed by value across an FFI boundary, or written into a memory-mapped file format, or used with a C library that reads the fields by byte offset — freedom to reorder is a bug, not a feature.

`#[repr(C)]` locks the layout to the C ABI rules: fields appear in declaration order, padding is inserted to satisfy alignment in the same way C does it, and the struct size matches `sizeof(struct ...)` in C. This lets you share structs between Rust and C code without any marshalling overhead — the same bytes mean the same thing on both sides.

unsafe is a tool, not a crutch — use only when safe Rust genuinely can't express the pattern.

## The Intuition

By default Rust structs are opaque to the linker — their layout is the compiler's internal concern. `#[repr(C)]` makes the layout a public contract. You can now write the corresponding C struct definition and know that `offset_of!(RustStruct, field)` matches `offsetof(CStruct, field)`.

This is a design-time commitment: every field type must also have a stable, C-compatible layout. Rust-specific types like `Vec<T>`, `Box<T>`, `String`, or enums without `#[repr(C)]` must not appear inside a `#[repr(C)]` struct — they have no C equivalent.

## How It Works in Rust

```rust
use std::mem;

/// C: typedef struct { double x; double y; } Point2D;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Point2D { pub x: f64, pub y: f64 }

/// C: typedef struct { Point2D origin; double width; double height; } Rect;
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Rect { pub origin: Point2D, pub width: f64, pub height: f64 }

/// C: typedef struct { uint8_t r, g, b, a; } Color;
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color { pub r: u8, pub g: u8, pub b: u8, pub a: u8 }

// Verify sizes match C at compile time or runtime:
fn verify_layout() {
    println!("Point2D: {} bytes (expect 16)", mem::size_of::<Point2D>());
    println!("Rect:    {} bytes (expect 32)", mem::size_of::<Rect>());
    println!("Color:   {} bytes (expect 4)",  mem::size_of::<Color>());
    // Field offsets:
    println!("Point2D.x offset: {}", mem::offset_of!(Point2D, x)); // 0
    println!("Point2D.y offset: {}", mem::offset_of!(Point2D, y)); // 8
}

// Pass by value across FFI — layout is guaranteed to match:
let r = Rect { origin: Point2D { x: 0.0, y: 0.0 }, width: 4.0, height: 3.0 };
let area = unsafe {
    // SAFETY: rect_area accepts any Rect by value; layout is #[repr(C)].
    rect_area(r)
};
```

Always verify sizes and offsets in tests. A layout mismatch between your `#[repr(C)]` struct and the actual C definition is a silent ABI bug — no compile error, but corrupt data at runtime.

## What This Unlocks

- **Graphics and physics engines** — share vertex structs, transformation matrices, and collision shapes between Rust game logic and C/C++ renderers.
- **Network protocols and file formats** — parse binary packet headers or on-disk structures by casting a byte buffer pointer to a `#[repr(C)]` struct.
- **OS and hardware interfaces** — Linux `ioctl` structures, Win32 `POINT`/`RECT`, and ARM CMSIS register structs are all `#[repr(C)]` by nature.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Struct layout | Unspecified (GC-managed) | Unspecified by default; `#[repr(C)]` locks to C ABI |
| C struct binding | `Ctypes.structure` / `cstruct` ppx | `#[repr(C)]` struct — zero-cost at runtime |
| Padding | Managed by GC/Ctypes | Matches C compiler padding rules exactly |
| Field access | Via Ctypes field descriptors | Direct Rust field access — same binary offsets |
| Layout verification | Runtime type descriptions | `mem::size_of!` and `mem::offset_of!` at compile or runtime |
