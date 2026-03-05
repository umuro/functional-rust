📖 **[View on hightechmind.io →](https://hightechmind.io/rust/263-iterator-chunks)**

---

# 263: Fixed-Size Chunks Iteration

**Difficulty:** 2  **Level:** Intermediate

Divide a slice into non-overlapping groups of N — batch processing, pagination, matrix rows.

## The Problem This Solves

You have a flat buffer and you want to process it in fixed-size batches: send 100 records to a database at a time, render a flat pixel array as image rows, paginate a list of results. The naive approach uses index arithmetic — `data[i*n..(i+1)*n]` — which requires manually computing the number of batches and handling the final short batch.

Unlike `windows()`, chunks are non-overlapping: each element appears in exactly one chunk. That makes them the right tool for batch operations where processing the same element twice would be wrong or wasteful.

OCaml has no built-in chunk iterator. You'd write a recursive function that splits the list at position `n` repeatedly. In Rust, `chunks(n)` is a zero-copy slice method. The last chunk may be shorter than `n` if the length isn't evenly divisible — `chunks_exact(n)` gives you only full-size chunks, with the remainder accessible separately.

## The Intuition

`chunks(n)` divides the slice into consecutive non-overlapping sub-slices of at most `n` elements each. Unlike `windows()` which slides forward by 1, `chunks()` advances by the full window size.

```rust
let data = [1, 2, 3, 4, 5, 6, 7];
for chunk in data.chunks(3) {
    println!("{:?}", chunk);  // [1,2,3], [4,5,6], [7]
}
```

## How It Works in Rust

```rust
let data = [1i32, 2, 3, 4, 5, 6, 7];

// Sum each chunk
let chunk_sums: Vec<i32> = data.chunks(3)
    .map(|c| c.iter().sum())
    .collect();
// → [6, 15, 7]

// chunks_exact: only full chunks, remainder accessible
let exact_iter = data.chunks_exact(3);
let remainder = exact_iter.remainder();  // → &[7]
let full_chunks: Vec<_> = data.chunks_exact(3).collect();
// → [[1,2,3], [4,5,6]]  (no partial chunk)

// Batch processing with index
let items: Vec<i32> = (1..=10).collect();
for (batch_num, batch) in items.chunks(4).enumerate() {
    println!("Batch {}: {:?}", batch_num, batch);
}

// Treat flat array as a 2D matrix (3 columns)
let matrix: Vec<i32> = (1..=9).collect();
for row in matrix.chunks(3) {
    println!("{:?}", row);  // [1,2,3], [4,5,6], [7,8,9]
}
```

Use `chunks_exact()` when a short final chunk would be a bug (e.g., parsing fixed-width binary records). Use `chunks()` when a short tail is acceptable.

## What This Unlocks

- **Batch database writes** — process N records at a time, reducing round trips.
- **Matrix / image operations** — treat a flat `Vec<u8>` as rows of pixels by chunking at the row width.
- **Pagination** — split a result set into pages without collecting multiple sub-vecs.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Non-overlapping groups | Manual recursion | `slice.chunks(n)` |
| Require exact size | Exception / manual check | `chunks_exact(n)` + `.remainder()` |
| Zero-copy | No | Yes — references into original slice |
| Overlapping variant | N/A | `windows(n)` |
| Works on iterators | N/A | Slice method — collect first if needed |
