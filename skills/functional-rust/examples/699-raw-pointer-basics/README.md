# 699: Raw Pointer Basics

**Difficulty:** 4  **Level:** Expert

Create, cast, and safely dereference `*const T` and `*mut T` raw pointers.

## The Problem This Solves

Safe Rust's reference rules — one writer XOR many readers, always valid, always aligned — give the borrow checker everything it needs to prove memory safety at compile time. But those same rules make certain low-level patterns impossible to express: building a custom allocator, writing into a C library's output parameter, slicing into a buffer at an arbitrary byte offset, or constructing a self-referential data structure.

Raw pointers exist for exactly these situations. They carry none of the borrow checker's guarantees, which means *you* must enforce those guarantees manually. Creating a raw pointer is always safe in Rust; dereferencing it requires an `unsafe` block where you assert, in a `// SAFETY:` comment, that the pointer is valid, aligned, non-null, and not aliased by a conflicting reference.

unsafe is a tool, not a crutch — use only when safe Rust genuinely can't express the pattern.

## The Intuition

Rust has two raw pointer types: `*const T` (read-only) and `*mut T` (read/write). Think of them as street addresses — you can write down any address without leaving your house (creation is safe), but physically going there and reading or writing memory is a commitment you make at your own risk (dereferencing requires `unsafe`).

The compiler cannot verify that the address is currently occupied, that the occupant is what you expect, or that nobody else is modifying it at the same time. It steps back and trusts your `// SAFETY:` comment.

## How It Works in Rust

```rust
// Creating raw pointers — safe, no dereference yet
let value: i32 = 42;
let const_ptr: *const i32 = &value as *const i32;

let mut mutable_val: i32 = 100;
let mut_ptr: *mut i32 = &mut mutable_val as *mut i32;

// Dereferencing — requires unsafe + proof
let read_val = unsafe {
    // SAFETY: const_ptr derives from `value` which is still live on the
    // stack; the pointer is valid, aligned, and no &mut alias exists.
    *const_ptr
};

// Safe wrapper: bounds-check before touching raw memory
fn safe_read(slice: &[u32], idx: usize) -> Option<u32> {
    if idx >= slice.len() { return None; }
    let ptr: *const u32 = slice.as_ptr();
    Some(unsafe {
        // SAFETY: idx < slice.len() verified above; ptr is valid for
        // slice.len() elements; slice invariants guarantee alignment.
        *ptr.add(idx)
    })
}
```

Key invariants you must guarantee manually:
1. The pointer is non-null.
2. The memory it points to is valid (live, properly allocated).
3. The type's alignment requirement is met.
4. No conflicting `&mut` reference exists at the same time.
5. The memory has been fully initialised before reading.

## What This Unlocks

- **Custom allocators and arena bumpers** — manage raw memory blocks without going through `Box` or `Vec`.
- **FFI boundaries** — receive a C pointer, validate it once, then wrap it in a safe API so the rest of Rust never sees it.
- **High-performance buffer manipulation** — skip redundant bounds checks in tight loops where the index was already validated externally.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Raw memory access | `Bigarray`, `Bytes.unsafe_get` | `*const T` / `*mut T` in `unsafe` block |
| Safety guarantee | Runtime checks or trust | Compile-time (references) or manual (raw pointers) |
| Null pointer | Not a concern (GC heap) | `*mut T` can be null; use `NonNull<T>` to rule it out |
| Lifetime tracking | Garbage collector | Manual: you guarantee the pointee outlives the pointer |
| Borrow rules | Not enforced | Suspended for raw pointers — you own the contract |
