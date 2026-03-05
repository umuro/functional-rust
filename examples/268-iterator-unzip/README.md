# 268: Splitting Pairs with unzip()

**Difficulty:** 2  **Level:** Intermediate

Split an iterator of `(A, B)` tuples into two separate collections in a single pass.

## The Problem This Solves

You have a sequence of paired data — students and their scores, keys and values, x/y coordinates — and you need them in two separate collections. Without `unzip()`, you'd iterate twice (once to collect the firsts, once to collect the seconds) or write a manual loop with two `push` calls. Both approaches are more code and less expressive than the intent.

`unzip()` is the inverse of `zip()`: where `zip` pairs two iterators into one, `unzip` splits one iterator of pairs into two. It's a single allocation pass — both collections are filled simultaneously.

OCaml has `List.split` for exactly this operation. In Rust, `unzip()` works on any iterator of 2-tuples and collects into any collection type that implements `Default + Extend`.

## The Intuition

`unzip()` takes `Iterator<Item=(A, B)>` and produces `(CollectionA<A>, CollectionB<B>)` — splitting at the seam of every tuple, in one pass.

```rust
let pairs = vec![(1, "one"), (2, "two"), (3, "three")];
let (nums, words): (Vec<i32>, Vec<&str>) = pairs.into_iter().unzip();
// nums  → [1, 2, 3]
// words → ["one", "two", "three"]
```

## How It Works in Rust

```rust
// Basic split
let pairs = vec![(1i32, "one"), (2, "two"), (3, "three")];
let (nums, words): (Vec<i32>, Vec<&str>) = pairs.into_iter().unzip();

// zip → unzip roundtrip (inverse operations)
let a = vec![1i32, 2, 3];
let b = vec![4i32, 5, 6];
let (a2, b2): (Vec<i32>, Vec<i32>) = a.iter().copied()
    .zip(b.iter().copied())
    .unzip();
// a2 == a, b2 == b

// Practical: separate names from scores, compute average
let students = [("Alice", 95u32), ("Bob", 87), ("Carol", 92)];
let (names, scores): (Vec<&str>, Vec<u32>) = students.iter().copied().unzip();
let avg = scores.iter().sum::<u32>() / scores.len() as u32;
// names → ["Alice", "Bob", "Carol"], avg → 91

// Generate two related sequences in one pass
let (squares, cubes): (Vec<u32>, Vec<u32>) = (1u32..=5)
    .map(|n| (n * n, n * n * n))  // produce pairs
    .unzip();
// squares → [1, 4, 9, 16, 25]
// cubes   → [1, 8, 27, 64, 125]
```

The type annotation `(Vec<A>, Vec<B>)` on the left side is required — `unzip()` can't infer which collection type you want without it.

## What This Unlocks

- **Separating structured records** — split `(key, value)` pairs into parallel key and value vecs for separate processing.
- **Roundtrip with `zip`** — build combined processing pipelines that split at the end for separate outputs.
- **Computing parallel statistics** — separate a `Vec<(x, y)>` into xs and ys to compute stats on each dimension independently.

## Key Differences

| Concept | OCaml | Rust |
|---------|-------|------|
| Split list of pairs | `List.split` | `iter.unzip()` |
| Inverse of pairing | `List.combine` ↔ `List.split` | `zip()` ↔ `unzip()` |
| Works on any iterator | No — list-specific | Yes — any `Iterator<Item=(A,B)>` |
| Collection type | Always `list` | Generic — `Vec`, `HashMap`, etc. |
