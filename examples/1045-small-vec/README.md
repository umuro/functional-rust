📖 **[View on hightechmind.io →](https://hightechmind.io/rust/1045-small-vec)**

---

# 1045-small-vec — Small Vector Optimization

## Problem Statement

Heap allocation is expensive: a small `Vec<T>` allocates a buffer on the heap even for zero or one elements. The small vector optimization (SVO) stores elements on the stack up to a threshold, spilling to the heap only when the count exceeds N. LLVM's `SmallVector`, C++'s `llvm::SmallVector`, and Rust's `smallvec` crate all implement this pattern.

For code paths that create many short-lived collections with typically 0–4 elements (AST node children, function argument lists, instruction operands), SVO can eliminate the vast majority of heap allocations.

## Learning Outcomes

- Understand the small vector optimization and its performance rationale
- Implement a simplified `SmallVec<T, const N: usize>` enum in safe Rust
- Handle the transition from inline (stack) storage to heap storage
- Understand `MaybeUninit` for production implementations
- Connect to the `smallvec` crate for production use

## Rust Application

`src/lib.rs` implements `SmallVec<T, const N: usize>` as an enum with `Inline { data: [Option<T>; N], len: usize }` and `Heap(Vec<T>)`. When `push` would overflow the inline array, it transitions to heap storage by collecting the inline elements into a `Vec`. The `Option<T>` wrapper is used instead of `MaybeUninit` for safety; a production implementation would use `MaybeUninit` to avoid the overhead.

The `smallvec` crate uses `MaybeUninit` and is the standard choice in performance-critical Rust code (rustc itself uses it).

## OCaml Approach

OCaml has no direct equivalent because the GC eliminates heap allocation overhead — small allocations are handled by the minor heap and collected quickly. The optimization is not needed:

```ocaml
(* OCaml: always heap-allocated, GC handles efficiently *)
let small_collection = [1; 2; 3]  (* minor heap allocation, fast GC *)
```

For truly performance-critical OCaml code, `Bytes` or `Bigarray` provide stack-like storage without GC overhead, but the pattern is rare.

## Key Differences

1. **GC vs manual**: OCaml's GC makes small allocations cheap via the minor heap; Rust's manual allocator treats each heap allocation equally.
2. **SVO necessity**: SVO is a major optimization in Rust for collections with 0–4 elements; OCaml's GC minor heap makes it less necessary.
3. **`const` generics**: Rust uses `const N: usize` for the stack capacity as a type parameter; OCaml would need a functor parameter.
4. **Production use**: `smallvec`, `arrayvec`, and `tinyvec` are widely used in Rust; OCaml equivalents are rare.

## Exercises

1. Rewrite the inline storage using `arrayvec::ArrayVec<T, N>` (from the `arrayvec` crate) to avoid the `Option<T>` overhead.
2. Implement `iter(&self) -> impl Iterator<Item=&T>` for `SmallVec` that works transparently for both inline and heap storage.
3. Benchmark `SmallVec<i32, 4>` vs `Vec<i32>` for collections of 1–10 elements using `criterion` to measure the SVO benefit.
