📖 **[View on hightechmind.io →](https://hightechmind.io/rust/262-iterator-windows)**

---

# 262: Sliding Windows over Slices

**Difficulty:** 2  **Level:** Intermediate

Iterate over every overlapping N-element sub-slice — zero-copy, no index arithmetic, no bounds checking.

## The Problem This Solves

Many algorithms need to look at a group of consecutive elements together: compute a moving average over the last 3 data points, check if a sequence is strictly sorted, detect local maxima by comparing each point to its neighbors, extract bigrams or trigrams from a word list. Without `windows()`, you'd use index arithmetic — `data[i-1]`, `data[i]`, `data[i+1]` — which requires careful bounds checking and obscures the intent.

The sliding window is a fundamental pattern in signal processing, text analysis, and sequence algorithms. Each window overlaps the previous by `n-1` elements, stepping one position forward each time. For a slice of length `L` and window size `n`, you get `L - n + 1` windows.

OCaml has no built-in sliding window — you'd write a recursive function or use `List.init` with sublist extraction, allocating new lists at each step. Rust's `windows(n)` is a zero-copy method on slices that returns slice references into the original data.

## The Intuition

`windows(n)` produces sub-slices of exactly `n` consecutive elements, advancing one step at a time. Every element except the first and last `n-1` appears in multiple windows.

```text
data = [1, 2, 3, 4, 5]    windows(3):
         [1, 2, 3]
            [2, 3, 4]
               [3, 4, 5]
```

Each window is a `&[T]` — a borrowed view into the original slice. No allocation, no copying. Passing a window to `.iter().sum()` or indexing `w[0]`, `w[1]` works as you'd expect.

## How It Works in Rust

```rust
let data = [1i32, 2, 3, 4, 5];

// Moving average (window size 3)
let moving_avg: Vec<f64> = data.windows(3)
    .map(|w| w.iter().sum::<i32>() as f64 / 3.0)
    .collect();
// → [2.0, 3.0, 4.0]

// Check if strictly increasing — compare each adjacent pair
let is_increasing = data.windows(2).all(|w| w[0] < w[1]);
// → true

// Find local maxima: center element greater than both neighbors
let signal = [1i32, 3, 2, 5, 4, 6, 2];
let local_max: Vec<usize> = signal.windows(3)
    .enumerate()
    .filter(|(_, w)| w[1] > w[0] && w[1] > w[2])  // middle > both sides
    .map(|(i, _)| i + 1)                            // i is index of w[0]; center is i+1
    .collect();
// → [1, 3, 5]

// Bigrams from a word list
let words = ["the", "quick", "brown", "fox"];
let bigrams: Vec<_> = words.windows(2).collect();
// → [["the","quick"], ["quick","brown"], ["brown","fox"]]
```

`windows()` is a slice method — call it on `&[T]` or arrays. Window size must be > 0 (panics otherwise). For non-overlapping chunks, use `chunks(n)` instead.

## What This Unlocks

- **Signal processing** — moving averages, rolling min/max, smoothing filters over sensor or financial data.
- **NLP n-grams** — bigrams and trigrams from token sequences without manual indexing or subslice extraction.
- **Sequence validation** — sorted check, monotonicity, pattern matching over consecutive elements in one clean expression.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sliding window | Manual recursion or `List.init` | `slice.windows(n)` — built in |
| Memory | Allocates new lists per window | Zero-copy — references into original slice |
| Overlapping | Must implement manually | Built-in behavior |
| Non-overlapping | Must implement manually | `chunks(n)` instead |
| Works on iterators | N/A | Slice method only — collect to `Vec` first if needed |
