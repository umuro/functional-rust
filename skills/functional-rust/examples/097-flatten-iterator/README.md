# 097: Flatten Iterator

**Difficulty:** 3  **Level:** Intermediate

Collapse one level of nesting in `Vec<Vec<T>>`, `Vec<Option<T>>`, or `Vec<Result<T>>` — no manual loops.

## The Problem This Solves

Data often arrives pre-grouped: a list of sentences (each being a list of words), a batch of optional values (each being `Some` or `None`), nested search results. To process the elements individually you need to unwrap the outer container.

Without `.flatten()`, you write nested for-loops or call `.iter().flat_map(|x| x.iter())` every time. With it, nesting collapses in one step — and it composes with the rest of the iterator chain.

The real surprise: in Rust, `Option` and `Result` implement `IntoIterator`. `Some(x)` iterates once, `None` iterates zero times. This makes `.flatten()` a clean replacement for "filter out Nones."

## The Intuition

Think of `.flatten()` as removing one layer of wrapping from each element. If you have `Vec<Vec<i32>>`, each inner `Vec` is unwrapped and its elements flow into the combined output.

The rule: `.flatten()` works on any `Iterator<Item: IntoIterator>`. Since `Option` and `Result` both implement `IntoIterator`, they "flatten out" naturally — Somes become items, Nones disappear.

`.flat_map(f)` = `.map(f).flatten()` — when the mapping function itself returns an iterable, use `flat_map` to skip the intermediate nesting.

## How It Works in Rust

```rust
// Flatten a Vec<Vec<T>> — one level deep
fn flatten_vecs(nested: Vec<Vec<i32>>) -> Vec<i32> {
    nested.into_iter().flatten().collect()
}

// flat_map: map to words then flatten into one stream
fn words_in_sentences(sentences: &[&str]) -> Vec<String> {
    sentences.iter()
        .flat_map(|s| s.split_whitespace()) // each &str → multiple &strs
        .map(String::from)
        .collect()
}

// Expand ranges: each (lo, hi) pair → range of integers
fn expand_ranges(ranges: &[(i32, i32)]) -> Vec<i32> {
    ranges.iter().flat_map(|&(lo, hi)| lo..=hi).collect()
}

// Option implements IntoIterator: Some → 1 item, None → 0 items
fn flatten_options(opts: Vec<Option<i32>>) -> Vec<i32> {
    opts.into_iter().flatten().collect()
    // equivalent to: .filter_map(|x| x)
}

// Parse what you can, silently drop failures
fn parse_ints(strs: &[&str]) -> Vec<i32> {
    strs.iter().filter_map(|s| s.parse::<i32>().ok()).collect()
}

// Deep flatten: two levels
fn deep_flatten(nested: Vec<Vec<Vec<i32>>>) -> Vec<i32> {
    nested.into_iter().flatten().flatten().collect()
}
```

Note: `.flatten()` only collapses **one** level. For deeper nesting, chain multiple `.flatten()` calls.

## What This Unlocks

- **Option pipelines**: `.flatten()` on `Vec<Option<T>>` to drop Nones without explicit match.
- **Text processing**: split by sentence then by word in one expression.
- **Tree traversal**: collect children of children without building intermediate `Vec`s.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Flatten one level | `List.flatten` | `.flatten()` |
| flat_map | `List.concat_map f` | `.flat_map(f)` |
| Option to iterator | `Option.to_list` | `Option` implements `IntoIterator` |
| Filter + map | `List.filter_map` | `.filter_map(f)` |
| Deep flatten | Nested `List.flatten` | Chained `.flatten().flatten()` |
