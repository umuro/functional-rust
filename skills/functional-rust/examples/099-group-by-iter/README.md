# 099: Group By Iterator

**Difficulty:** 3  **Level:** Intermediate

Group consecutive equal (or same-key) elements — run-length encoding, deduplication, sequential grouping.

## The Problem This Solves

Many data formats encode repeated values as runs: `"aaabbbcc"` → `[('a',3), ('b',3), ('c',2)]`. Log files group consecutive events of the same type. Sequence compression identifies repeated blocks.

Rust's standard library has `.dedup()` on `Vec` (removes consecutive duplicates in-place) but no built-in group-by for iterators. The `itertools` crate provides one, but for many use cases a clean manual implementation using `Peekable` is more educational and crate-free.

The key pattern: compare each element to the current group's key, flush the group when the key changes.

## The Intuition

Imagine reading cards one by one. You build a pile of identical cards. When you draw a different card, you put the pile down and start a new one. That's group-by on consecutive elements.

Python's `itertools.groupby(data)` does exactly this. Rust has no direct equivalent in std — but the `Peekable` iterator makes it clean to implement: peek at the next element, consume it if it matches the current group, stop if it doesn't.

Important: this groups **consecutive** equal elements only, like Python's `itertools.groupby`. For global grouping by key, use `HashMap`.

## How It Works in Rust

```rust
// Group consecutive equal elements
fn group_consecutive<T: PartialEq + Clone>(data: &[T]) -> Vec<Vec<T>> {
    if data.is_empty() { return vec![]; }
    let mut groups = Vec::new();
    let mut current = vec![data[0].clone()];

    for item in &data[1..] {
        if *item == current[0] {
            current.push(item.clone());        // same as current group
        } else {
            groups.push(current);              // flush completed group
            current = vec![item.clone()];      // start new group
        }
    }
    groups.push(current); // flush last group
    groups
}

// Group by key function — consecutive elements with same key
fn group_by_key<T: Clone, K: PartialEq>(
    data: &[T],
    key: impl Fn(&T) -> K,
) -> Vec<(K, Vec<T>)> {
    if data.is_empty() { return vec![]; }
    let mut groups = Vec::new();
    let mut current_key = key(&data[0]);
    let mut current_group = vec![data[0].clone()];

    for item in &data[1..] {
        let k = key(item);
        if k == current_key {
            current_group.push(item.clone());
        } else {
            groups.push((current_key, current_group));
            current_key = k;
            current_group = vec![item.clone()];
        }
    }
    groups.push((current_key, current_group));
    groups
}

// Run-length encoding: group then map to (value, count)
fn rle_encode<T: PartialEq + Clone>(data: &[T]) -> Vec<(T, usize)> {
    group_consecutive(data)
        .into_iter()
        .map(|g| (g[0].clone(), g.len()))
        .collect()
}

// Decode RLE: expand (value, count) pairs back to sequence
fn rle_decode<T: Clone>(encoded: &[(T, usize)]) -> Vec<T> {
    encoded.iter()
        .flat_map(|(val, count)| std::iter::repeat(val.clone()).take(*count))
        .collect()
}
```

## What This Unlocks

- **Run-length encoding**: compress sequences of repeated values efficiently.
- **Log aggregation**: count consecutive events of the same type in audit logs.
- **Deduplication with counts**: see how many times each consecutive value appeared before changing.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Built-in group_by | No | No (use `itertools` crate or manual) |
| Manual approach | `fold_left` with accumulator | Loop + `Peekable`, or simple loop |
| Dedup in-place | Manual | `Vec::dedup()` built-in |
| RLE encode | Map over fold groups | Map over `group_consecutive` |
| Global group by key | `Map.add` in fold | `HashMap` accumulation |
