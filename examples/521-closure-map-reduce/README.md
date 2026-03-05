📖 **[View on hightechmind.io →](https://hightechmind.io/rust/521-closure-map-reduce)**

---

# 521: Map-Reduce with Closures

**Difficulty:** 2  **Level:** Beginner-Intermediate

Transform data with `map`, aggregate it with `fold` — single-pass, zero intermediate allocations.

## The Problem This Solves

Data processing tasks follow a universal pattern: transform each record (map), then aggregate the results (reduce). Word counts, statistics, inverted indices, and histogram generation all share this shape.

Without the map-reduce pattern, you write nested loops with mutable accumulators scattered through the body — hard to test, hard to parallelize, and easy to introduce off-by-one errors.

The second problem is performance: naive implementations create intermediate collections. Square every element into a new `Vec`, then sum that `Vec`. Two passes, two allocations. `iter.map(square).sum()` does it in one pass, zero intermediate allocation.

## The Intuition

Map-reduce is a two-phase recipe:
1. **Map**: cook each ingredient separately (transform each element).
2. **Reduce**: combine everything into a final dish (fold into a single result).

Google's MapReduce framework scales this to thousands of machines. Rust's iterator combinators apply the same pattern locally, with compile-time fusion turning the chain into a single optimized loop.

In Python: `sum(x*x for x in items if x % 2 == 0)` — a generator expression is lazy map-reduce. In JavaScript: `items.filter(x => x%2===0).map(x => x*x).reduce((a,b) => a+b, 0)`. Rust's version: `items.iter().filter(|&&x| x%2==0).map(|&x| x*x).sum()` — identical semantics, but the compiler proves there are no data races and eliminates all intermediate allocations.

## How It Works in Rust

```rust
use std::collections::HashMap;

let nums: Vec<i32> = (1..=10).collect();

// Classic map-reduce: sum of squares
let sum_sq: i32 = nums.iter()
    .map(|&x| x * x)   // map: transform
    .sum();             // reduce: aggregate

// Generic map-reduce function
fn map_reduce<T, U, V, M, R>(items: &[T], mapper: M, reducer: R, init: V) -> V
where M: Fn(&T) -> U, R: Fn(V, U) -> V {
    items.iter().map(mapper).fold(init, reducer)
}
let result = map_reduce(&nums, |&x| x * x, |acc, sq| acc + sq, 0);

// Word frequency — fold into HashMap
let words = ["hello", "world", "foo", "hello", "foo", "foo"];
let freq = words.iter().fold(HashMap::new(), |mut acc: HashMap<&str, usize>, &word| {
    *acc.entry(word).or_insert(0) += 1;
    acc
});

// Inverted index: word → list of positions
let text = "the cat sat on the mat the cat";
let idx = text.split_whitespace()
    .enumerate()
    .fold(HashMap::new(), |mut map: HashMap<&str, Vec<usize>>, (pos, word)| {
        map.entry(word).or_insert_with(Vec::new).push(pos);
        map
    });

// Single-pass statistics — no intermediate Vec needed
#[derive(Debug)]
struct Stats { count: usize, sum: f64, min: f64, max: f64 }
let data = [3.0f64, 1.0, 4.0, 1.0, 5.0, 9.0];
let stats = data.iter().copied().fold(None::<Stats>, |acc, x| {
    Some(match acc {
        None    => Stats { count: 1, sum: x, min: x, max: x },
        Some(s) => Stats { count: s.count+1, sum: s.sum+x,
                           min: s.min.min(x), max: s.max.max(x) },
    })
}).unwrap();
println!("mean: {:.2}", stats.sum / stats.count as f64);
```

## What This Unlocks

- **Single-pass analytics** — compute count, sum, min, max, and mean in one `fold` without creating intermediate collections.
- **Functional data transformation** — word counts, histograms, frequency tables, and inverted indices expressed declaratively.
- **Foundation for parallelism** — map-reduce's structure is inherently parallelizable; Rayon's `par_iter().map().reduce()` uses the same API with thread-level parallelism.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Map | `List.map f xs` | `iter.map(f)` — lazy |
| Reduce/fold | `List.fold_left f init xs` | `iter.fold(init, \|a, x\| f(a, x))` |
| Eager vs lazy | Eager — intermediate lists allocated | Lazy — fused into one loop at compile time |
| Associative reduce | `List.fold_left (+) 0` | `iter.sum()` / `iter.product()` |
| Parallel map-reduce | External library | `rayon::par_iter().map().reduce()` |
