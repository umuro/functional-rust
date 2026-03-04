# 022: Range

**Difficulty:** 1  **Level:** Foundations

Create a list of all integers between two values — ascending or descending.

## The Problem This Solves

Generating integer sequences is one of the most basic programming tasks: page numbers, loop indices, test data, coordinate grids. Given `start=4` and `end=9`, produce `[4, 5, 6, 7, 8, 9]`. If `start > end`, produce the reverse: `[9, 8, 7, 6, 5, 4]`.

Python has `range(4, 10)` built in, and JavaScript uses `Array.from({length: n}, (_, i) => start + i)`. These are fine but don't auto-detect direction — you need `range(9, 3, -1)` for descending in Python, or a separate implementation.

Rust's range syntax `(start..=end).collect()` is native and zero-cost — it generates values lazily and collects into a `Vec` only when needed. Adding direction detection is two lines. And since Rust's ranges are typed, the compiler catches mixing `i32` and `usize` before they cause bugs.

## The Intuition

In Python: `list(range(4, 10))` (ascending) or `list(range(9, 3, -1))` (descending)

In JavaScript: `Array.from({length: end - start + 1}, (_, i) => start + i)`

Rust uses the `..=` syntax for inclusive ranges (both ends included). `(4..=9)` creates a `RangeInclusive<i64>` iterator. Calling `.collect()` on it gathers all values into a `Vec<i64>`.

For descending ranges, Rust's `(end..=start).rev()` reverses the iteration. The function detects which direction to use based on `start <= end`.

## How It Works in Rust

```rust
fn range(start: i64, end: i64) -> Vec<i64> {
    if start <= end {
        (start..=end).collect()          // ascending: 4, 5, 6, 7, 8, 9
    } else {
        (end..=start).rev().collect()    // descending: 9, 8, 7, 6, 5, 4
    }
}
```

- `start..=end` — inclusive range (both endpoints included)
- `.collect()` — turns the lazy iterator into a `Vec<i64>`
- `.rev()` — reverses iteration direction; no extra allocation needed
- The type `Vec<i64>` is inferred from context

The bonus `range_step` function in the code shows how to generate ranges with arbitrary step sizes using `.map(|i| start + i * step).take_while(|&x| x <= end)` — the iterator equivalent of Python's `range(start, end, step)`.

## What This Unlocks

- **Test data generation** — create input sequences without hardcoding arrays.
- **Coordinate grids** — `range(0, width)` and `range(0, height)` for 2D iteration.
- **Index sequences** — generate valid indices to use in other slice operations.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Built-in range | `List.init n (fun i -> start + i)` | `(start..=end).collect()` |
| Inclusive range | Manual: `start..end+1` in `List.init` | `..=` is inclusive on both ends |
| Descending | Requires `List.rev` or negative step | `(end..=start).rev().collect()` |
| Lazy evaluation | Evaluated eagerly in list form | `Range` is lazy; `.collect()` forces it |
| Step size | Custom recursive function | `.map()` + `.take_while()` chain |
