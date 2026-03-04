# 018: Slice List

**Difficulty:** 1  **Level:** Foundations

Extract a subrange from a list using 1-based inclusive indices — like `lst[i-1:k]` in Python.

## The Problem This Solves

You have a list of items and need to extract a specific window: records 3 through 7, characters 2 through 5, rows 10 through 20. The task: given `['a','b','c','d','e','f','g','h','i','k']` and indices `3..7`, produce `['c','d','e','f','g']`.

In Python you'd write `lst[2:7]` (0-based, exclusive end). In most textbooks and many APIs, indices are 1-based and inclusive — which requires a mental translation every time. The off-by-one error is one of the most common bugs in slice operations.

Rust makes this translation explicit in one place and safe everywhere else. You convert once (`i - 1` for 0-based start, `k.min(len)` for bounded end), then use the native slice syntax which the compiler verifies is in-bounds.

## The Intuition

In Python: `lst[i-1:k]` (0-based, exclusive end)

In JavaScript: `lst.slice(i-1, k)`

This example uses 1-based inclusive indexing (like the OCaml 99 Problems source). So "slice from 3 to 7" means "include elements at positions 3, 4, 5, 6, 7" — which in 0-based terms is indices 2 through 6.

Rust's native slice syntax is 0-based and exclusive at the end: `lst[2..7]`. The function does the conversion for you. After that, `.to_vec()` copies the subrange into a new owned vector.

## How It Works in Rust

```rust
fn slice<T: Clone>(lst: &[T], i: usize, k: usize) -> Vec<T> {
    let start = if i == 0 { 0 } else { i - 1 };  // 1-based → 0-based
    let end = k.min(lst.len());                    // clamp to list length
    if start >= end {
        return vec![];  // empty range
    }
    lst[start..end].to_vec()  // Rust slice: 0-based, exclusive end
}
```

- `i - 1` converts from 1-based start to 0-based
- `k.min(lst.len())` ensures we never exceed the list (no panic!)
- `lst[start..end]` is Rust's native slice syntax — fast, bounds-checked
- `.to_vec()` creates an owned copy

The recursive version (`slice_rec`) walks the list with a position counter, collecting elements when `pos >= i && pos <= k`. It's slower (O(n) allocations) but shows how the index bookkeeping maps to explicit counting.

## What This Unlocks

- **Windowed processing** — extract any fixed-size window from a buffer or time series.
- **Substring operations** — slice bytes or chars from a larger sequence.
- **API pagination** — extract page N with offset/limit converted to slice indices.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Slice syntax | No native slice; use `List.filteri` | `lst[start..end]` (0-based, exclusive) |
| Index convention | 1-based (99 Problems style) | 0-based natively; convert at boundary |
| Bounds safety | Runtime `Invalid_argument` | `.min(lst.len())` + safe slice |
| Copy to owned | `List.map Fun.id` | `.to_vec()` |
| Empty range | Returns `[]` | Returns `vec![]` |
