[![hightechmind.io](https://img.shields.io/badge/hightechmind.io-functional--rust-blue)](https://hightechmind.io)

# 093 — Windows and Chunks

## Problem Statement

Use Rust's slice methods `.windows(n)` and `.chunks(n)` to create overlapping and non-overlapping subslice iterators. Demonstrate that `windows` slides one element at a time (producing `len - n + 1` windows) while `chunks` partitions without overlap (last chunk may be shorter). Compare with OCaml's manual recursive implementations.

## Learning Outcomes

- Apply `slice.windows(n)` for overlapping n-element views
- Apply `slice.chunks(n)` for non-overlapping n-element partitions
- Understand that both return iterators of `&[T]` — zero-copy subslice references
- Use `chunks_exact(n)` when trailing partial chunks should be excluded
- Map Rust's built-in slice methods to OCaml's recursive list splitting
- Recognise common uses: sliding averages (windows), batch processing (chunks)

## Rust Application

`v.windows(3)` on `[1, 2, 3, 4, 5]` yields `[1,2,3]`, `[2,3,4]`, `[3,4,5]` — three overlapping slices. Each is a borrowed `&[i32]` — no copying. `v.chunks(2)` yields `[1,2]`, `[3,4]`, `[5]` — the last chunk has only one element. `chunks(3)` on a length-6 slice yields two exact chunks of three. The methods are part of `std::slice` and work on any `&[T]`.

## OCaml Approach

OCaml has no built-in `windows`/`chunks` on lists. The recursive `windows n lst` takes a prefix of length `n`, adds it to the accumulator, and recurses on the tail. `chunks n lst` groups elements into sublists of size `n`. Both use `List.rev` and manual counting — more verbose than Rust's built-in methods. Array operations (`Array.sub`) would be closer to Rust's approach.

## Key Differences

| Aspect | Rust | OCaml |
|--------|------|-------|
| Windows | `slice.windows(n)` built-in | Manual recursive function |
| Chunks | `slice.chunks(n)` built-in | Manual recursive function |
| Element type | `&[T]` (borrowed subslice) | `'a list` (new list) |
| Allocation | Zero-copy | Allocates sublists |
| Exact chunks | `chunks_exact(n)` | Manual length check |
| String windows | `s.as_bytes().windows(n)` | Substring extraction |

`windows` is particularly useful for sliding window algorithms: computing moving averages, detecting runs, or finding maximum subarray sums. `chunks` is useful for batch processing: splitting work across threads or formatting data in fixed-size blocks.

## Exercises

1. Compute a 3-element moving average of a `Vec<f64>` using `windows(3)`.
2. Use `windows(2)` to count adjacent pairs where the second element is greater than the first (number of increases).
3. Implement `batch_process<T, F>(items: &[T], batch_size: usize, f: F)` using `chunks` to process items in groups.
4. Use `v.windows(2)` to implement `is_sorted(v: &[i32]) -> bool`.
5. In OCaml, implement `windows_seq n lst` using `Seq` for laziness — producing windows on demand without materialising all at once.
