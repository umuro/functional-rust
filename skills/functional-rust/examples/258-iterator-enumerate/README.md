# 258: Index-Value Pairs with enumerate()

**Difficulty:** 1  **Level:** Beginner

Get `(index, value)` pairs from any iterator without tracking a counter manually.

## The Problem This Solves

You're iterating a list and you need the position of each element — to number output lines, filter by index, or find which position holds a value. The naive solution is a mutable `let mut i = 0;` counter that you increment at the bottom of every loop body. That counter can drift, get forgotten after a `continue`, or just clutter the code.

In Python you reach for `enumerate(items)`. In OCaml you use `List.iteri` or `List.mapi`. In Rust, `enumerate()` is the idiomatic answer — it works on *any* iterator, not just slices, and integrates cleanly with the rest of the adapter chain.

The bigger payoff comes in chains. Without `enumerate()` you can't easily combine index-awareness with `filter`, `map`, or `find` — you'd need to `collect()` first, then use index-based loops. With `enumerate()` you stay in the lazy pipeline.

## The Intuition

`enumerate()` wraps each element with its zero-based position, turning `Iterator<Item=T>` into `Iterator<Item=(usize, T)>`.

```rust
let fruits = ["apple", "banana", "cherry"];
for (i, fruit) in fruits.iter().enumerate() {
    // i = 0, 1, 2
}
```

## How It Works in Rust

```rust
let fruits = ["apple", "banana", "cherry", "date"];

// Basic: loop with position
for (i, fruit) in fruits.iter().enumerate() {
    println!("{}: {}", i, fruit);
}

// Filter by index — keep only even positions
let even_indexed: Vec<_> = fruits.iter()
    .enumerate()
    .filter(|(i, _)| i % 2 == 0)  // pattern-match the tuple
    .map(|(_, v)| *v)               // drop the index again
    .collect();

// Format with 1-based numbers
let numbered: Vec<String> = fruits.iter()
    .enumerate()
    .map(|(i, f)| format!("{}. {}", i + 1, f))  // i+1 for 1-based
    .collect();

// Find first matching element AND its position
let found = fruits.iter()
    .enumerate()
    .find(|(_, f)| f.starts_with('c'));  // → Some((2, "cherry"))
```

Destructure the tuple immediately in the closure signature — it's cleaner than `pair.0`/`pair.1`.

## What This Unlocks

- **Numbered output** — generate "1. item", "2. item" lists without a manual counter.
- **Index-aware filtering** — keep every Nth element, skip the header row (index 0), etc.
- **Position search** — find the index of the first element matching a condition (as an alternative to `position()` when you need both the index and the value).

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Iterate with index | `List.iteri (fun i x -> ...)` | `iter.enumerate()` |
| Map with index | `List.mapi (fun i x -> ...)` | `.enumerate().map(\|(i, x)\| ...)` |
| Filter with index | `List.filteri (fun i _ -> ...)` | `.enumerate().filter(\|(i, _)\| ...)` |
| Index type | `int` | `usize` (always non-negative) |
| Works on any iterator | No — list-specific functions | Yes — any `Iterator` |
