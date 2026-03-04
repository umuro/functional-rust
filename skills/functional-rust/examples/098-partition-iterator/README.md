# 098: Partition Iterator

**Difficulty:** 3  **Level:** Intermediate

Split a collection into two groups by a predicate — evens/odds, valid/invalid, successes/failures.

## The Problem This Solves

You have a mixed collection and need to separate it into two groups without traversing it twice. Classic examples: split a list of parse results into successes and errors, separate positive from negative numbers, classify events into handled and unhandled.

Writing two separate `.filter()` calls is intuitive but wasteful — two full passes, two allocations. `.partition()` does both in a single pass, building both output collections simultaneously.

For multi-way classification (more than two groups), `.partition()` isn't enough. But the same single-pass pattern extends naturally to `match` branches or a manual fold.

## The Intuition

Think of `.partition()` as two `.filter()` calls fused into one. The predicate routes each element: `true` → first collection, `false` → second collection. Both collections are built in one pass, in original order.

Python's equivalent is a comprehension pattern: `trues = [x for x in data if pred(x)]` + `falses = [x for x in data if not pred(x)]` — two passes. Rust's `.partition()` is one.

The return type is generic: `(B, B) where B: Default + Extend<Item>`. In practice this almost always means `(Vec<T>, Vec<T>)`.

## How It Works in Rust

```rust
// Binary partition: even vs odd
fn split_even_odd(data: &[i32]) -> (Vec<i32>, Vec<i32>) {
    data.iter().partition(|&&x| x % 2 == 0)
    // → (evens, odds)
}

// Multi-way classification: use a loop for 3+ categories
fn classify_numbers(data: &[i32]) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut negative = Vec::new();
    let mut zero = Vec::new();
    let mut positive = Vec::new();
    for &x in data {
        match x.cmp(&0) {
            std::cmp::Ordering::Less    => negative.push(x),
            std::cmp::Ordering::Equal   => zero.push(x),
            std::cmp::Ordering::Greater => positive.push(x),
        }
    }
    (negative, zero, positive)
}

// Partition Results: separate Ok and Err in one pass
fn partition_results(data: &[&str]) -> (Vec<i32>, Vec<String>) {
    let mut oks = Vec::new();
    let mut errs = Vec::new();
    for &s in data {
        match s.parse::<i32>() {
            Ok(n)  => oks.push(n),
            Err(e) => errs.push(format!("{}: {}", s, e)),
        }
    }
    (oks, errs)
}

// Split at first match: keep before + after the split point
fn split_at_first(data: &[i32], pred: impl Fn(&i32) -> bool) -> (&[i32], &[i32]) {
    match data.iter().position(|x| pred(x)) {
        Some(i) => (&data[..i], &data[i..]),
        None    => (data, &[]),
    }
}
```

## What This Unlocks

- **Error handling pipelines**: collect all parse errors to display, proceed with valid values.
- **Routing**: partition incoming events by type, process each group with specialized logic.
- **Data cleaning**: separate well-formed records from malformed ones in a single scan.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Binary split | `List.partition pred` | `.partition(pred)` |
| Return type | `('a list * 'a list)` | `(B, B) where B: Default + Extend` |
| Multi-way | `fold_left` with accumulator | Manual `match` loop |
| With transformation | `partition_map` (external) | Manual `match` loop |
| Split at position | Manual recursion | `.position()` + slice indexing |
