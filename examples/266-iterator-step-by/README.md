📖 **[View on hightechmind.io →](https://hightechmind.io/rust/266-iterator-step-by)**

---

# 266: Striding with step_by()

## Problem Statement

Subsampling — taking every nth element from a sequence — is needed in signal processing (downsampling), matrix operations (accessing column strides), image processing (pixel subsampling), and data analysis (sampling large datasets). Without a dedicated combinator, this requires index-based loops or manual counter tracking. The `step_by(n)` adapter yields the first element, then skips `n-1` elements, repeatedly — turning strided access into a composable iterator operation.

## Learning Outcomes

- Understand that `step_by(n)` yields the first element then every nth subsequent element
- Use `step_by()` with ranges to generate arithmetic progressions
- Combine `step_by()` with other adapters for strided processing pipelines
- Recognize the relationship between `step_by()` and array stride concepts in numerical computing

## Rust Application

`Iterator::step_by(n)` skips `n-1` elements between each yielded element. The first element is always included. Step size must be at least 1:

```rust
// Every 3rd element from a range
let result: Vec<usize> = (0..10).step_by(3).collect();
// [0, 3, 6, 9]

// Odd-indexed elements of a slice
let arr = [10, 20, 30, 40, 50];
let odd_indexed: Vec<i32> = arr.iter().copied().step_by(2).collect();
// [10, 30, 50]

// Generate even numbers: range starting at 0, step 2
let evens: Vec<i32> = (0..).step_by(2).take(5).collect();
// [0, 2, 4, 6, 8]
```

## OCaml Approach

OCaml lacks a built-in `step_by` for lists. It is implemented with `List.filteri` modulo arithmetic or via `Array` index stepping:

```ocaml
let step_by n lst =
  List.filteri (fun i _ -> i mod n = 0) lst

(* Or with sequences *)
let step_by_seq n seq =
  Seq.filter_mapi (fun i x -> if i mod n = 0 then Some x else None) seq
```

## Key Differences

1. **Standard library**: `step_by()` is built into Rust's `Iterator`; OCaml requires `filteri` or manual index tracking.
2. **Infinite sources**: Rust's `step_by()` composes with infinite ranges (`(0..).step_by(2)`); OCaml needs lazy sequences for this.
3. **Numerical computing link**: `step_by` mirrors the "stride" concept in NumPy arrays (`a[::step]`), BLAS Level 1 routines (DAXPY stride), and C array pointer arithmetic.
4. **Efficiency**: Rust's `step_by` skips elements without evaluating them on most iterator types; OCaml's `filteri` always calls the predicate on every element.

## Exercises

1. Use `step_by(2)` to extract all elements at odd indices from a slice, and `skip(1).step_by(2)` to extract elements at even indices.
2. Generate the arithmetic sequence 3, 7, 11, 15, ... (starting at 3 with step 4) using `(3i32..).step_by(4)`.
3. Implement matrix column extraction: given a flat row-major matrix as `&[f64]` with `cols` columns, extract column `k` using `step_by(cols)` with an appropriate skip.
