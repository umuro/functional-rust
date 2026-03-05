📖 **[View on hightechmind.io →](https://hightechmind.io/rust/290-iterator-unzip-partition)**

---

# 290: Advanced Splitting Patterns

**Difficulty:** 3  **Level:** Advanced

Split iterators into multiple collections in a single pass — unzip, partition, and multi-way categorization without multiple traversals.

## The Problem This Solves

Data processing often requires splitting a collection based on a predicate or structure. The naive approach iterates multiple times — once to find the positives, once for the negatives, once per category. For large datasets or non-replayable iterators (network streams, lazy generators), this is wrong or impossible.

Rust's `partition` and `unzip` consume an iterator in a single pass and produce two collections simultaneously. `partition(|x| predicate)` splits by boolean — true elements go left, false go right, exactly like `List.partition` in OCaml. `unzip()` on an iterator of pairs separates the pairs into two collections — the iterator analogue of transposing a list of tuples.

Combining these with `fold` gives you n-way splits, categorization by enum variant, and simultaneous accumulation into different container types — all in one traversal. This is the functional pattern for what imperative code does with multiple loops and mutable accumulators.

## The Intuition

`partition` and `unzip` are single-pass transformations that split one iterator into two collections — fold lets you extend this to any number of buckets.

## How It Works in Rust

```rust
// partition — split by predicate
let nums = vec![-3i32, -1, 0, 1, 2, 5];
let (negatives, non_neg): (Vec<i32>, Vec<i32>) =
    nums.into_iter().partition(|&x| x < 0);
// negatives: [-3, -1], non_neg: [0, 1, 2, 5]

// unzip — split pairs into two collections
let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c')];
let (numbers, letters): (Vec<i32>, Vec<char>) = pairs.into_iter().unzip();
// numbers: [1, 2, 3], letters: ['a', 'b', 'c']

// Nested unzip — unzip pairs-of-pairs
let nested: Vec<((i32, i32), &str)> = vec![((1, 2), "a"), ((3, 4), "b")];
let (inner_pairs, labels): (Vec<(i32,i32)>, Vec<&str>) = nested.into_iter().unzip();
let (lefts, rights): (Vec<i32>, Vec<i32>) = inner_pairs.into_iter().unzip();

// partition_map pattern: split by parse success
let data = vec!["1", "two", "3", "four"];
let (nums, words): (Vec<i32>, Vec<&&str>) = data.iter().fold(
    (Vec::new(), Vec::new()),
    |(mut ns, mut ws), s| {
        match s.parse::<i32>() {
            Ok(n) => ns.push(n),
            Err(_) => ws.push(s),
        }
        (ns, ws)
    }
);

// Three-way split using fold
let values = [1u32, 15, 100, 8, 50, 3, 200];
let (small, medium, large) = values.iter().fold(
    (vec![], vec![], vec![]),
    |(mut s, mut m, mut l), &v| {
        match v {
            0..=10   => s.push(v),
            11..=99  => m.push(v),
            _        => l.push(v),
        }
        (s, m, l)
    }
);
```

1. `partition(|x| bool)` → `(Vec<T>, Vec<T>)` — single pass, two buckets.
2. `unzip()` on `Iterator<Item=(A,B)>` → `(Vec<A>, Vec<B>)` — separate the pairs.
3. `fold((vec![], vec![], ...), |acc, item| { ... acc })` — n-way split in one pass.
4. Combine with `flat_map`, `filter_map`, or `scan` for transform-then-split pipelines.

## What This Unlocks

- **Single-pass efficiency**: Partition large datasets or streams without multiple traversals.
- **Non-replayable iterators**: Split a file reader or network stream — you only read it once.
- **Clean functional style**: Replace multiple mutable accumulator loops with one `fold` returning a tuple.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Two-way split | `List.partition pred lst` | `iter.partition(\|x\| pred(x))` |
| Pair separation | `List.split [(a,b);...]` | `iter.unzip()` |
| N-way split | `List.fold_left` with tuple accumulator | `fold((vec![], ...), \|acc, x\| ...)` |
| Result split | `List.partition_map` (>=4.12) | `fold` with `match Ok/Err` pattern |
| Single-pass guarantee | Depends on strictness | Guaranteed — `Iterator` consumed once |
