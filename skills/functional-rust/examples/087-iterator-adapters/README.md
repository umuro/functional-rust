# 087: Iterator Adapters

**Difficulty:** 2  **Level:** Intermediate

Chain lazy transformations on iterators — `map`, `filter`, `flat_map`, `take`, `skip`, `enumerate`, `chain` — building data pipelines that compute nothing until you consume them.

## The Problem This Solves

Processing collections often involves multiple steps: filter out invalid entries, transform each value, take the first N results. The naive approach is a series of loops, each producing a temporary `Vec`. This allocates memory you don't need and forces you to complete each step before starting the next.

Iterator adapters solve this by chaining transformations lazily. When you write `.filter(...).map(...).take(5)`, nothing runs. Only when you call `.collect()`, `.sum()`, or any other consumer does the chain execute — and it processes elements one at a time, passing each through the entire pipeline before moving to the next. No intermediate allocations.

This is the core of functional data processing in Rust. The pipeline reads like what you're doing, not how you're doing it.

## The Intuition

In Python, you have `map()`, `filter()`, and list comprehensions — though Python's `map`/`filter` are lazy by default. In JavaScript, `.map().filter().reduce()` chains on arrays are eager (each step creates a new array). In OCaml, `List.map`, `List.filter` are eager too; `Seq.map` is lazy.

Rust's iterator adapters are *always* lazy — they return a new iterator type that wraps the previous one. The only work done is in the final consumer. This makes long pipelines memory-efficient even on large datasets.

## How It Works in Rust

```rust
// Each adapter returns a new iterator — nothing runs yet
let result: Vec<String> = data.iter()
    .filter(|&&x| x > 0)       // lazy: wrap in Filter struct
    .map(|&x| x * x)            // lazy: wrap in Map struct
    .map(|x| x.to_string())     // lazy: another Map struct
    .collect();                  // ONLY HERE does computation happen
```

```rust
// flat_map: each element produces an iterator, then all are flattened
let words: Vec<String> = sentences.iter()
    .flat_map(|s| s.split_whitespace())   // "hello world" → ["hello", "world"]
    .map(String::from)
    .collect();
```

```rust
// take and skip for pagination / windowing
let page: Vec<i32> = data.iter()
    .copied()
    .skip(page_num * page_size)   // skip previous pages
    .take(page_size)               // take one page
    .collect();
```

```rust
// chain: concatenate two iterators without allocating
let combined: Vec<i32> = first_half.iter().chain(second_half.iter()).copied().collect();
```

```rust
// enumerate: get (index, value) pairs
let indexed: Vec<(usize, i32)> = data.iter()
    .enumerate()
    .filter(|(_, &v)| v % 2 == 0)   // keep even values, with their original index
    .map(|(i, &v)| (i, v))
    .collect();
```

```rust
// inspect: side-effect for debugging without breaking the chain
let result = data.iter()
    .inspect(|x| eprintln!("before filter: {}", x))  // prints but doesn't change value
    .filter(|x| x % 2 == 0)
    .inspect(|x| eprintln!("after filter: {}", x))
    .collect::<Vec<_>>();
```

## What This Unlocks

- **ETL pipelines**: read records, filter by criteria, transform fields, write output — all in one expressive chain with no intermediate buffers.
- **Windowed or paginated processing**: `skip(n * size).take(size)` for pagination, `windows(n)` and `chunks(n)` for sliding-window algorithms.
- **Text processing**: `flat_map(str::split)` to tokenize, then filter, map, and collect — cleaner than nested loops.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Map | `List.map` (eager) / `Seq.map` (lazy) | `.map(f)` — always lazy |
| Filter | `List.filter` (eager) / `Seq.filter` (lazy) | `.filter(p)` — always lazy |
| Flatten | `List.concat_map` / `Seq.flat_map` | `.flat_map(f)` |
| Take N | `Seq.take` | `.take(n)` |
| Combine sequences | `Seq.append` | `.chain(other)` |
| Index pairing | `List.mapi` | `.enumerate()` |
| Debug side-effect | No standard equivalent | `.inspect(f)` |
