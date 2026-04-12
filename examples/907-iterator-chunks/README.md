📖 **[View on hightechmind.io →](https://hightechmind.io/rust/907-iterator-chunks)**

---

# 907-iterator-chunks — Iterator Chunks
**Difficulty:** ⭐⭐  
**Category:** Functional Programming  



## Problem Statement

Processing data in fixed-size batches is fundamental to I/O buffering, pagination, parallel work distribution, and batch database operations. Reading 4096-byte I/O blocks, processing 100 database rows at a time, distributing work across 8 threads — all require splitting a sequence into non-overlapping fixed-size groups. Rust provides `.chunks(n)` for variable-size last chunk and `.chunks_exact(n)` for uniform-size-only processing. These are zero-copy slice operations returning references into the original data. OCaml requires recursive functions or `Array.sub` for equivalent functionality.

## Learning Outcomes

- Use `.chunks(n)` to split a slice into groups of at most n elements
- Use `.chunks_exact(n)` when only full chunks are valid and remainders need separate handling
- Access the remainder via `.chunks_exact(n).remainder()`
- Implement the recursive OCaml-style chunking for comparison
- Understand the difference between overlapping windows and non-overlapping chunks

## Rust Application

`chunk_sums` uses `data.chunks(n).map(|c| c.iter().sum()).collect()`. `chunks_owned` uses `.map(<[T]>::to_vec)` to produce owned chunks. `full_chunks` uses `.chunks_exact(n).map(<[T]>::to_vec)` to discard partial chunks. `chunks_remainder` accesses `.chunks_exact(n).remainder()` — the leftover elements after all full chunks are consumed. `chunks_recursive` implements the same logic recursively: `split_at(n).collect()` on the head, recurse on the tail.

## OCaml Approach

OCaml's `Array.init` with `Array.sub` can chunk arrays: `Array.init (n / k) (fun i -> Array.sub arr (i*k) k)`. For lists: recursive chunking via `let rec take n = function ... in let rec chunks n xs = match take n xs with | ...`. The standard library lacks built-in chunk functions for lists. The `Base` library provides `List.chunks_of: 'a list -> length:int -> 'a list list` as a first-class function.

## Key Differences

1. **Zero-copy**: Rust `.chunks(n)` yields references into the original slice; OCaml `Array.sub` allocates new arrays.
2. **Exact chunks**: `chunks_exact` and `.remainder()` provide clean separation of full and partial chunks; OCaml requires explicit length checking.
3. **Standard library**: Rust has first-class `.chunks()` and `.chunks_exact()` on all slices; OCaml requires the `Base` library or manual recursion.
4. **Mutable chunks**: Rust also has `.chunks_mut(n)` for in-place processing of each chunk; OCaml `Array.blit` for equivalent mutation.

## Exercises

1. Implement `process_in_batches<T: Clone, U, F: Fn(&[T]) -> U>(data: &[T], batch_size: usize, f: F) -> Vec<U>` that applies `f` to each chunk.
2. Write `pad_to_multiple<T: Clone>(data: &[T], n: usize, pad: T) -> Vec<T>` that extends the last chunk to full size.
3. Implement `chunk_by_weight<T>(data: &[T], weight: impl Fn(&T) -> usize, max_weight: usize) -> Vec<Vec<&T>>` that creates chunks where total weight stays below the limit.
