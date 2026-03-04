# 088: Iterator Consumers

**Difficulty:** 2  **Level:** Intermediate

Drive an iterator to completion with terminal operations — `collect`, `fold`, `sum`, `find`, `any`, `all` — the eager step that makes lazy pipelines produce results.

## The Problem This Solves

Iterator adapters are lazy — they set up transformations but compute nothing. To get results, you need a *consumer*: something that walks the iterator from start to finish and produces a value. Without consumers, your pipeline is just a description sitting in memory.

Consumers answer different questions: "give me all the results as a Vec" (`collect`), "reduce to a single value" (`fold`/`sum`/`product`), "does any element satisfy this?" (`any`), "find the first match" (`find`), "how many elements?" (`count`). Knowing which consumer to reach for — and understanding that each one processes the *entire* iterator exactly once — is essential for writing efficient Rust.

The distinction between adapters (lazy, return iterators) and consumers (eager, return values) is one of the most important mental models in Rust's iterator system.

## The Intuition

In Python, consuming an iterator is usually `list(it)`, `sum(it)`, or a for-loop. In OCaml, `List.fold_left` is the universal combinator. In Haskell, `foldl` and `foldr`. In Rust, `fold` is the universal consumer too — but common patterns like sum, product, count, and collect are implemented as specialized methods for clarity and performance.

Think of `fold` as the foundation: every other consumer could be implemented in terms of `fold`. Rust provides the specialized ones because they express intent more clearly and sometimes optimize better.

## How It Works in Rust

```rust
// collect: materialize the iterator into a collection
let vec: Vec<i32>          = iter.collect();
let set: HashSet<i32>      = iter.collect();
let string: String         = chars.collect();
// The target type is inferred from the annotation — same method, different behavior

// Collecting Results: stops at first error, or collects all errors
let parsed: Result<Vec<i32>, _> = vec!["1", "2", "abc"]
    .iter()
    .map(|s| s.parse::<i32>())
    .collect();  // Err on "abc"
```

```rust
// fold: universal reduction — (accumulator, element) -> accumulator
let sum = data.iter().fold(0, |acc, x| acc + x);
let product = data.iter().fold(1, |acc, x| acc * x);

// fold for complex accumulations: build a HashMap in one pass
let frequencies = data.iter().fold(HashMap::new(), |mut map, &x| {
    *map.entry(x).or_insert(0) += 1;
    map
});
```

```rust
// Specific consumers for common patterns — prefer these over fold when they fit
let sum: i32 = data.iter().sum();
let product: i32 = data.iter().product();
let max: Option<&i32> = data.iter().max();
let count: usize = data.iter().filter(|x| **x > 0).count();
```

```rust
// Short-circuit consumers — stop early when possible
let has_negative = data.iter().any(|&x| x < 0);     // stops at first negative
let all_positive = data.iter().all(|&x| x > 0);      // stops at first non-positive
let first_big = data.iter().find(|&&x| x > 100);     // stops at first match
let pos = data.iter().position(|&x| x > 100);        // index of first match
```

```rust
// group_by pattern with fold
fn group_by<T, K: Hash + Eq>(data: &[T], key: impl Fn(&T) -> K) -> HashMap<K, Vec<&T>> {
    data.iter().fold(HashMap::new(), |mut acc, item| {
        acc.entry(key(item)).or_default().push(item);
        acc
    })
}
```

## What This Unlocks

- **One-pass aggregation**: compute frequencies, group-by, running totals in a single iteration — no need to loop twice.
- **Ergonomic collection building**: `collect()` into `Vec`, `HashSet`, `HashMap` — the type annotation drives the behavior.
- **Short-circuit logic**: `any`/`all`/`find` stop as soon as possible — efficient for large or infinite iterators.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Universal reduction | `List.fold_left f init lst` | `iter.fold(init, f)` |
| Collect to list | `List.of_seq` or `Seq.fold` | `.collect::<Vec<_>>()` |
| Short-circuit exists | `List.exists` | `.any(predicate)` |
| Short-circuit forall | `List.for_all` | `.all(predicate)` |
| Find first | `List.find_opt` | `.find(predicate)` |
| Sum / product | Manual fold or `List.fold_left (+) 0` | `.sum()` / `.product()` |
