# 279: Iterator nth()

**Difficulty:** 1  **Level:** Beginner

Skip to the n-th element of an iterator (0-indexed), consuming all elements before it in the process.

## The Problem This Solves

You need the third item in a CSV after skipping the header, or the fifth element of a filtered stream, or you want to advance a stateful iterator past the first few items. Without `nth`, you'd call `skip(n).next()` — which works but requires knowing the relationship between `skip` and `next`. Or you'd collect everything into a Vec and index it — wasteful if you only need one element.

`nth(n)` is the direct expression of "give me element at position n." It's 0-indexed (nth(0) is the first element, like array indexing). It's O(n) for most iterators — it has to skip through n elements. For random access on known collections, direct indexing `v[n]` is O(1); use `nth` when you're working with a pipeline.

Critical behavior: `nth` *consumes* the iterator up to and including index n. After calling `iter.nth(2)`, the iterator is positioned at index 3. Calling `iter.nth(0)` afterward returns the element at the original index 3.

## The Intuition

Advance the iterator n positions, discard everything before position n, and return the element at position n as `Option<T>`.

## How It Works in Rust

```rust
let nums = [10i32, 20, 30, 40, 50];

nums.iter().nth(0);   // Some(&10) — first element
nums.iter().nth(2);   // Some(&30) — third element (0-indexed)
nums.iter().nth(10);  // None — out of bounds

// nth advances the iterator — subsequent calls see the rest
let mut iter = nums.iter();
iter.nth(1);  // Some(&20)  — skips index 0, returns index 1
iter.nth(0);  // Some(&30)  — iterator is now at index 2; returns index 2

// Skip CSV header, get the second data row
let csv = "name,age\nAlice,30\nBob,25\nCarol,35";
csv.lines().nth(2);  // Some("Bob,25") — skip header + Alice

// nth on a filtered iterator
let fourth_even = (0i32..20).filter(|x| x % 2 == 0).nth(3);
// → Some(6) — the evens are [0,2,4,6,...]; index 3 is 6
```

## What This Unlocks

- **CSV/file parsing:** Skip header rows and access specific data rows without collecting all lines into a Vec.
- **Iterator slicing:** Use `nth(n)` then continue iterating — effectively "cut off the first n elements" of a stateful iterator.
- **Lazy random access:** Access position n in a `filter`/`map` pipeline without materializing the whole sequence.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Get by index | `List.nth lst n` | `iter.nth(n)` |
| Out of bounds | `Failure` exception | Returns `None` |
| Indexing base | 0-based | 0-based |
| Consumes items | No (list persists) | Yes — advances iterator state |
| After call | List unchanged | Iterator is past index n |
