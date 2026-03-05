📖 **[View on hightechmind.io →](https://hightechmind.io/rust/266-iterator-step-by)**

---

# 266: Iterator step_by()

**Difficulty:** 1  **Level:** Beginner

Stride through an iterator, yielding every nth element — lazy, zero-allocation, works on infinite sequences.

## The Problem This Solves

You want every third element from a list, or multiples of 5 from a range, or to downsample a signal array. Without `step_by`, you'd write a counter variable, manually increment it, and check `if counter % n == 0` — imperative noise for a declarative idea. Or you'd reach for `filter(|i| i % n == 0)` on an enumerated iterator, which is verbose and less readable.

`step_by(n)` names the intent directly: "give me every nth element." It's lazy (no intermediate allocation), works on any iterator including infinite ones, and composes with the rest of the iterator pipeline. For downsampling audio, generating arithmetic sequences, or accessing every kth row of a matrix, it's the idiomatic choice.

A common gotcha: `step_by(1)` is identity — it yields every element. `step_by(0)` panics.

## The Intuition

Yield the first element, skip n-1, yield the next, skip n-1, repeat — effectively striding through the iterator at a fixed interval.

## How It Works in Rust

```rust
// Every 3rd element from a range
let thirds: Vec<usize> = (0..10).step_by(3).collect();
// → [0, 3, 6, 9]

// Arithmetic sequences from infinite iterator
let odd_positions: Vec<u64> = (1u64..).step_by(2).take(5).collect();
// → [1, 3, 5, 7, 9]

// Downsample a signal array
let signal = [1.0f64, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
let downsampled: Vec<f64> = signal.iter().copied().step_by(2).collect();
// → [1.0, 3.0, 5.0, 7.0]

// Every other character
let every_other: String = "abcdefgh".chars().step_by(2).collect();
// → "aceg"
```

The key: `step_by` takes n as the stride, starting from index 0. It does **not** skip the first element.

## What This Unlocks

- **Signal downsampling:** Reduce audio/image resolution by taking every nth sample without loading all data into memory.
- **Arithmetic progressions:** `(0..).step_by(k)` generates multiples of k lazily — no `range(0, max, k)` needed.
- **Matrix row access:** Stride through a flat array to visit every nth row of a row-major matrix.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Equivalent | `List.filteri (fun i _ -> i mod n = 0) lst` | `iter.step_by(n)` |
| Lazy | No (lists are eager) | Yes |
| Infinite sequences | No | Yes (with `(0..).step_by(n)`) |
| Readability | Verbose (index + modulo) | Declarative |
| Zero-copy | No (new list) | Yes (adapter over original) |
