# 010: Run-Length Encoding

**Difficulty:** ⭐⭐  **Level:** Intermediate

Compress a list by replacing repeated runs with a count + value pair.

## The Problem This Solves

Run-length encoding (RLE) is one of the simplest compression techniques. Instead of storing `["a", "a", "a", "b", "c", "c", "c"]`, you store `[(3, "a"), (1, "b"), (3, "c")]` — the run length and the value. Older image formats, fax machines, and simple protocols all use RLE.

Building it from scratch in a loop is straightforward but tedious: maintain a counter, track the "current element," compare, flush when it changes. Rust's iterator model lets you build this cleanly, either by composing smaller operations (pack-then-map) or in a single efficient fold.

This example also introduces *function composition*: the `encode` function builds on a `pack` helper, chaining transformations rather than computing everything at once.

## The Intuition

Think of it in two steps:

1. **Pack**: group consecutive equal elements together
   `["a","a","b","c","c","c"]` → `[["a","a"], ["b"], ["c","c","c"]]`

2. **Encode**: count each group
   `[["a","a"], ["b"], ["c","c","c"]]` → `[(2,"a"), (1,"b"), (3,"c")]`

In Python you'd reach for `itertools.groupby`:

```python
from itertools import groupby
result = [(sum(1 for _ in g), k) for k, g in groupby(lst)]
```

Rust does the same with `.fold()` or by composing a pack function with `.map()`. The end result is a `Vec<(usize, T)>` — a vector of (count, value) tuples.

## How It Works in Rust

```rust
// The composed approach: pack first, then map each group to (count, first_element)
fn encode<T: PartialEq + Clone>(list: &[T]) -> Vec<(usize, T)> {
    pack(list)
        .into_iter()
        .map(|group| (group.len(), group[0].clone()))
        .collect()
}
```

The single-pass fold version (no intermediate groups, most efficient):

```rust
fn encode_fold<T: PartialEq + Clone>(list: &[T]) -> Vec<(usize, T)> {
    list.iter().fold(Vec::new(), |mut acc, x| {
        match acc.last_mut() {
            Some((count, val)) if val == x => *count += 1,  // extend current run
            _                              => acc.push((1, x.clone())),  // start new run
        }
        acc
    })
}
```

The `last_mut()` call gets a mutable reference to the last element — if it matches `x`, we increment the counter in place. Otherwise we start a new `(1, x)` run. One pass, no intermediate allocations.

Using the result:

```rust
let encoded = encode(&["a","a","b","c","c","c"]);
// [(2, "a"), (1, "b"), (3, "c")]

// Decoding: reverse the process
let decoded: Vec<&str> = encoded.iter()
    .flat_map(|(count, val)| std::iter::repeat(*val).take(*count))
    .collect();
// ["a", "a", "b", "c", "c", "c"]
```

## What This Unlocks

- **Simple compression** — image data, network packets, any repetitive sequence
- **Sequence summarization** — "3 consecutive failures, then 1 success" in logs
- **Decoding** — `flat_map` with `repeat().take()` reverses the encoding in one expression

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Tuple type | `int * 'a` | `(usize, T)` |
| Count type | `int` (signed) | `usize` (unsigned) |
| Composing pack + encode | `List.map` after `pack` | `.into_iter().map()` |
| Single-pass fold | `List.fold_left` | `.fold()` with `last_mut()` |
| Intermediate groups | `Vec<Vec<T>>` then map | Optional — fold avoids it |
