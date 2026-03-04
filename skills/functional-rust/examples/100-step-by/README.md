# 100: Step By, Enumerate, Rev

**Difficulty:** 3  **Level:** Intermediate

Three zero-cost iterator adapters that change *how* you traverse — every nth element, indexed pairs, or reverse order.

## The Problem This Solves

Common traversal patterns need small modifications: take every other sample from sensor data, process items with their position in the list, walk a slice backwards without copying. Without dedicated adapters, you'd mix in index arithmetic and manual counters that obscure the real logic.

These three adapters — `step_by`, `enumerate`, `rev` — are the building blocks of structured traversal. They're all zero-cost: no allocation, no extra passes, just a modified view of the underlying iterator.

## The Intuition

- **`step_by(n)`**: skip `n-1` elements between each yield. `iter.step_by(2)` → every other element. `(0..10).step_by(3)` → `0, 3, 6, 9`. Python's `range(0, 10, 3)` works the same way.
- **`enumerate()`**: wraps each element with its 0-based index → `(usize, &T)`. Python's `enumerate(data)` is identical.
- **`rev()`**: iterate backwards. Requires `DoubleEndedIterator`. Python's `reversed()` is similar but creates a new iterator object; Rust's is zero-cost.

All three compose freely with `.map()`, `.filter()`, `.zip()`, and each other.

## How It Works in Rust

```rust
// step_by: every nth element
fn every_nth(data: &[i32], n: usize) -> Vec<i32> {
    data.iter().step_by(n).copied().collect()
}

// Stepped range — equivalent to Python's range(start, stop, step)
fn range_step(start: i32, stop: i32, step: usize) -> Vec<i32> {
    (start..stop).step_by(step).collect()
}

// enumerate: index alongside value
fn find_with_index(data: &[i32], pred: impl Fn(&i32) -> bool) -> Option<(usize, i32)> {
    data.iter()
        .enumerate()
        .find(|(_, x)| pred(x))
        .map(|(i, &x)| (i, x))
}

// Numbered list: "1. item", "2. item", ...
fn format_numbered(items: &[&str]) -> Vec<String> {
    items.iter()
        .enumerate()
        .map(|(i, s)| format!("{}. {}", i + 1, s))
        .collect()
}

// rev: backward traversal without copy
fn reverse_words(sentence: &str) -> String {
    sentence.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

// Combine all three: every other element, numbered, reversed
fn combined(data: &[i32]) -> Vec<String> {
    data.iter()
        .step_by(2)
        .enumerate()
        .rev()
        .map(|(i, &x)| format!("{}: {}", i, x))
        .collect()
}
```

Note: `step_by` must be called before `enumerate` if you want the index to count *after* stepping. Swap the order for different index semantics.

## What This Unlocks

- **Sampling**: take every nth measurement from a sensor stream with `.step_by(n)`.
- **Validation with context**: `.enumerate()` lets you report *which* element failed in an error message.
- **Reversed display**: show items newest-first without reversing the underlying storage.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| step_by | `List.filteri (fun i _ -> i mod n = 0)` | `.step_by(n)` |
| Enumerate | `List.mapi (fun i x -> (i, x))` | `.enumerate()` |
| Reverse | `List.rev` (allocates new list) | `.rev()` (zero-cost adapter) |
| Range with step | Manual recursion | `(start..stop).step_by(n)` |
| Composability | Pipe `\|>` with intermediate lists | Method chaining, no intermediates |
