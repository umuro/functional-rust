# 703: Raw Pointer Arithmetic

**Difficulty:** 4  **Level:** Expert

Navigate memory with `ptr.add()`, `ptr.sub()`, and `ptr.offset()` — and wrap the results safely.

## The Problem This Solves

Safe Rust slices are bounds-checked at every indexed access. That's the right default, but it generates redundant checks in loops where you've already verified the range once at entry. More critically, some patterns don't map to slice indexing at all: strided access over every Nth element, in-place reversal via two converging pointers, or reading a struct field at a known byte offset in a memory-mapped buffer.

Raw pointer arithmetic gives you the same power as C pointer arithmetic — advance a pointer by N elements, swap two elements by address, walk a buffer without re-indexing — with one difference: Rust requires you to put the arithmetic in an `unsafe` block and document *why* each offset stays in bounds. The compiler trusts you; reviewers verify you.

unsafe is a tool, not a crutch — use only when safe Rust genuinely can't express the pattern.

## The Intuition

A raw pointer is a typed address. `ptr.add(n)` computes `ptr + n * size_of::<T>()` — it advances by N *elements*, not N bytes. `ptr.offset(n)` is the signed version: positive advances forward, negative steps back. `ptr.sub(n)` is `ptr.add(wrapping_neg(n))`.

The compiler cannot see that `ptr.add(offset)` stays within the original allocation. You must carry that proof in your head (or your `// SAFETY:` comment). The contract: the resulting pointer must point within the same allocated object, or one-past-the-end (for comparison only, not dereference).

## How It Works in Rust

```rust
/// Collect every `stride`-th element without repeated bounds checks.
pub fn strided_collect(slice: &[i32], stride: usize) -> Vec<i32> {
    if slice.is_empty() || stride == 0 { return vec![]; }
    let mut result = Vec::new();
    let base: *const i32 = slice.as_ptr();
    let len = slice.len();
    let mut offset = 0usize;
    while offset < len {
        result.push(unsafe {
            // SAFETY: offset < len == slice.len(); base is valid for len
            // elements; alignment guaranteed by slice invariant.
            *base.add(offset)
        });
        offset = offset.saturating_add(stride);
    }
    result
}

/// In-place reversal via two converging raw pointers.
pub fn ptr_reverse(slice: &mut [i32]) {
    let len = slice.len();
    if len < 2 { return; }
    let base: *mut i32 = slice.as_mut_ptr();
    let (mut lo, mut hi) = (0usize, len - 1);
    while lo < hi {
        unsafe {
            // SAFETY: lo < hi < len; both offsets are in bounds; lo != hi
            // so the two pointers never alias the same slot.
            std::ptr::swap(base.add(lo), base.add(hi));
        }
        lo += 1; hi -= 1;
    }
}
```

Key rules: always compute offsets from a slice's `as_ptr()` (which guarantees alignment), always check bounds *before* the `unsafe` block, and never create two `*mut` pointers to the same element simultaneously.

## What This Unlocks

- **SIMD and vectorisation** — feed contiguous raw pointers to `std::arch` intrinsics that process 4/8/16 elements at a time.
- **Memory-mapped file parsing** — walk a `*const u8` over a mmap'd buffer, reading fixed-size records at computed offsets without copying.
- **Custom data structures** — implement a ring buffer, deque, or gap buffer where advancing the write head wraps around with pointer arithmetic.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Pointer arithmetic | `Bigarray` indexing or `Bytes.get` | `ptr.add(n)` / `ptr.sub(n)` in `unsafe` |
| Bounds checking | Always on (or `unsafe_get` convention) | Safe slices always check; raw pointers never check |
| Signed offset | Not a language concept | `ptr.offset(n: isize)` — negative steps backwards |
| Two-pointer swap | `Array` swap function | `std::ptr::swap(a, b)` — takes two raw pointers |
| Stride access | Manual index arithmetic | `ptr.add(offset)` with manual stride tracking |
