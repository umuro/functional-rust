📖 **[View on hightechmind.io →](https://hightechmind.io/rust/890-exact-size)**

---

# 890-exact-size — ExactSizeIterator

## Problem Statement

When processing data with a known count, reporting progress percentages, pre-allocating output buffers, or rendering progress bars, you need the total element count upfront. Standard `Iterator` does not guarantee this — `.count()` might consume the iterator. `ExactSizeIterator` adds a guaranteed-O(1) `.len()` method for iterators that know their exact size before iteration. Slice iterators, range iterators, and many standard adapters implement it. This enables accurate progress reporting, guaranteed single-allocation output with `Vec::with_capacity`, and chunk-boundary calculations without consuming the source.

## Learning Outcomes

- Use `.len()` on `ExactSizeIterator` for O(1) size queries
- Pre-allocate output with `Vec::with_capacity(iter.len())` to avoid reallocations
- Report progress using `enumerate().map(|(i, x)| format!("[{}/{}]", i+1, total, x))`
- Implement a custom `ExactSizeIterator` with correct `size_hint` and `len`
- Understand which standard adapters preserve vs destroy `ExactSizeIterator`

## Rust Application

`process_with_progress` captures `let total = data.len()` then uses `.enumerate().map(|(i, &x)| format!("[{}/{}] ...", i+1, total, x))` — the total is known before any elements are processed. `progress_bar` renders an ASCII progress bar using integer division. `map_preallocated` uses `Vec::with_capacity(data.len())` to allocate exactly once before filling. `chunks_exact` divides by `n` to compute the exact number of full chunks. The key insight: `.len()` on a slice is O(1) via `ExactSizeIterator`.

## OCaml Approach

OCaml arrays have O(1) `Array.length`. OCaml lists have O(n) `List.length`. For pre-allocation, OCaml's `Array.make n default` pre-allocates; `Buffer.create hint_size` pre-allocates for strings. Progress reporting requires storing the length before iteration begins: `let total = Array.length arr in Array.iteri (fun i x -> Printf.printf "[%d/%d]" (i+1) total) arr`. The `ExactSizeIterator` trait has no direct OCaml equivalent as a formal interface.

## Key Differences

1. **Formalized guarantee**: Rust `ExactSizeIterator` is a formal trait with a compiler-checkable contract; OCaml has no equivalent interface — length must be tracked manually.
2. **Adapter propagation**: `map`, `zip`, `enumerate` preserve `ExactSizeIterator` when both inputs implement it; `filter` and `flat_map` do not.
3. **Pre-allocation idiom**: Rust `Vec::with_capacity(iter.len())` is the canonical pattern; OCaml uses `Array.make` for known-size output.
4. **size_hint**: `ExactSizeIterator` strengthens `Iterator::size_hint` to return `(len, Some(len))` — adapters use this for optimization hints.

## Exercises

1. Implement a custom `ExactSizeIterator` for a `Grid<T>` that iterates over all cells, reporting the exact cell count.
2. Write `progress_map<T, U, F>(data: &[T], f: F, report: impl Fn(usize, usize))` that calls `report(current, total)` for each element.
3. Implement `batch_process<T: Clone>(data: &[T], batch_size: usize) -> Vec<Vec<T>>` using `chunks_exact` and pre-allocating exactly `data.len() / batch_size` batches.
