📖 **[View on hightechmind.io →](https://hightechmind.io/rust/887-windows-chunks)**

---

# 887-windows-chunks — Windows and Chunks

## Problem Statement

Sliding window algorithms process overlapping sub-sequences for signal analysis, moving averages, and local extrema detection. Non-overlapping chunking processes data in fixed batches for pagination, block processing, and parallel work distribution. Both are fundamental in data processing pipelines. Rust provides `.windows(n)` (overlapping, zero-copy) and `.chunks(n)` / `.chunks_exact(n)` (non-overlapping) as built-in slice methods. These are unique in that they operate on slices (borrowing from the original data) rather than consuming iterators, enabling zero-allocation window and chunk processing.

## Learning Outcomes

- Use `.windows(n)` for overlapping sliding windows over slices
- Use `.chunks(n)` for non-overlapping batch processing
- Use `.chunks_exact(n)` when uniform chunk sizes are required and remainders should be handled separately
- Compute moving averages, pairwise differences, and local maxima using windows
- Compare with OCaml's recursive list splitting for similar operations

## Rust Application

`moving_average` uses `data.windows(window_size).map(|w| w.iter().sum::<f64>() / window_size as f64)`. `pairwise_diff` uses `data.windows(2).map(|w| w[1] - w[0])`. `local_maxima` uses `data.windows(3).filter(|w| w[1] > w[0] && w[1] > w[2]).map(|w| w[1])`. For chunks: `chunk_sums` uses `data.chunks(size).map(|c| c.iter().sum())`. `chunks_exact` yields only full-size chunks with the remainder accessible via `.remainder()` — critical for batch processing where partial batches need special handling.

## OCaml Approach

OCaml lacks built-in window/chunk operations on lists. The idiomatic approach uses recursive functions: `let rec windows n lst = match lst with | [] -> [] | _ -> if List.length lst < n then [] else (List.filteri (fun i _ -> i < n) lst) :: windows n (List.tl lst)`. For arrays, `Array.sub arr i n` extracts a chunk. OCaml's `Array.init` with modular arithmetic can implement circular windows. The absence of built-in windows/chunks is a significant usability difference.

## Key Differences

1. **Zero-copy**: Rust `.windows(n)` returns references into the original slice — no allocation; OCaml typically allocates new lists or arrays per window.
2. **Exact chunks**: `chunks_exact` provides a separate remainder accessor — OCaml requires explicit length-check after manual chunking.
3. **Built-in vs recursive**: Rust has first-class slice methods; OCaml requires writing recursive functions for window and chunk operations.
4. **Bounds safety**: Rust windows/chunks are bounds-safe by construction; OCaml `Array.sub` can raise `Invalid_argument` on out-of-bounds.

## Exercises

1. Use `.windows(5)` to find the position of the subarray with the maximum sum in a slice of integers.
2. Implement `chunk_by_weight<T: Clone, F: Fn(&T) -> usize>` that splits a slice into chunks where each chunk's total weight stays below a limit.
3. Write `overlapping_pairs_sum` using `.windows(2)` that returns the sum of every adjacent pair.
