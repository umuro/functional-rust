📖 **[View on hightechmind.io →](https://hightechmind.io/rust/775-const-array-size)**

---

# 775-const-array-size — Const Array Size

## Problem Statement

Heap-allocated `Vec<T>` is versatile but has overhead: a pointer, length, and capacity word, plus a heap allocation. For collections with a small, known maximum size (network packet fields, audio samples, fixed-size queues), a stack-allocated vector with a compile-time capacity bound eliminates all this overhead. `StackVec<T, CAP>` provides `Vec`-like operations with a static capacity guarantee, failing gracefully when the capacity is exceeded rather than allocating more space.

## Learning Outcomes

- Implement `StackVec<T: Copy + Default, const CAP: usize>` with `data: [T; CAP]` and a `len: usize` runtime counter
- Provide `push() -> Result<(), T>` that returns the item on overflow instead of panicking
- Implement `pop() -> Option<T>`, `get(i)`, and `as_slice()` for `Vec`-like ergonomics
- Verify size: `size_of::<StackVec<u8, 64>>()` = 64 + 8 (len) bytes — no heap pointer
- See real-world applications: `heapless::Vec` in embedded Rust, `arrayvec` crate

## Rust Application

`StackVec<T, CAP>` stores `data: [T; CAP]` initialized to `T::default()` and `len: usize`. `push` checks `len < CAP` before writing and incrementing; returns `Err(value)` on overflow. `pop` decrements `len` and returns the element. `as_slice()` returns `&self.data[..self.len]`. The `capacity()` method is `const fn`. Tests verify push-to-capacity, overflow handling, pop behavior, and iteration.

## OCaml Approach

OCaml has no stack-allocated arrays of generic size. All arrays (`Array.make n x`) are heap-allocated. For fixed-size buffers, OCaml uses `Bytes.create n` (mutable) or `Bigarray` (C-backed). The `Cstruct` library in MirageOS provides a `StackVec`-like interface over C-allocated buffers for network packet processing. OCaml's GADT-based length-indexed lists provide type-level length tracking but are heap-allocated.

## Key Differences

1. **Stack vs heap**: Rust's `StackVec<T, CAP>` is stack-allocated for small `CAP`; OCaml arrays are always heap-allocated.
2. **Overflow handling**: Rust returns `Err(item)` on overflow, giving callers control; OCaml would raise an exception.
3. **Embedded use**: `heapless::Vec` (Rust crate) is used in `no_std` embedded firmware; OCaml cannot target microcontrollers without GC.
4. **Type system**: `StackVec<u8, 64>` and `StackVec<u8, 128>` are different Rust types; OCaml arrays of different sizes are the same type.

## Exercises

1. Implement `extend_from_slice(slice: &[T]) -> Result<(), ()>` that copies all slice elements into the `StackVec` or fails if there is insufficient capacity.
2. Add `sort(&mut self)` using `self.data[..self.len].sort()` and verify it works correctly with the length boundary.
3. Implement `StackVec::from_slice<const N: usize>(s: &[T; N]) -> Option<Self>` that creates a `StackVec` from a fixed array, returning `None` if `N > CAP`.
