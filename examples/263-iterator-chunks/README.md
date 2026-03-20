📖 **[View on hightechmind.io →](https://hightechmind.io/rust/263-iterator-chunks)**

---

# 263: Fixed-Size Chunks Iteration

## Problem Statement

Batch processing is fundamental to systems programming: sending data in fixed-size network packets, processing database rows in pages, dividing work among threads, or encoding binary data in base64 groups of 3 bytes. The `chunks(n)` method partitions a slice into non-overlapping sub-slices of at most `n` elements, handling the remainder (a possibly smaller final chunk) automatically.

## Learning Outcomes

- Understand how `chunks(n)` divides a slice into non-overlapping groups of at most `n` elements
- Distinguish `chunks()` from `chunks_exact()`: the latter excludes the remainder
- Access the remainder from `chunks_exact()` separately for clean batch+remainder logic
- Process batches in parallel by distributing chunks across threads

## Rust Application

`slice::chunks(n)` yields `&[T]` sub-slices. All chunks except possibly the last have exactly `n` elements. `chunks_exact(n)` yields only full chunks and provides a `.remainder()` method for the tail:

```rust
let data = [1i32, 2, 3, 4, 5];
let chunks: Vec<&[i32]> = data.chunks(2).collect();
// [[1,2], [3,4], [5]]  ← last chunk has 1 element

let exact = data.chunks_exact(2);
let remainder = exact.remainder(); // &[5]
// exact yields [[1,2], [3,4]] only
```

Used in practice: base64 encoding processes `chunks(3)`, HTTP/2 splits frames with `chunks(max_frame_size)`, and parallel iterators partition work with chunks.

## OCaml Approach

OCaml lacks a standard `chunks` function on lists. The standard approach uses `List.filteri` with modular arithmetic or a recursive accumulator:

```ocaml
let rec chunks n lst = match lst with
  | [] -> []
  | _ ->
    let (h, t) = (List.filteri (fun i _ -> i < n) lst,
                  List.filteri (fun i _ -> i >= n) lst) in
    h :: chunks n t
```

For arrays, `Array.sub arr (i * n) n` achieves the same but allocates new arrays.

## Key Differences

1. **Remainder handling**: Rust provides `chunks_exact()` + `.remainder()` to separate full chunks from the tail; OCaml requires manual logic.
2. **Zero-copy**: Rust yields borrowed `&[T]` slices; OCaml creates new sub-lists or arrays.
3. **Standard library**: `chunks()` is built into Rust's slice API; OCaml requires third-party libraries or custom code.
4. **Industrial use**: Batch processing in production systems: page-based DB reads, TLS record splitting, parallel work distribution.

## Exercises

1. Use `chunks(3)` to implement a simple base64 encoder that processes three bytes at a time and handles padding for the remainder.
2. Divide a `Vec<i32>` into `N` approximately equal chunks and compute the sum of each chunk in parallel using threads.
3. Process a binary file's bytes in 512-byte chunks, computing a checksum for each chunk independently.
