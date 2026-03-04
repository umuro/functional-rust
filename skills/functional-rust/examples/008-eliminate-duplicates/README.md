# 008: Eliminate Consecutive Duplicates

**Difficulty:** ⭐  **Level:** Beginner

Remove repeated adjacent elements from a list, keeping one of each run.

## The Problem This Solves

You have a sequence with repeated runs: `["a", "a", "a", "b", "c", "c", "d", "e", "e"]`. You want: `["a", "b", "c", "d", "e"]`. This comes up in log deduplication, compressing repeated sensor readings, processing run-length encoded data, or collapsing repeated keypresses.

Note the "consecutive" qualifier: only adjacent duplicates are removed. `["a", "b", "a"]` stays as-is — the two `"a"`s are not adjacent. If you want to remove *all* duplicates regardless of position, that's a different problem (use a `HashSet`).

The loop-based approach is straightforward but wordy: track the "last seen" element, compare, conditionally push. Rust offers two cleaner paths: a one-liner for in-place mutation, and a functional version for when you need a new collection.

## The Intuition

```python
# Python — no built-in, need itertools or a loop
from itertools import groupby
result = [k for k, _ in groupby(lst)]

# JavaScript — no built-in, manual filter
const result = arr.filter((x, i) => i === 0 || x !== arr[i-1]);
```

```rust
// Rust — in-place, one line
vec.dedup();

// Or functional — returns a new Vec
let result: Vec<_> = list.iter()
    .enumerate()
    .filter(|(i, x)| *i == 0 || list[i-1] != **x)
    .map(|(_, x)| x.clone())
    .collect();
```

`dedup()` is short for "deduplicate". It's built into `Vec<T>` and runs in a single pass, O(n) time, O(1) extra space. It requires `T: PartialEq` — the elements must be comparable.

## How It Works in Rust

```rust
// Option 1: In-place mutation (fastest, no allocation)
fn compress(list: &mut Vec<T>) where T: PartialEq {
    list.dedup();  // modifies the Vec directly
}

// Option 2: Functional — returns a new Vec (input unchanged)
fn compress_functional<T: PartialEq + Clone>(list: &[T]) -> Vec<T> {
    list.iter()
        .enumerate()
        .filter(|(i, x)| *i == 0 || list[i - 1] != **x)
        .map(|(_, x)| x.clone())
        .collect()
}
```

The `&mut` in Option 1 is explicit — you're saying "I'm modifying this data." In JavaScript, mutation is implicit and often surprising. In Rust, you can't mutate without opt-in.

The `windows(2)` approach compares adjacent pairs directly:

```rust
// Using windows — reads naturally as "pairs of neighbors"
fn compress_windows<T: PartialEq + Clone>(list: &[T]) -> Vec<T> {
    if list.is_empty() { return vec![]; }
    
    let mut result = vec![list[0].clone()];
    result.extend(
        list.windows(2)
            .filter(|w| w[0] != w[1])
            .map(|w| w[1].clone())
    );
    result
}
```

`windows(2)` gives you overlapping pairs: `[a,b]`, `[b,c]`, `[c,d]`... wherever two neighbors differ, the second one belongs in the result.

## What This Unlocks

- **Log deduplication** — collapse repeated identical log lines before storing
- **Signal processing** — remove identical consecutive sensor readings (noise elimination)
- **Run-length encoding** — `dedup` is the preprocessing step before counting runs (see example 010)

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Idiomatic approach | Recursive pattern match on cons cells | `vec.dedup()` (in-place) |
| In-place option | Not available (immutable lists) | `dedup()` with `&mut Vec<T>` |
| Pairwise comparison | `h1 :: h2 :: _` pattern | `windows(2)` iterator |
| Equality | Polymorphic `=` | `PartialEq` trait bound |
| New list vs mutation | Always new list | Explicit choice (`&` vs `&mut`) |
