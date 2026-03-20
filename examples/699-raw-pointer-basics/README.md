📖 **[View on hightechmind.io →](https://hightechmind.io/rust/699-raw-pointer-basics)**

---

# Raw Pointer Basics
**Difficulty:** ⭐  
**Category:** Functional Programming  


## Problem Statement

Raw pointers (`*const T` and `*mut T`) are Rust's lowest-level memory access primitive — the equivalent of C's pointers. They bypass the borrow checker entirely: no lifetime tracking, no aliasing rules, no validity guarantees. They are essential for FFI, custom allocators, lock-free data structures, and performance-critical code that cannot afford the overhead of safe abstractions. The safe-wrapper idiom encapsulates raw pointer operations behind a safe function boundary, isolating the `unsafe` footprint.

## Learning Outcomes

- How `*const T` and `*mut T` differ from references: no lifetime, no borrow rules
- How to create raw pointers from references: `slice.as_ptr()`, `&x as *const T`
- How `ptr.add(idx)` performs bounds-checked offset arithmetic
- How to write safe wrappers that validate preconditions before using raw pointers
- Where raw pointers are necessary: FFI, custom allocators, `MaybeUninit` initialization

## Rust Application

`safe_read(slice: &[u32], idx: usize) -> Option<u32>` checks bounds, gets `slice.as_ptr()` as `*const u32`, then uses `unsafe { *ptr.add(idx) }` inside the validated block. Every `unsafe` block includes a `// SAFETY:` comment explaining the invariant that makes the operation sound. Raw pointers can be created from references without unsafe but can only be dereferenced inside `unsafe`.

Key patterns:
- `slice.as_ptr()` — raw pointer from reference (safe creation)
- `unsafe { *ptr.add(n) }` — dereference with SAFETY comment
- `if idx >= slice.len() { return None; }` — bounds check before unsafe
- `// SAFETY: idx < slice.len() checked above` — mandatory documentation

## OCaml Approach

OCaml's GC manages all memory — raw pointer access is not part of normal OCaml programming. Unsafe memory operations use `Bigarray`, `Bytes`, or the `ctypes` FFI library:

```ocaml
(* OCaml equivalent: use array indexing with bounds checking *)
let safe_read arr idx =
  if idx >= Array.length arr then None
  else Some arr.(idx)
```

## Key Differences

1. **Memory model**: Rust raw pointers are explicit in source code and require `unsafe`; OCaml's GC manages all pointers transparently.
2. **SAFETY comments**: Rust's idiom requires `// SAFETY:` comments explaining why each `unsafe` block is sound; OCaml has no equivalent because unsafe is not part of the language.
3. **Bounds checking**: Rust manually checks bounds before raw pointer access; OCaml array access always bounds-checks.
4. **FFI integration**: Rust raw pointers map directly to C pointers in FFI; OCaml's `ctypes` library wraps C pointers in type-safe OCaml values.

## Exercises

1. **Stride access**: Implement `fn every_nth(slice: &[i32], n: usize) -> Vec<i32>` using raw pointer arithmetic — validate `n > 0` and bounds check before each unsafe dereference.
2. **Swap via pointers**: Write `unsafe fn swap_raw<T>(a: *mut T, b: *mut T)` and a safe wrapper `fn safe_swap<T>(a: &mut T, b: &mut T)` — include SAFETY documentation.
3. **SAFETY audit**: Take the source code and add SAFETY comments to every unsafe operation listing: (a) the precondition that must hold, (b) why it holds in this context.
