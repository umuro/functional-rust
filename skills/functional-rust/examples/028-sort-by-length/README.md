# 028: Sort By Length

**Difficulty:** 1  **Level:** Foundations

Sort a list of lists by length — and optionally by length frequency (rarest lengths first).

## The Problem This Solves

You have a list of word lists, file paths, or token sequences of varying lengths and need to sort them: shortest first (like a spell-checker ranking by word length), or by how *rare* each length is (useful for prioritizing unusual patterns). The task: given `[['a','b','c'], ['d','e'], ['f'], ['g','h']]`, sort by length to get `[['f'], ['d','e'], ['g','h'], ['a','b','c']]`.

In Python: `sorted(lst, key=len)`. One line. Done.

Rust's version is equally concise — `.sort_by_key(|v| v.len())` — but it works on generic `Vec<T>` elements and is guaranteed to be a stable sort (preserving relative order of equal-length lists). The frequency-sort variant shows how to combine a HashMap with a custom comparator in a clean, readable way.

## The Intuition

In Python:
```python
sorted(lists, key=len)  # sort by length
sorted(lists, key=lambda lst: (lists.count(len(lst)), len(lst)))  # sort by freq
```

In Rust, `.sort_by_key(|v| v.len())` is the direct translation of `key=len`. The key is extracted once per element and used for comparison — same semantics as Python's `key=` argument.

For the frequency sort: first build a frequency map (how many lists have each length), then sort by `(frequency_of_this_length, length_itself)`. This two-level sort means: rarest length first; among equal-frequency lengths, shorter ones come first.

## How It Works in Rust

```rust
// Part a: sort by length, shortest first
fn sort_by_length<T: Clone>(lists: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut result = lists.to_vec();
    result.sort_by_key(|v| v.len());  // stable sort by length
    result
}

// Part b: sort by how rare each length is (rarest first)
fn sort_by_length_freq<T: Clone>(lists: &[Vec<T>]) -> Vec<Vec<T>> {
    // Step 1: count how many lists have each length
    let mut freq: HashMap<usize, usize> = HashMap::new();
    for lst in lists {
        *freq.entry(lst.len()).or_insert(0) += 1;
    }
    let mut result = lists.to_vec();
    // Step 2: sort by (frequency of this length, length itself)
    result.sort_by(|a, b| {
        let fa = freq[&a.len()];
        let fb = freq[&b.len()];
        fa.cmp(&fb).then_with(|| a.len().cmp(&b.len()))
    });
    result
}
```

- `sort_by_key` — stable sort using an extracted key (like Python's `key=`)
- `HashMap::entry(...).or_insert(0)` — count occurrences idiomatically
- `fa.cmp(&fb).then_with(...)` — multi-level comparison: first by frequency, then by length as tiebreaker
- `sort_by` vs `sort_by_key` — use `sort_by` when you need a comparison involving *both* elements simultaneously

## What This Unlocks

- **NLP preprocessing** — sort token sequences by length before batching (common in ML pipelines).
- **Log analysis** — sort log entry groups by length to find outlier patterns.
- **Priority ranking** — sort candidates by a derived metric (length as proxy for complexity).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Sort by key | `List.sort (fun a b -> compare (len a) (len b))` | `.sort_by_key(\|v\| v.len())` |
| Stability | `List.stable_sort` | `.sort_by_key` is always stable |
| Frequency count | `List.fold_left` to build a map | `HashMap::entry().or_insert(0)` |
| Multi-level sort | Nested `compare` | `.cmp().then_with(...)` chained comparison |
| Generic over T | Polymorphic `'a list list` | `Vec<Vec<T>>` with `T: Clone` bound |
