# Example 1183: Array.blit — Copy Subarray
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Copy a contiguous sub-range of one array into another array at a specified offset. The OCaml primitive `Array.blit src src_pos dst dst_pos len` copies `len` elements starting at `src_pos` in `src` into `dst` beginning at `dst_pos`.

## Learning Outcomes

- How Rust's `copy_from_slice` provides the same bulk-copy primitive as `Array.blit`
- Slice sub-range syntax `&arr[a..b]` is the direct analogue of OCaml's `src_pos`/`len` pair
- The ownership split between `&[T]` (immutable source) and `&mut [T]` (mutable destination) makes aliasing impossible at compile time
- A functional alternative using `Vec` shows how to avoid in-place mutation while keeping clarity

## OCaml Approach

`Array.blit` is OCaml's built-in for block array copies. It is imperative by nature: it mutates `dst` directly. The type system does not enforce source/destination separation — both arguments are `'a array`, and the programmer must ensure no aliasing.

## Rust Approach

Rust encodes the mutability contract in the type: the destination must be `&mut [T]` while the source is `&[T]`. Sub-ranges are expressed via slice indices `[start..end]`, and `copy_from_slice` performs the bulk copy in one call. The `T: Copy` bound ensures the element type is cheaply bitwise-copyable, mirroring OCaml's `Array.blit` which works on any value type.

## Key Differences

1. **Mutability:** OCaml arrays are always mutable; Rust requires `&mut [T]` explicitly.
2. **Bounds expression:** OCaml uses `(src_pos, len)` pair; Rust uses slice range `[pos..pos+len]`.
3. **Aliasing safety:** Rust's borrow checker prevents passing the same array as both source and destination without `unsafe`.
4. **Functional alternative:** Rust can produce a new `Vec<T>` instead of mutating, making the operation pure at the cost of an allocation.
